//! `cargo xtask web-offline [DIR]`: let a web build work offline and update itself.
//!
//! Runs as Trunk's post_build hook (apps/solidcraft-web/Trunk.toml) on the staging directory
//! (`TRUNK_STAGING_DIR`), or on DIR. It writes `sw.js` from apps/solidcraft-web/sw.js with this
//! build's version and files (each with its SHA-256, which the worker checks every download
//! against, so a deployment that changes mid-download never mixes into a kept build), and the
//! same as `build.json` (which launches check to find a newly deployed build). The version is the
//! workspace version plus a hash of all that, so any change to the build changes `sw.js`, which
//! is what makes browsers install the update.
//! Debug builds (`trunk serve`) get the unbuilt template instead, a worker that keeps nothing and
//! retires one a release build left on the same origin.

use std::path::{Path, PathBuf};

use serde_json::{Value, json};
use sha2::{Digest, Sha256};

/// The template line the build is written into.
const MARKER: &str = "/* @build */";
const SW: &str = "sw.js";
const BUILD: &str = "build.json";
/// Files the app fetches only when it needs them (the CJK font, ~4.7 MB: `JAPANESE_FONT_PATH`
/// in apps/solidcraft-web/src/main.rs): not kept for offline use, so not every user downloads
/// them. The worker leaves them to the network.
const ON_DEMAND: &[&str] = &["fonts/"];
/// The version that makes sw.js retire (the template's own).
const RETIRE_VERSION: &str = "dev";

pub fn run(root: &Path, dir: Option<&str>) -> Result<(), String> {
    let dir = match dir {
        Some(d) => PathBuf::from(d),
        None => {
            let dir = PathBuf::from(
                std::env::var("TRUNK_STAGING_DIR").map_err(|_| "web-offline: give the built site's directory (or run as Trunk's hook)")?,
            );
            #[cfg(windows)]
            {
                let dist = PathBuf::from(std::env::var_os("TRUNK_DIST_DIR").ok_or("web-offline: Trunk's hook requires TRUNK_DIST_DIR")?);
                prepare_trunk_staging(&dir, &dist)?;
            }
            if std::env::var("TRUNK_PROFILE").is_ok_and(|p| p != "release") {
                retire(root, &dir)?;
                humanln!("web-offline: debug build, {SW} retires a kept release build");
                return Ok(());
            }
            dir
        }
    };
    let (version, n) = generate(root, &dir)?;
    humanln!("web-offline: {SW} keeps {n} files (version {version})");
    Ok(())
}

/// Trunk removes old distribution subdirectories and its empty staging directory.
/// Windows may refuse either removal while FILE_ATTRIBUTE_READONLY is set. Only
/// real descendant directories of the guarded Trunk distribution lose that bit;
/// its root, every file, directory links, and explicit DIR invocations are untouched.
#[cfg(windows)]
fn prepare_trunk_staging(dir: &Path, dist: &Path) -> Result<(), String> {
    use std::os::windows::fs::MetadataExt;

    const REPARSE_POINT: u32 = 0x400;
    let metadata = std::fs::symlink_metadata(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    if dir.file_name() != Some(std::ffi::OsStr::new(".stage")) || !metadata.is_dir() || metadata.file_attributes() & REPARSE_POINT != 0 {
        return Err(format!("web-offline: {} is not Trunk's real .stage directory", dir.display()));
    }
    let canonical = dir.canonicalize().map_err(|e| format!("{}: {e}", dir.display()))?;
    let dist = dist.canonicalize().map_err(|e| format!("{}: {e}", dist.display()))?;
    if canonical.parent() != Some(dist.as_path()) {
        return Err(format!("web-offline: {} is not directly inside {}", canonical.display(), dist.display()));
    }

    fn collect_directories(dir: &Path, out: &mut Vec<(PathBuf, std::fs::Permissions)>) -> Result<(), String> {
        use std::os::windows::fs::MetadataExt;

        for entry in std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
            let path = entry.map_err(|e| format!("{}: {e}", dir.display()))?.path();
            let metadata = std::fs::symlink_metadata(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            // Never follow junctions or directory symlinks into another tree.
            if !metadata.is_dir() || metadata.file_attributes() & REPARSE_POINT != 0 {
                continue;
            }
            out.push((path.clone(), metadata.permissions()));
            collect_directories(&path, out)?;
        }
        Ok(())
    }

    // Finish validation and enumeration before changing any directory attributes.
    let mut directories = Vec::new();
    collect_directories(&dist, &mut directories)?;
    for (path, mut permissions) in directories {
        if permissions.readonly() {
            // Windows only: retain every other attribute and all file permissions.
            #[allow(clippy::permissions_set_readonly_false)]
            permissions.set_readonly(false);
            std::fs::set_permissions(&path, permissions).map_err(|e| format!("{}: {e}", path.display()))?;
        }
    }
    Ok(())
}

/// Write `dir/sw.js` and `dir/build.json` for the site in `dir`; returns its version and how many
/// files it keeps.
pub fn generate(root: &Path, dir: &Path) -> Result<(String, usize), String> {
    let files = list(dir)?;
    if !files.iter().any(|(name, _)| name == "index.html") {
        return Err(format!("web-offline: {} has no index.html", dir.display()));
    }
    let manifest: Vec<Value> = files.iter().map(|(name, bytes)| json!({ "file": name, "sha256": sha256(bytes) })).collect();
    let manifest = Value::Array(manifest);
    let digest = sha256(manifest.to_string().as_bytes());
    let version = format!("{}-{}", env!("CARGO_PKG_VERSION"), digest.get(..16).unwrap_or(&digest));
    let template = template(root)?;
    let sw = render(&template, &version, &manifest)?;
    std::fs::write(dir.join(SW), sw).map_err(|e| format!("{}: {e}", dir.join(SW).display()))?;
    let build = json!({ "version": version, "files": manifest }).to_string();
    std::fs::write(dir.join(BUILD), build).map_err(|e| format!("{}: {e}", dir.join(BUILD).display()))?;
    Ok((version, files.len()))
}

/// Write `dir/sw.js` as the worker that keeps nothing and retires a kept release build.
pub fn retire(root: &Path, dir: &Path) -> Result<(), String> {
    let template = template(root)?;
    let sw = render(&template, RETIRE_VERSION, &json!([]))?;
    std::fs::write(dir.join(SW), sw).map_err(|e| format!("{}: {e}", dir.join(SW).display()))
}

fn template(root: &Path) -> Result<String, String> {
    let path = root.join("apps/solidcraft-web").join(SW);
    std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))
}

/// Every file under `dir` (but `sw.js`, `build.json` and ON_DEMAND ones) as (URL path relative to `dir`, contents), sorted.
fn list(dir: &Path) -> Result<Vec<(String, Vec<u8>)>, String> {
    fn walk(dir: &Path, prefix: &str, out: &mut Vec<(String, Vec<u8>)>) -> Result<(), String> {
        let rd = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        for e in rd {
            let e = e.map_err(|e| format!("{}: {e}", dir.display()))?;
            let name = format!("{prefix}{}", e.file_name().to_string_lossy());
            let p = e.path();
            if p.is_dir() {
                walk(&p, &format!("{name}/"), out)?;
            } else if name != SW && name != BUILD && !ON_DEMAND.iter().any(|p| name.starts_with(p)) {
                out.push((name, std::fs::read(&p).map_err(|e| format!("{}: {e}", p.display()))?));
            }
        }
        Ok(())
    }
    let mut out = Vec::new();
    walk(dir, "", &mut out)?;
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
}

/// Lower-case hex SHA-256, as the worker's `crypto.subtle.digest` check computes it.
fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
}

/// The template with its marker line replaced by the version and files.
fn render(template: &str, version: &str, files: &Value) -> Result<String, String> {
    let build = format!("{MARKER} const VERSION = {}, FILES = {files};", Value::from(version));
    let mut found = false;
    let lines: Vec<&str> = template
        .lines()
        .map(|l| {
            if l.starts_with(MARKER) && !found {
                found = true;
                build.as_str()
            } else {
                l
            }
        })
        .collect();
    if !found {
        return Err(format!("web-offline: the {SW} template has no `{MARKER}` line"));
    }
    Ok(lines.join("\n") + "\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_replaces_the_marker_line_only() {
        let t = "// head\n/* @build */ const VERSION = \"dev\", FILES = [];\nconst X = 1;\n";
        let files = json!([{ "file": "a \"b\".js", "sha256": "00" }]);
        let out = render(t, "0.3.2-00ff", &files).unwrap_or_default();
        assert_eq!(
            out,
            "// head\n/* @build */ const VERSION = \"0.3.2-00ff\", FILES = [{\"file\":\"a \\\"b\\\".js\",\"sha256\":\"00\"}];\nconst X = 1;\n"
        );
        assert!(render("no marker\n", "v", &json!([])).is_err());
    }

    #[test]
    fn the_template_has_the_marker() {
        let t = std::fs::read_to_string(crate::root().join("apps/solidcraft-web").join(SW)).unwrap_or_default();
        assert!(render(&t, "v", &json!(["x"])).is_ok_and(|s| s.contains("FILES = [\"x\"]")));
    }

    #[test]
    fn sha256_is_lower_case_hex() {
        // FIPS 180-2's "abc" example.
        assert_eq!(sha256(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    }

    #[test]
    fn generate_lists_every_file_with_its_digest() {
        let dir = std::env::temp_dir().join(format!("solidcraft-web-offline-unit-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let ok = std::fs::create_dir_all(dir.join("snippets"))
            .and_then(|()| std::fs::create_dir_all(dir.join("fonts")))
            .and_then(|()| std::fs::write(dir.join("fonts/big.ttf"), "on demand"))
            .and_then(|()| std::fs::write(dir.join("index.html"), "<html>"))
            .and_then(|()| std::fs::write(dir.join("snippets/a.js"), "abc"))
            .and_then(|()| std::fs::write(dir.join(SW), "old"));
        assert!(ok.is_ok());
        let (version, n) = generate(&crate::root(), &dir).unwrap_or_default();
        let build: Value = std::fs::read(dir.join(BUILD)).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default();
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(n, 2, "on-demand files aren't listed");
        assert_eq!(build["version"], version.as_str());
        assert_eq!(build["files"][1], json!({ "file": "snippets/a.js", "sha256": sha256(b"abc") }));
        assert_eq!(build["files"][0]["file"], "index.html");
    }

    #[cfg(windows)]
    fn staging_fixture(label: &str) -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let root = crate::root().canonicalize()?;
        let dist = root.join("target").join(format!("web-offline-{label}-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        std::fs::create_dir(&dist)?;
        let dist = dist.canonicalize()?;
        assert!(dist.starts_with(root.join("target")));
        let stage = dist.join(".stage");
        std::fs::create_dir(&stage)?;
        Ok((dist, stage))
    }

    #[cfg(windows)]
    fn set_readonly(path: &Path, readonly: bool) -> Result<(), std::io::Error> {
        let mut permissions = std::fs::metadata(path)?.permissions();
        permissions.set_readonly(readonly);
        std::fs::set_permissions(path, permissions)
    }

    #[cfg(windows)]
    #[test]
    fn trunk_staging_allows_final_removal_and_preserves_moved_assets() -> Result<(), Box<dyn std::error::Error>> {
        let (dist, stage) = staging_fixture("remove")?;
        std::fs::write(stage.join("index.html"), b"sentinel build file")?;
        std::fs::create_dir(stage.join("fonts"))?;
        std::fs::write(stage.join("fonts/font.ttf"), b"unchanged font")?;
        set_readonly(&stage.join("fonts/font.ttf"), true)?;
        set_readonly(&stage, true)?;
        let parent_permissions = std::fs::metadata(&dist)?.permissions();
        prepare_trunk_staging(&stage, &dist)?;
        assert!(!std::fs::metadata(&stage)?.permissions().readonly());
        assert!(std::fs::metadata(stage.join("fonts/font.ttf"))?.permissions().readonly());
        assert_eq!(std::fs::metadata(&dist)?.permissions(), parent_permissions);
        std::fs::rename(stage.join("index.html"), dist.join("index.html"))?;
        std::fs::rename(stage.join("fonts"), dist.join("fonts"))?;
        std::fs::remove_dir(&stage)?;
        assert_eq!(std::fs::read(dist.join("index.html"))?, b"sentinel build file");
        assert_eq!(std::fs::read(dist.join("fonts/font.ttf"))?, b"unchanged font");
        set_readonly(&dist.join("fonts/font.ttf"), false)?;
        std::fs::remove_file(dist.join("fonts/font.ttf"))?;
        std::fs::remove_dir(dist.join("fonts"))?;
        std::fs::remove_file(dist.join("index.html"))?;
        std::fs::remove_dir(dist)?;
        Ok(())
    }

    #[cfg(windows)]
    #[test]
    fn trunk_repeated_publications_can_remove_old_readonly_directories() -> Result<(), Box<dyn std::error::Error>> {
        use std::os::windows::fs::MetadataExt;

        let (dist, stage) = staging_fixture("repeat")?;
        set_readonly(&dist, true)?;
        let root_attributes = std::fs::metadata(&dist)?.file_attributes();
        for generation in 0..2 {
            if generation != 0 {
                std::fs::create_dir(&stage)?;
                // Model directories inherited by the prior distribution.
                set_readonly(&dist.join("fonts"), true)?;
                set_readonly(&dist.join("fonts/nested"), true)?;
            }
            std::fs::create_dir_all(stage.join("fonts/nested"))?;
            let bytes: &[u8] = if generation == 0 { b"font one" } else { b"font two" };
            std::fs::write(stage.join("fonts/nested/font.ttf"), bytes)?;
            std::fs::write(stage.join("index.html"), bytes)?;
            let file_attributes = std::fs::metadata(stage.join("fonts/nested/font.ttf"))?.file_attributes();
            for path in [&stage, &stage.join("fonts"), &stage.join("fonts/nested")] {
                set_readonly(path, true)?;
            }

            prepare_trunk_staging(&stage, &dist)?;
            assert_eq!(std::fs::metadata(&dist)?.file_attributes(), root_attributes);
            for path in [&stage, &stage.join("fonts"), &stage.join("fonts/nested")] {
                assert!(!std::fs::metadata(path)?.permissions().readonly());
            }
            assert_eq!(std::fs::metadata(stage.join("fonts/nested/font.ttf"))?.file_attributes(), file_attributes);
            assert_eq!(std::fs::read(stage.join("fonts/nested/font.ttf"))?, bytes);
            if generation != 0 {
                assert!(!std::fs::metadata(dist.join("fonts"))?.permissions().readonly());
                assert!(!std::fs::metadata(dist.join("fonts/nested"))?.permissions().readonly());
                assert_eq!(std::fs::metadata(dist.join("fonts/nested/font.ttf"))?.file_attributes(), file_attributes);
                assert_eq!(std::fs::read(dist.join("fonts/nested/font.ttf"))?, b"font one");
            }

            // Follow Trunk's clean_final/move_stage_to_final/remove_dir sequence.
            for entry in std::fs::read_dir(&dist)? {
                let entry = entry?;
                if entry.file_name() == ".stage" {
                    continue;
                }
                if entry.file_type()?.is_dir() {
                    std::fs::remove_dir_all(entry.path())?;
                } else {
                    std::fs::remove_file(entry.path())?;
                }
            }
            for entry in std::fs::read_dir(&stage)? {
                let entry = entry?;
                std::fs::rename(entry.path(), dist.join(entry.file_name()))?;
            }
            std::fs::remove_dir(&stage)?;
            assert!(!stage.exists());
            assert_eq!(std::fs::read(dist.join("fonts/nested/font.ttf"))?, bytes);
            assert_eq!(std::fs::read(dist.join("index.html"))?, bytes);
            assert_eq!(std::fs::metadata(dist.join("fonts/nested/font.ttf"))?.file_attributes(), file_attributes);
        }
        set_readonly(&dist, false)?;
        std::fs::remove_dir_all(dist)?;
        Ok(())
    }

    #[cfg(windows)]
    #[test]
    fn trunk_preparation_does_not_follow_descendant_directory_links() -> Result<(), Box<dyn std::error::Error>> {
        let (dist, stage) = staging_fixture("descendant-links")?;
        let target = dist.with_extension("outside");
        assert_eq!(target.parent(), dist.parent());
        assert!(target.starts_with(crate::root().canonicalize()?.join("target")));
        std::fs::create_dir(&target)?;
        std::fs::write(target.join("font.ttf"), b"outside distribution")?;
        set_readonly(&target, true)?;
        let link = stage.join("fonts");
        let linked = std::os::windows::fs::symlink_dir(&target, &link);
        if let Err(error) = linked {
            set_readonly(&target, false)?;
            std::fs::remove_file(target.join("font.ttf"))?;
            std::fs::remove_dir(&target)?;
            std::fs::remove_dir(&stage)?;
            std::fs::remove_dir(&dist)?;
            if error.raw_os_error() == Some(1314) {
                eprintln!("directory-link regression skipped: Windows requires Developer Mode or symlink privilege");
                return Ok(());
            }
            return Err(error.into());
        }
        set_readonly(&stage, true)?;
        prepare_trunk_staging(&stage, &dist)?;
        assert!(!std::fs::metadata(&stage)?.permissions().readonly());
        assert!(std::fs::metadata(&target)?.permissions().readonly());
        assert_eq!(std::fs::read(target.join("font.ttf"))?, b"outside distribution");
        std::fs::remove_dir(link)?;
        set_readonly(&target, false)?;
        std::fs::remove_file(target.join("font.ttf"))?;
        std::fs::remove_dir(target)?;
        std::fs::remove_dir(stage)?;
        std::fs::remove_dir(dist)?;
        Ok(())
    }

    #[cfg(windows)]
    #[test]
    fn trunk_staging_rejects_other_names_parents_and_files_without_changing_them() -> Result<(), Box<dyn std::error::Error>> {
        let (dist, stage) = staging_fixture("guards")?;
        set_readonly(&stage, true)?;
        assert!(prepare_trunk_staging(&stage, &crate::root()).is_err());
        assert!(std::fs::metadata(&stage)?.permissions().readonly());
        std::fs::create_dir(dist.join("arbitrary"))?;
        set_readonly(&dist.join("arbitrary"), true)?;
        assert!(prepare_trunk_staging(&dist.join("arbitrary"), &dist).is_err());
        assert!(std::fs::metadata(dist.join("arbitrary"))?.permissions().readonly());
        prepare_trunk_staging(&stage, &dist)?;
        std::fs::remove_dir(&stage)?;
        std::fs::write(&stage, b"file, not directory")?;
        set_readonly(&stage, true)?;
        assert!(prepare_trunk_staging(&stage, &dist).is_err());
        assert!(std::fs::metadata(&stage)?.permissions().readonly());
        assert_eq!(std::fs::read(&stage)?, b"file, not directory");
        set_readonly(&stage, false)?;
        std::fs::remove_file(stage)?;
        set_readonly(&dist.join("arbitrary"), false)?;
        std::fs::remove_dir(dist.join("arbitrary"))?;
        std::fs::remove_dir(dist)?;
        Ok(())
    }

    #[cfg(windows)]
    #[test]
    fn explicit_site_generation_keeps_directory_permissions() -> Result<(), Box<dyn std::error::Error>> {
        let (dist, stage) = staging_fixture("explicit")?;
        std::fs::write(stage.join("index.html"), "<html>")?;
        std::fs::create_dir(stage.join("fonts"))?;
        std::fs::write(stage.join("fonts/font.ttf"), b"explicit font")?;
        set_readonly(&stage.join("fonts/font.ttf"), true)?;
        set_readonly(&stage.join("fonts"), true)?;
        set_readonly(&stage, true)?;
        let explicit = stage.to_string_lossy();
        run(&crate::root(), Some(&explicit))?;
        assert!(std::fs::metadata(&stage)?.permissions().readonly());
        assert!(std::fs::metadata(stage.join("fonts"))?.permissions().readonly());
        assert!(std::fs::metadata(stage.join("fonts/font.ttf"))?.permissions().readonly());
        assert_eq!(std::fs::read(stage.join("fonts/font.ttf"))?, b"explicit font");
        set_readonly(&stage.join("fonts/font.ttf"), false)?;
        set_readonly(&stage.join("fonts"), false)?;
        std::fs::remove_file(stage.join("fonts/font.ttf"))?;
        std::fs::remove_dir(stage.join("fonts"))?;
        set_readonly(&stage, false)?;
        for name in ["index.html", SW, BUILD] {
            std::fs::remove_file(stage.join(name))?;
        }
        std::fs::remove_dir(stage)?;
        std::fs::remove_dir(dist)?;
        Ok(())
    }

    #[cfg(windows)]
    #[test]
    fn trunk_staging_rejects_directory_links_without_changing_targets() -> Result<(), Box<dyn std::error::Error>> {
        let (dist, stage) = staging_fixture("links")?;
        std::fs::remove_dir(&stage)?;
        let target = dist.join("target");
        std::fs::create_dir(&target)?;
        set_readonly(&target, true)?;
        let linked = std::os::windows::fs::symlink_dir(&target, &stage);
        if let Err(error) = linked {
            set_readonly(&target, false)?;
            std::fs::remove_dir(&target)?;
            std::fs::remove_dir(&dist)?;
            if error.raw_os_error() == Some(1314) {
                eprintln!("directory-link regression skipped: Windows requires Developer Mode or symlink privilege");
                return Ok(());
            }
            return Err(error.into());
        }
        assert!(prepare_trunk_staging(&stage, &dist).is_err());
        assert!(std::fs::metadata(&target)?.permissions().readonly());
        std::fs::remove_dir(stage)?;
        set_readonly(&target, false)?;
        std::fs::remove_dir(target)?;
        std::fs::remove_dir(dist)?;
        Ok(())
    }
}
