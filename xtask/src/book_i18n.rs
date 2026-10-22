//! Translate mdBook's generated chrome without changing chapter content or the English book.
//!
//! The editable master is `assets/locales/book/pt-BR.json`. mdBook owns the generated HTML
//! and JavaScript; only its presentation strings are translated after the Portuguese build.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

type Strings = BTreeMap<String, String>;
type Catalog = BTreeMap<String, Strings>;

pub fn run(root: &Path) -> Result<(), String> {
    let catalog_path = root.join("assets/locales/book/pt-BR.json");
    let text = read(&catalog_path)?;
    let catalog: Catalog = serde_json::from_str(&text).map_err(|e| format!("{}: {e}", catalog_path.display()))?;
    if catalog.len() != 2 {
        return Err("book chrome catalog must contain only html and javascript groups".into());
    }
    let html = catalog.get("html").ok_or("book chrome catalog: missing html group")?;
    let javascript = catalog.get("javascript").ok_or("book chrome catalog: missing javascript group")?;
    for (group, strings) in &catalog {
        if strings.is_empty() || strings.iter().any(|(source, translation)| source.trim().is_empty() || translation.trim().is_empty()) {
            return Err(format!("book chrome catalog: empty entry in {group}"));
        }
    }

    let output = root.join("target/book/pt-BR");
    let mut names = BTreeMap::new();
    let mut scripts = Vec::new();
    let mut js_used = BTreeSet::new();
    for entry in std::fs::read_dir(&output).map_err(|e| format!("{}: {e}", output.display()))? {
        let entry = entry.map_err(|e| format!("book chrome script: {e}"))?;
        if !entry.file_type().map_err(|e| format!("book chrome script: {e}"))?.is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        let prefix = if name == "book.js" || (name.starts_with("book-") && name.ends_with(".js")) {
            "book"
        } else if name == "searcher.js" || (name.starts_with("searcher-") && name.ends_with(".js")) {
            "searcher"
        } else {
            continue;
        };
        let source = read(&entry.path())?;
        let translated = localize_javascript(&source, javascript, &mut js_used);
        // Keep cache keys tied to the translated content, including changes to the catalog.
        let digest = Sha256::digest(translated.as_bytes());
        let hash: String = digest.iter().take(4).map(|byte| format!("{byte:02x}")).collect();
        let translated_name = format!("{prefix}-{hash}.js");
        names.insert(name, translated_name.clone());
        scripts.push((entry.path(), output.join(translated_name), translated));
    }

    let mut paths = Vec::new();
    html_files(&output, &mut paths)?;
    let mut pages = Vec::new();
    let mut html_used = BTreeSet::new();
    for path in paths {
        let source = read(&path)?;
        let not_found = path.file_name().is_some_and(|name| name == "404.html");
        let translated = localize_html(&source, html, &names, not_found, &mut html_used)?;
        pages.push((path, translated));
    }
    require_coverage("html", html, &html_used)?;
    require_coverage("javascript", javascript, &js_used)?;

    for (_, translated_path, text) in &scripts {
        write(translated_path, text)?;
    }
    for (path, text) in pages {
        write(&path, &text)?;
    }
    for (source_path, translated_path, _) in scripts {
        if source_path != translated_path {
            std::fs::remove_file(&source_path).map_err(|e| format!("{}: {e}", source_path.display()))?;
        }
    }
    Ok(())
}

fn read(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))
}

fn write(path: &Path, text: &str) -> Result<(), String> {
    std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

fn html_files(directory: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in std::fs::read_dir(directory).map_err(|e| format!("{}: {e}", directory.display()))? {
        let entry = entry.map_err(|e| format!("book chrome page: {e}"))?;
        let kind = entry.file_type().map_err(|e| format!("book chrome page: {e}"))?;
        if kind.is_dir() {
            html_files(&entry.path(), paths)?;
        } else if kind.is_file() && entry.path().extension().is_some_and(|extension| extension == "html") {
            paths.push(entry.path());
        }
    }
    paths.sort();
    Ok(())
}

fn require_coverage(group: &str, strings: &Strings, used: &BTreeSet<String>) -> Result<(), String> {
    let missing: Vec<_> = strings.keys().filter(|source| !used.contains(*source)).collect();
    if !missing.is_empty() {
        return Err(format!("book chrome: {group} strings not found in mdBook output: {missing:?}; check the mdBook version and catalog"));
    }
    Ok(())
}

fn localize_javascript(source: &str, strings: &Strings, used: &mut BTreeSet<String>) -> String {
    let mut translated = source.to_owned();
    let mut entries: Vec<_> = strings.iter().collect();
    // Complete messages may contain another catalog entry, such as the plural
    // results suffix inside "No search results for ". Translate them first.
    entries.sort_by_key(|(english, _)| std::cmp::Reverse(english.len()));
    for (english, portuguese) in entries {
        if translated.contains(english) || translated.contains(portuguese) {
            used.insert(english.clone());
        }
        translated = translated.replace(english, portuguese);
    }
    translated
}

fn html_fragment(source: &str, strings: &Strings, names: &BTreeMap<String, String>, used: &mut BTreeSet<String>) -> String {
    let mut translated = source.replace("<html lang=\"en\">", "<html lang=\"pt-BR\">");
    for (english, portuguese) in strings {
        for attribute in ["title", "aria-label", "placeholder", "content"] {
            let source = format!("{attribute}=\"{english}\"");
            let replacement = format!("{attribute}=\"{portuguese}\"");
            if translated.contains(&source) || translated.contains(&replacement) {
                used.insert(english.clone());
            }
            translated = translated.replace(&source, &replacement);
        }
        let source = format!(">{english}<");
        let replacement = format!(">{portuguese}<");
        if translated.contains(&source) || translated.contains(&replacement) {
            used.insert(english.clone());
        }
        translated = translated.replace(&source, &replacement);
    }
    for (source, replacement) in names {
        translated = translated.replace(source, replacement);
    }
    translated
}

fn localize_html(
    source: &str,
    strings: &Strings,
    names: &BTreeMap<String, String>,
    not_found: bool,
    used: &mut BTreeSet<String>,
) -> Result<String, String> {
    let Some((before, rest)) = source.split_once("<main>") else {
        if source.contains("<main") {
            return Err("book chrome: unsupported main element; chapter content was left unchanged".into());
        }
        // Redirects and the standalone table of contents contain no chapter content.
        return Ok(html_fragment(source, strings, names, used));
    };
    let Some((chapter, after)) = rest.split_once("</main>") else {
        return Err("book chrome: unclosed main element; chapter content was left unchanged".into());
    };
    let before = html_fragment(before, strings, names, used);
    let after = html_fragment(after, strings, names, used);
    // mdBook's built-in 404 text is presentation chrome, rather than an authored chapter.
    let chapter = if not_found { html_fragment(chapter, strings, names, used) } else { chapter.to_owned() };
    Ok(format!("{before}<main>{chapter}</main>{after}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn catalog() -> Catalog {
        serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/locales/book/pt-BR.json")))
            .expect("book chrome catalog is valid JSON")
    }

    #[test]
    fn chapter_code_and_links_remain_literal() {
        let catalog = catalog();
        let strings = catalog.get("html").expect("html translations");
        let names = BTreeMap::from([("book-old.js".into(), "book-new.js".into())]);
        let chapter =
            r#"<pre><code>Keyboard shortcuts; book-old.js</code></pre><a title="Next chapter" href="guide/sketching.html#profiles">literal link</a>"#;
        let source = format!(
            r#"<html lang="pt-BR"><h2>Keyboard shortcuts</h2><main>{chapter}</main><a title="Next chapter" href="guide/sketching.html">→</a><script src="../book-old.js"></script>"#
        );
        let translated = localize_html(&source, strings, &names, false, &mut BTreeSet::new()).expect("complete main element");
        assert!(translated.contains(&format!("<main>{chapter}</main>")));
        assert!(translated.contains("<h2>Atalhos de teclado</h2>"));
        assert!(translated.contains(r#"title="Próximo capítulo" href="guide/sketching.html""#));
        assert!(translated.contains(r#"src="../book-new.js""#));
    }

    #[test]
    fn not_found_and_redirects_use_portuguese_without_changing_targets() {
        let catalog = catalog();
        let strings = catalog.get("html").expect("html translations");
        let source = "<html lang=\"en\"><title>Redirecting...</title><p>Redirecting to... <a href=\"../guide/mcp.html\">../guide/mcp.html</a>.</p>";
        let translated = localize_html(source, strings, &BTreeMap::new(), false, &mut BTreeSet::new()).expect("redirect");
        assert!(translated.contains("<html lang=\"pt-BR\">"));
        assert!(translated.contains("<title>Redirecionando...</title>"));
        assert!(translated.contains("<p>Redirecionando para... <a href=\"../guide/mcp.html\">../guide/mcp.html</a>.</p>"));
        let source = "<main><h1 id=\"document-not-found-404\">Document not found (404)</h1><p>This URL is invalid, sorry. Please use the navigation bar or search to continue.</p></main>";
        let translated = localize_html(source, strings, &BTreeMap::new(), true, &mut BTreeSet::new()).expect("404 page");
        assert!(translated.contains("id=\"document-not-found-404\">Documento não encontrado (404)"));
        assert!(translated.contains("Use a barra de navegação ou a pesquisa para continuar."));
    }

    #[test]
    fn javascript_preserves_selectors_and_interpolation() {
        let catalog = catalog();
        let strings = catalog.get("javascript").expect("javascript translations");
        let source = "document.getElementById('mdbook-searchresults-header').innerText = count + ' search results for \\' ' + searchterm; this.title = 'Show hidden lines'; if (this.title === 'Show hidden lines') { block.classList.remove('hide-boring'); }";
        let translated = localize_javascript(source, strings, &mut BTreeSet::new());
        assert!(translated.contains("document.getElementById('mdbook-searchresults-header')"));
        assert!(translated.contains("count + ' resultados para \\' ' + searchterm"));
        assert!(translated.contains("this.title === 'Mostrar linhas ocultas'"));
        assert!(translated.contains("block.classList.remove('hide-boring')"));
    }

    #[test]
    fn search_messages_keep_zero_singular_and_plural_results_distinct() {
        let catalog = catalog();
        let strings = catalog.get("javascript").expect("javascript translations");
        let source =
            "'No search results for ' + searchterm; count + ' search result for ' + searchterm; count + ' search results for ' + searchterm;";
        let mut used = BTreeSet::new();
        let translated = localize_javascript(source, strings, &mut used);
        assert_eq!(
            translated,
            "'Nenhum resultado para ' + searchterm; count + ' resultado para ' + searchterm; count + ' resultados para ' + searchterm;"
        );
        for source in ["No search results for ", " search result for ", " search results for "] {
            assert!(used.contains(source), "search translation not counted: {source}");
        }
    }
}
