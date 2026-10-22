//! `solidcraft-cli crashes`: the crash reports the desktop app wrote on this computer
//! (docs/crash-reports.md): list them, show one, replay one, delete them.

use std::sync::Mutex;

use serde_json::{Value, json};
use solidcraft_engine::crash;
use solidcraft_engine::i18n::{Locale, interpolate};

const USAGE: &str = "usage: solidcraft-cli crashes [list [--json] | show <id> | replay <id> | dir | clear [<id>]]";

pub fn run(args: &[String], locale: Locale) -> Result<(), String> {
    let dir = crash::default_dir().ok_or_else(|| locale.protocol_label("no crash folder on this system").to_string())?;
    let id = || args.get(1).map(String::as_str).ok_or_else(|| locale.protocol_label(USAGE).to_string());
    match args.first().map(String::as_str) {
        None | Some("list") => {
            let reports = crash::list(&dir);
            if args.iter().any(|a| a == "--json") {
                println!("{}", super::pretty(&json!({ "dir": dir.to_string_lossy(), "reports": reports })));
                return Ok(());
            }
            println!("{}", interpolate(locale.protocol_label("crash reports in {dir}"), &[("dir", &dir.display().to_string())]));
            if reports.is_empty() {
                println!("{}", locale.protocol_label("  (none)"));
            }
            for r in reports {
                let times = if r.count > 1 { format!(" ×{}", r.count) } else { String::new() };
                let cmd = r.command.map(|c| interpolate(locale.protocol_label(" in {c}"), &[("c", &c)])).unwrap_or_default();
                let replay = if r.replayable { locale.protocol_label("  [replayable]") } else { "" };
                println!("  {}  {}{times}{cmd}  v{}{replay}\n      {}", r.id, r.kind, r.version, locale.protocol_error(&r.message));
            }
            Ok(())
        }
        Some("show") => {
            println!("{}", super::pretty(&crash::read(&dir, id()?)?));
            Ok(())
        }
        Some("replay") => replay(&crash::read(&dir, id()?)?, locale),
        Some("dir") => {
            println!("{}", dir.display());
            Ok(())
        }
        Some("clear") => {
            let count = crash::clear(&dir, args.get(1).map(String::as_str)).to_string();
            println!("{}", interpolate(locale.protocol_label("deleted {count} report(s)"), &[("count", &count)]));
            Ok(())
        }
        _ => Err(locale.protocol_label(USAGE).to_string()),
    }
}

/// Panics seen while replaying: (location, message).
static PANICS: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());

/// Run a report's commands on its design and say whether the crash happens again.
fn replay(report: &Value, locale: Locale) -> Result<(), String> {
    let id = report["id"].as_str().unwrap_or_default();
    let replay_path = report["replay_path"]
        .as_str()
        .ok_or_else(|| interpolate(locale.protocol_label("report {id} cannot be replayed (see its notes)"), &[("id", id)]))?;
    let script = super::read_json(replay_path)?;
    let commands = script["commands"].as_array().cloned().unwrap_or_default();
    let mut s = match report["design_path"].as_str() {
        Some(d) => super::load(d)?,
        None => solidcraft_engine::Session::default(),
    };
    std::panic::set_hook(Box::new(|info| {
        let loc = info.location().map_or_else(|| "(unknown)".to_string(), |l| format!("{}:{}", l.file(), l.line()));
        let msg = info.payload_as_str().unwrap_or("(not a text message)").to_string();
        if let Ok(mut p) = PANICS.lock() {
            p.push((loc, msg));
        }
    }));
    if report["kind"] == "unclean_exit" {
        println!("{}", locale.protocol_label("this crash ended the app without a report: if this process ends abruptly, it reproduced"));
    }
    let count = commands.len().to_string();
    let design = report["design_path"].as_str().unwrap_or_else(|| locale.protocol_label("a new design"));
    println!("{}", interpolate(locale.protocol_label("replaying {count} command(s) on {design}"), &[("count", &count), ("design", design)]));
    for (i, c) in commands.iter().enumerate() {
        let cmd = c["command"].as_str().unwrap_or_default();
        let r = s.execute(cmd, c.get("params").unwrap_or(&Value::Null));
        match r {
            Ok(_) => println!("  {:>3}. {cmd}: {}", i + 1, locale.protocol_error("ok")),
            Err(e) => println!("  {:>3}. {cmd}: {}", i + 1, locale.protocol_error(&e.to_string())),
        }
    }
    let _ = std::panic::take_hook();
    let seen = PANICS.lock().map(|p| p.clone()).unwrap_or_default();
    let want = report["signature"].as_str().unwrap_or_default();
    let same = seen.iter().any(|(l, m)| format!("{:016x}", crash::signature(l, m)) == want);
    if same {
        println!(
            "{}",
            interpolate(
                locale.protocol_label("reproduced: the same internal error ({message}) at {location}"),
                &[("message", report["message"].as_str().unwrap_or_default()), ("location", report["location"].as_str().unwrap_or_default())]
            )
        );
    } else if let Some((l, m)) = seen.first() {
        println!("{}", interpolate(locale.protocol_label("an internal error happened, but not the reported one: {m} at {l}"), &[("m", m), ("l", l)]));
        println!(
            "{}",
            interpolate(
                locale.protocol_label("(the report says: {message} at {location})"),
                &[("message", report["message"].as_str().unwrap_or_default()), ("location", report["location"].as_str().unwrap_or_default())]
            )
        );
    } else {
        println!("{}", locale.protocol_label("did not reproduce: no internal error (fixed, or it depends on something the replay does not have)"));
    }
    Ok(())
}
