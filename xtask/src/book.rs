//! `cargo xtask book`: generate the book's command reference and screenshots, then build it.
//!
//! The command reference (`docs/book/src/reference/commands.md`) comes from
//! `solidcraft-cli commands`, so it can't drift from the registry. The screenshots in
//! `docs/book/src/generated/` are GPU renders by `solidcraft-cli snapshot` of `examples/*.json`
//! and `docs/book/scripts/*.json`, plus the app screenshot from `docs/screenshots/`. Both are
//! gitignored and attributed in ATTRIBUTION.md. The book itself is built by `mdbook` (a separate
//! tool, not a dependency) into `target/book`.

use std::path::Path;

use serde_json::Value;

use crate::cargo;

#[path = "book_i18n.rs"]
mod book_i18n;

/// Toolbar tabs in toolbar order; commands not on the toolbar come last, grouped by id prefix.
const TABS: &[&str] = &["SOLID", "SKETCH", "SURFACE", "SHEET METAL", "PLASTIC"];
const SNAPSHOT_SIZE: (&str, &str) = ("960", "600");

pub fn run(root: &Path, build: bool) -> Result<(), String> {
    let book = root.join("docs/book");
    std::fs::copy(root.join("assets/branding/web/source/og.png"), book.join("src/og.png")).map_err(|e| format!("book preview card: {e}"))?;
    let mut c = cargo();
    c.args(["build", "-q", "--release", "-p", "solidcraft-cli"]);
    crate::run(c, "cargo build -p solidcraft-cli")?;

    let out = cli().arg("commands").output().map_err(|e| format!("solidcraft-cli: {e}"))?;
    if !out.status.success() {
        return Err(format!("solidcraft-cli commands failed:\n{}", String::from_utf8_lossy(&out.stderr)));
    }
    let cmds: Value = serde_json::from_slice(&out.stdout).map_err(|e| format!("commands JSON: {e}"))?;
    let md = reference(cmds.as_array().map(Vec::as_slice).unwrap_or_default());
    write(&book.join("src/reference/commands.md"), &md)?;
    let portuguese_book = root.join("docs/pt-BR/book");
    let portuguese_reference = reference_pt_br(root, cmds.as_array().map(Vec::as_slice).unwrap_or_default())?;
    write(&portuguese_book.join("src/reference/commands.md"), &portuguese_reference)?;

    let gen_dir = book.join("src/generated");
    std::fs::create_dir_all(&gen_dir).map_err(|e| format!("{}: {e}", gen_dir.display()))?;
    let mut scripts = vec![root.join("examples/bracket.json")];
    scripts.extend(json_files(&book.join("scripts"))?);
    for s in &scripts {
        let stem = s.file_stem().map(|x| x.to_string_lossy().to_string()).unwrap_or_default();
        let png = gen_dir.join(format!("{stem}.png"));
        let mut c = cli();
        c.arg("snapshot").arg(s).arg("--out").arg(&png).args(["--width", SNAPSHOT_SIZE.0, "--height", SNAPSHOT_SIZE.1]);
        let o = c.output().map_err(|e| format!("solidcraft-cli snapshot: {e}"))?;
        if !o.status.success() {
            return Err(format!("snapshot of {} failed:\n{}", s.display(), String::from_utf8_lossy(&o.stderr)));
        }
        println!("book: {}", png.strip_prefix(root).unwrap_or(&png).display());
    }
    let shot = "sample-plate.png";
    std::fs::copy(root.join("docs/screenshots").join(shot), gen_dir.join(shot)).map_err(|e| format!("{shot}: {e}"))?;
    let portuguese_generated = portuguese_book.join("src/generated");
    std::fs::create_dir_all(&portuguese_generated).map_err(|e| format!("{}: {e}", portuguese_generated.display()))?;
    for entry in std::fs::read_dir(&gen_dir).map_err(|e| format!("{}: {e}", gen_dir.display()))? {
        let entry = entry.map_err(|e| format!("book image: {e}"))?;
        if entry.path().is_file() {
            std::fs::copy(entry.path(), portuguese_generated.join(entry.file_name())).map_err(|e| format!("book image: {e}"))?;
        }
    }
    std::fs::copy(root.join("assets/documentation/screenshots/source/sample-plate-pt-BR.png"), portuguese_generated.join(shot))
        .map_err(|e| format!("Portuguese book screenshot: {e}"))?;

    if build {
        let mut c = std::process::Command::new("mdbook");
        c.arg("build").arg(&book);
        crate::run(c, "mdbook build docs/book").map_err(|e| format!("{e}\n(install mdbook with `cargo install mdbook`)"))?;
        let mut c = std::process::Command::new("mdbook");
        c.arg("build").arg(&portuguese_book);
        crate::run(c, "mdbook build docs/pt-BR/book")?;
        book_i18n::run(root)?;
        println!("book: built into target/book");
    }
    Ok(())
}

/// Use the same authoritative command registry, preserving command IDs and argument names.
/// Missing parameter documentation fails the build instead of silently publishing English.
fn reference_pt_br(root: &Path, cmds: &[Value]) -> Result<String, String> {
    let read = |relative: &str| -> Result<std::collections::BTreeMap<String, String>, String> {
        let path = root.join(relative);
        let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
    };
    let english = read("assets/locales/en.json")?;
    let portuguese = read("assets/locales/pt-BR.json")?;
    let parameter_docs = read("assets/locales/commands/pt-BR.json")?;
    let labels: std::collections::BTreeMap<&str, &str> = english.iter().map(|(id, text)| (text.as_str(), id.as_str())).collect();
    let translated =
        |label: &str| -> String { labels.get(label).and_then(|id| portuguese.get(*id)).map_or_else(|| label.to_string(), String::clone) };
    let field = |c: &Value, key: &str| c.get(key).and_then(Value::as_str).unwrap_or_default().to_string();
    let mut sections: Vec<(String, Vec<&Value>)> = Vec::new();
    for tab in TABS {
        for command in cmds.iter().filter(|c| field(c, "tab") == *tab) {
            add(&mut sections, format!("{} › {}", translated(tab), translated(&field(command, "panel"))), command);
        }
    }
    for command in cmds.iter().filter(|c| !TABS.contains(&field(c, "tab").as_str())) {
        let id = field(command, "id");
        add(&mut sections, format!("Outros: {}.*", id.split('.').next().unwrap_or_default()), command);
    }
    let mut md = format!(
        "# Referência de comandos\n\nGerada por `cargo xtask book` a partir de `solidcraft-cli commands` ({} comandos). \
         Cada comando funciona da mesma forma na barra de ferramentas, em scripts, em `solidcraft-cli exec <id> '<json>'`, \
         no canal de controle e no MCP. Parâmetros marcados com `?` são opcionais; `expr` é um número ou uma expressão com unidades.\n",
        cmds.len()
    );
    for (title, list) in sections {
        md.push_str(&format!("\n## {title}\n\n"));
        for command in list {
            let id = field(command, "id");
            let shortcut = command.get("shortcut").and_then(Value::as_str).map(|k| format!(" ({k})")).unwrap_or_default();
            md.push_str(&format!("- `{id}` **{}**{shortcut}", translated(&field(command, "label"))));
            if !field(command, "params").is_empty() {
                let parameters = parameter_docs.get(&id).ok_or_else(|| format!("pt-BR parameter documentation missing: {id}"))?;
                md.push_str(&format!("<br>`` {} ``", parameters.replace('\n', " ")));
            }
            md.push('\n');
        }
    }
    Ok(md)
}

/// The release solidcraft-cli just built.
fn cli() -> std::process::Command {
    let mut c = cargo();
    c.args(["run", "-q", "--release", "-p", "solidcraft-cli", "--", "--language", "en"]);
    c
}

fn write(path: &Path, text: &str) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

fn json_files(dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
    let rd = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let mut v: Vec<_> = rd.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "json")).collect();
    v.sort();
    Ok(v)
}

/// The command reference page: one section per toolbar tab and panel, in registry order.
fn reference(cmds: &[Value]) -> String {
    let s = |c: &Value, k: &str| c[k].as_str().unwrap_or_default().to_string();
    let mut md = format!(
        "# Command reference\n\nGenerated by `cargo xtask book` from `solidcraft-cli commands` ({} commands). Every command runs the same \
         way from the toolbar, a script, `solidcraft-cli exec <id> '<json>'`, the control channel and MCP. Parameters marked `?` are \
         optional; `expr` is a number or an expression with units.\n",
        cmds.len()
    );
    let mut sections: Vec<(String, Vec<&Value>)> = Vec::new();
    for tab in TABS {
        for c in cmds.iter().filter(|c| s(c, "tab") == *tab) {
            let title = format!("{tab} › {}", s(c, "panel"));
            add(&mut sections, title, c);
        }
    }
    for c in cmds.iter().filter(|c| !TABS.contains(&s(c, "tab").as_str())) {
        let id = s(c, "id");
        let prefix = id.split('.').next().unwrap_or_default().to_string();
        add(&mut sections, format!("Other: {prefix}.*"), c);
    }
    for (title, list) in sections {
        md.push_str(&format!("\n## {title}\n\n"));
        for c in list {
            let key = c["shortcut"].as_str().map(|k| format!(" ({k})")).unwrap_or_default();
            md.push_str(&format!("- `{}` **{}**{key}", s(c, "id"), s(c, "label")));
            let p = s(c, "params");
            if !p.is_empty() {
                md.push_str(&format!("<br>`` {} ``", p.replace('\n', " ")));
            }
            md.push('\n');
        }
    }
    md
}

fn add<'a>(sections: &mut Vec<(String, Vec<&'a Value>)>, title: String, c: &'a Value) {
    match sections.iter_mut().find(|(t, _)| *t == title) {
        Some((_, v)) => v.push(c),
        None => sections.push((title, vec![c])),
    }
}
