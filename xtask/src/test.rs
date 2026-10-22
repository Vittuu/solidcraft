//! `cargo xtask test`: build once, then share a bounded CPU budget between test binaries.
//! Timing and process-resource regressions, and the action matrix's workers, run without
//! competing tests or side jobs. The default engine suite shares the ordinary CPU budget.

use std::collections::VecDeque;
use std::process::{Command, Stdio};
use std::sync::{Mutex, mpsc};
use std::time::Instant;

use crate::{cargo, root};

/// A test executable from `cargo test --no-run`.
struct TestBin {
    label: String,
    exe: String,
    dir: String,
    name: String,
}

// Each case measures elapsed time, timing ratios, or process-wide CPU/peak memory. Run each
// in its own process after the shared work. Keep exact full names: sibling tests must stay in
// the ordinary suite, and libtest's normal ignore policy must apply to these cases too.
const ISOLATED_ENGINE_TESTS: &[&str] = &[
    "boolean_regression_tests::press_pull_on_the_enclosure_lid_is_prompt",
    "cmd::features::patterns_tests::one_extrude_of_many_sketch_circles_scales_linearly",
    "cmd::features::patterns_tests::pattern_of_512_hole_cuts_finishes_in_reasonable_time",
    "thread_tests::fillet_beside_a_modelled_thread_finishes",
];

impl TestBin {
    fn exclusive(&self, test_args: &[&str]) -> bool {
        self.name == "action_matrix" || (self.name == "solidcraft_engine" && !test_args.is_empty())
    }

    fn command(&self, test_args: &[&str], threads: usize, isolated: bool) -> Command {
        let mut c = Command::new(&self.exe);
        c.args(test_args).current_dir(&self.dir).env("CARGO_MANIFEST_DIR", &self.dir);
        if self.name == "solidcraft_engine" && test_args.is_empty() && !isolated {
            // --exact also applies to --skip. No positive filter means all other tests run.
            c.arg("--exact");
            for name in ISOLATED_ENGINE_TESTS {
                c.args(["--skip", name]);
            }
        }
        if isolated && std::env::var_os("RUST_TEST_THREADS").is_none() {
            c.env("RUST_TEST_THREADS", "1");
        }
        let threads = if self.name == "action_matrix" {
            // Its design workers are child processes. Do not give each one the full
            // CPU pool through the top-level environment, unless the user requests it.
            let jobs = std::env::var("ACTION_MATRIX_JOBS")
                .ok()
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or_else(|| worker_limit(threads))
                .max(1);
            if std::env::var_os("ACTION_MATRIX_JOBS").is_none() {
                c.env("ACTION_MATRIX_JOBS", jobs.to_string());
            }
            (threads / jobs).max(1)
        } else {
            threads
        };
        budget(&mut c, threads);
        c
    }
}

enum TestRun<'a> {
    Shared(&'a TestBin),
    Isolated(&'a TestBin),
    Exact(&'a TestBin, &'static str),
}

impl TestRun<'_> {
    fn label(&self) -> String {
        match self {
            Self::Shared(b) | Self::Isolated(b) => b.label.clone(),
            Self::Exact(b, name) => format!("{}: {name}", b.label),
        }
    }

    fn command(&self, test_args: &[&str], threads: usize) -> Command {
        match self {
            Self::Shared(b) => b.command(test_args, threads, false),
            Self::Isolated(b) => b.command(test_args, threads, true),
            Self::Exact(b, name) => b.command(&["--exact", name], threads, true),
        }
    }
}

fn schedule<'a>(bins: &'a [TestBin], test_args: &[&str]) -> (Vec<TestRun<'a>>, Vec<TestRun<'a>>) {
    let mut shared = Vec::new();
    let mut isolated = Vec::new();
    for b in bins {
        if b.exclusive(test_args) {
            isolated.push(TestRun::Isolated(b));
        } else {
            shared.push(TestRun::Shared(b));
            if b.name == "solidcraft_engine" {
                isolated.extend(ISOLATED_ENGINE_TESTS.iter().map(|name| TestRun::Exact(b, name)));
            }
        }
    }
    (shared, isolated)
}

enum Job<'a> {
    Tests(TestRun<'a>),
    Side(Vec<(String, Command)>),
}

fn worker_limit(cpus: usize) -> usize {
    cpus.clamp(1, 4)
}

/// Apply defaults to this child only. Keep parent preferences, explicit command environment
/// overrides/removals and CLI flags; Cargo and libtest give their flags precedence over env.
fn budget(c: &mut Command, threads: usize) {
    for key in ["CARGO_BUILD_JOBS", "RUST_TEST_THREADS", "RAYON_NUM_THREADS"] {
        if std::env::var_os(key).is_none() && !c.get_envs().any(|(name, _)| env_key_eq(name, key)) {
            c.env(key, threads.to_string());
        }
    }
}

fn env_key_eq(name: &std::ffi::OsStr, key: &str) -> bool {
    #[cfg(windows)]
    {
        name.to_str().is_some_and(|name| name.eq_ignore_ascii_case(key))
    }
    #[cfg(not(windows))]
    {
        name == key
    }
}

fn work<T>(queue: &Mutex<VecDeque<T>>, mut run: impl FnMut(T)) {
    loop {
        let job = queue.lock().unwrap_or_else(|e| e.into_inner()).pop_front();
        let Some(job) = job else { break };
        run(job);
    }
}

fn report(label: String, result: Result<String, String>, secs: f64, failed: &mut Vec<String>) {
    match result {
        Ok(summary) => humanln!("ok    {label:<44} {secs:6.1}s  {summary}"),
        Err(output) => {
            humanln!("FAIL  {label:<44} {secs:6.1}s\n{output}");
            failed.push(label);
        }
    }
}

/// Build the tests with `cargo_args` (e.g. `--locked`), run each binary with `test_args`, and run
/// `side` jobs sequentially alongside the shared suites. Every suite runs even after a failure;
/// timing/resource-sensitive cases follow only after all shared and side work has ended.
pub fn run(cargo_args: &[&str], test_args: &[&str], side: Vec<(String, Command)>) -> Result<(), String> {
    let bins = build(cargo_args)?;
    let cpus = std::thread::available_parallelism().map_or(1, |n| n.get());
    let workers = worker_limit(cpus);
    let threads = (cpus / workers).max(1);
    let (shared, isolated) = schedule(&bins, test_args);
    let invocations = shared.len() + isolated.len();
    let mut jobs: VecDeque<Job<'_>> = shared.into_iter().map(Job::Tests).collect();
    if !side.is_empty() {
        jobs.push_back(Job::Side(side));
    }
    let queue = Mutex::new(jobs);
    let start = Instant::now();
    let (tx, rx) = mpsc::channel::<(String, Result<String, String>, f64)>();
    let mut failed = Vec::new();
    std::thread::scope(|scope| {
        for _ in 0..workers {
            let tx = tx.clone();
            let queue = &queue;
            scope.spawn(move || {
                work(queue, |job| match job {
                    Job::Tests(b) => {
                        let t0 = Instant::now();
                        let _ = tx.send((b.label(), finish(b.command(test_args, threads)), t0.elapsed().as_secs_f64()));
                    }
                    Job::Side(side) => {
                        for (label, mut c) in side {
                            let t0 = Instant::now();
                            budget(&mut c, threads);
                            let _ = tx.send((label, finish(c), t0.elapsed().as_secs_f64()));
                        }
                    }
                });
            });
        }
        drop(tx);
        for (label, result, secs) in rx {
            report(label, result, secs, &mut failed);
        }
    });
    for b in isolated {
        let t0 = Instant::now();
        report(b.label(), finish(b.command(test_args, cpus)), t0.elapsed().as_secs_f64(), &mut failed);
    }
    humanln!("test: {} binaries ({} binary invocations) in {:.1}s", bins.len(), invocations, start.elapsed().as_secs_f64());
    if failed.is_empty() { Ok(()) } else { Err(format!("failed: {}", failed.join(", "))) }
}

/// Run to the end: the last `test result:` line on success, all output on failure.
fn finish(mut c: Command) -> Result<String, String> {
    let out = c.stdin(Stdio::null()).output().map_err(|e| format!("failed to spawn: {e}"))?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    if out.status.success() {
        Ok(stdout.lines().rev().find(|l| l.starts_with("test result:")).unwrap_or("").trim_start_matches("test result: ").to_string())
    } else {
        Err(format!("{stdout}{}\n({})", String::from_utf8_lossy(&out.stderr), out.status))
    }
}

/// `cargo test --workspace --profile ci --no-run`, and the executables it built.
fn build(cargo_args: &[&str]) -> Result<Vec<TestBin>, String> {
    let mut c = cargo();
    c.args(["test", "--workspace", "--profile", "ci", "--no-run", "--message-format=json-render-diagnostics"]).args(cargo_args);
    eprintln!("$ cargo test --workspace --profile ci --no-run");
    let out = c.stderr(Stdio::inherit()).output().map_err(|e| format!("cargo test --no-run: failed to spawn: {e}"))?;
    if !out.status.success() {
        return Err(format!("cargo test --no-run: exited with {}", out.status));
    }
    let mut bins: Vec<TestBin> = Vec::new();
    for line in String::from_utf8_lossy(&out.stdout).lines() {
        let Ok(m) = serde_json::from_str::<serde_json::Value>(line) else { continue };
        let Some(exe) = m["executable"].as_str().filter(|_| m["reason"] == "compiler-artifact" && m["profile"]["test"] == true) else {
            continue;
        };
        if bins.iter().any(|b| b.exe == exe) {
            continue;
        }
        let manifest = std::path::Path::new(m["manifest_path"].as_str().unwrap_or_default()).to_path_buf();
        let dir = manifest.parent().map(|d| d.to_path_buf()).unwrap_or_else(root);
        let pkg = dir.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        let kind = m["target"]["kind"][0].as_str().unwrap_or("");
        let label = format!("{pkg} {kind} {}", m["target"]["name"].as_str().unwrap_or(""));
        bins.push(TestBin {
            label,
            exe: exe.to_string(),
            dir: dir.to_string_lossy().into_owned(),
            name: m["target"]["name"].as_str().unwrap_or("").into(),
        });
    }
    if bins.is_empty() {
        return Err("cargo test --no-run built no test executables".into());
    }
    Ok(bins)
}

/// The doc tests (`cargo test --doc`), to run as a side job.
pub fn doc_tests(cargo_args: &[&str], test_args: &[&str]) -> (String, Command) {
    let mut c = cargo();
    c.args(["test", "--workspace", "--profile", "ci", "--doc"]).args(cargo_args).arg("--").args(test_args);
    ("doc tests".into(), c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn bin(name: &str) -> TestBin {
        TestBin { name: name.into(), label: name.into(), exe: "test-child".into(), dir: ".".into() }
    }

    fn args(command: &Command) -> Vec<String> {
        command.get_args().map(|value| value.to_str().unwrap().to_string()).collect()
    }

    #[test]
    fn default_engine_schedule_partitions_exact_cases_without_losing_siblings() {
        let bins = [bin("solidcraft_engine"), bin("solidcraft_kernel"), bin("action_matrix")];
        let (shared, isolated) = schedule(&bins, &[]);
        assert_eq!(shared.iter().map(TestRun::label).collect::<Vec<_>>(), ["solidcraft_engine", "solidcraft_kernel"]);
        assert_eq!(isolated.len(), ISOLATED_ENGINE_TESTS.len() + 1);
        assert_eq!(shared.len() + isolated.len(), bins.len() + ISOLATED_ENGINE_TESTS.len());
        assert!(matches!(isolated.last(), Some(TestRun::Isolated(b)) if b.name == "action_matrix"));

        let ordinary = args(&shared[0].command(&[], 2));
        assert_eq!(ordinary[0], "--exact");
        let (skip_pairs, remainder) = ordinary[1..].as_chunks::<2>();
        assert!(remainder.is_empty());
        let skips: Vec<_> = skip_pairs
            .iter()
            .map(|pair| {
                assert_eq!(pair[0], "--skip");
                pair[1].as_str()
            })
            .collect();
        assert_eq!(skips, ISOLATED_ENGINE_TESTS);
        assert_eq!(ordinary.len(), 1 + 2 * ISOLATED_ENGINE_TESTS.len());
        assert!(args(&shared[1].command(&[], 2)).is_empty());
        assert!(args(&isolated.last().unwrap().command(&[], 8)).is_empty());

        let exact: Vec<_> = isolated[..ISOLATED_ENGINE_TESTS.len()]
            .iter()
            .map(|run| {
                let command = run.command(&[], 8);
                let arguments = args(&command);
                assert_eq!(arguments.len(), 2);
                assert_eq!(arguments[0], "--exact");
                assert!(run.label().ends_with(&arguments[1]));
                arguments[1].clone()
            })
            .collect();
        assert_eq!(exact, ISOLATED_ENGINE_TESTS);

        // libtest's exact skip means these same-prefix sibling names still run in the ordinary
        // invocation. Each real isolated name has one deferred invocation and no ordinary one.
        for name in ["another_test".to_string()]
            .into_iter()
            .chain(ISOLATED_ENGINE_TESTS.iter().map(|name| format!("{name}_sibling")))
            .chain(ISOLATED_ENGINE_TESTS.iter().map(|name| name.to_string()))
        {
            let ordinary_runs = usize::from(!skips.contains(&name.as_str()));
            let exact_runs = exact.iter().filter(|selected| **selected == name).count();
            assert_eq!(ordinary_runs + exact_runs, 1, "{name}");
        }
    }

    #[test]
    fn any_explicit_test_arguments_keep_the_whole_engine_isolated_and_literal() {
        let bins = [bin("solidcraft_engine"), bin("solidcraft_kernel"), bin("action_matrix")];
        for requested in [
            vec!["shell"],
            vec!["--ignored"],
            vec!["--include-ignored", "--test-threads=7", "--skip", "thread_tests"],
            vec!["--list"],
            vec!["--exact", ISOLATED_ENGINE_TESTS[0]],
        ] {
            let (shared, isolated) = schedule(&bins, &requested);
            assert_eq!(shared.len(), 1);
            assert!(matches!(&shared[0], TestRun::Shared(b) if b.name == "solidcraft_kernel"));
            assert_eq!(isolated.len(), 2);
            assert!(isolated.iter().all(|run| matches!(run, TestRun::Isolated(_))));
            for run in shared.iter().chain(&isolated) {
                assert_eq!(args(&run.command(&requested, 8)), requested);
            }
        }
    }

    #[test]
    fn isolation_changes_only_the_default_libtest_thread_count() {
        let bins = [bin("solidcraft_engine")];
        let (shared, isolated) = schedule(&bins, &[]);
        let common = shared[0].command(&[], 2);
        let single = isolated[0].command(&[], 8);
        let value = |command: &Command, key| {
            command.get_envs().find(|(name, _)| env_key_eq(name, key)).and_then(|(_, value)| value.map(|value| value.to_str().unwrap().to_string()))
        };
        if std::env::var_os("RUST_TEST_THREADS").is_none() {
            assert_eq!(value(&common, "RUST_TEST_THREADS").as_deref(), Some("2"));
            assert_eq!(value(&single, "RUST_TEST_THREADS").as_deref(), Some("1"));
        }
        if std::env::var_os("RAYON_NUM_THREADS").is_none() {
            assert_eq!(value(&common, "RAYON_NUM_THREADS").as_deref(), Some("2"));
            assert_eq!(value(&single, "RAYON_NUM_THREADS").as_deref(), Some("8"));
        }
    }

    #[test]
    fn bounded_workers_execute_every_job_once() {
        let count = 257;
        let queue = Mutex::new((0..count).collect::<VecDeque<_>>());
        let visits: Vec<_> = (0..count).map(|_| AtomicUsize::new(0)).collect();
        let active = AtomicUsize::new(0);
        let peak = AtomicUsize::new(0);
        let workers = worker_limit(12);
        std::thread::scope(|scope| {
            for _ in 0..workers {
                scope.spawn(|| {
                    work(&queue, |job| {
                        let running = active.fetch_add(1, Ordering::SeqCst) + 1;
                        peak.fetch_max(running, Ordering::SeqCst);
                        visits[job].fetch_add(1, Ordering::SeqCst);
                        active.fetch_sub(1, Ordering::SeqCst);
                    })
                });
            }
        });
        assert!(visits.iter().all(|n| n.load(Ordering::SeqCst) == 1));
        assert_eq!(active.load(Ordering::SeqCst), 0);
        assert!(peak.load(Ordering::SeqCst) <= workers);
        assert_eq!(worker_limit(1), 1);
    }

    #[test]
    fn child_cpu_defaults_preserve_explicit_environment_and_arguments() {
        let mut c = Command::new("test-child");
        c.env("RUST_TEST_THREADS", "7").env_remove("RAYON_NUM_THREADS").arg("--test-threads=9");
        budget(&mut c, 2);
        assert_eq!(c.get_envs().find(|(key, _)| *key == "RUST_TEST_THREADS").and_then(|(_, value)| value), Some(std::ffi::OsStr::new("7")));
        assert!(matches!(c.get_envs().find(|(key, _)| *key == "RAYON_NUM_THREADS"), Some((_, None))));
        assert_eq!(c.get_args().collect::<Vec<_>>(), vec![std::ffi::OsStr::new("--test-threads=9")]);
        if std::env::var_os("CARGO_BUILD_JOBS").is_none() {
            assert_eq!(c.get_envs().find(|(key, _)| *key == "CARGO_BUILD_JOBS").and_then(|(_, value)| value), Some(std::ffi::OsStr::new("2")));
        }
    }

    #[cfg(windows)]
    #[test]
    fn child_cpu_defaults_keep_windows_case_insensitive_environment_overrides() {
        let mut c = Command::new("test-child");
        c.env("rust_test_threads", "7").env_remove("RaYoN_NuM_ThReAdS");
        budget(&mut c, 2);
        assert_eq!(c.get_envs().find(|(key, _)| env_key_eq(key, "RUST_TEST_THREADS")).and_then(|(_, value)| value), Some(std::ffi::OsStr::new("7")));
        assert!(matches!(c.get_envs().find(|(key, _)| env_key_eq(key, "RAYON_NUM_THREADS")), Some((_, None))));
    }
}
