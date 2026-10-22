//! Presentation of SolidCraft-owned prose. Values captured from a formatted diagnostic
//! stay byte-for-byte intact, except fields audited as nested project-authored prose.
//! A message absent from the catalog stays intact too.
//!
//! This module uses only std and serde_json so workspace tooling can share it without
//! depending on the engine or changing lower-layer error types.

use std::collections::BTreeMap;
use std::sync::OnceLock;

const SOURCE: &str = include_str!("../../../../assets/locales/diagnostics/pt-BR.json");
const ARGUMENTS: &str = include_str!("../../../../assets/locales/diagnostics/arguments.json");
const MAX_BYTES: usize = 64 * 1024;
const MAX_SLOTS: usize = 32;
const MAX_STEPS: usize = 8192;
const PREFIX_BYTES: usize = 8;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Part {
    Text(String),
    Slot(usize),
}

struct Pattern {
    source: Vec<Part>,
    target: Vec<Part>,
    slots: usize,
    weight: usize,
    prose: Vec<bool>,
    wordy: bool,
}

#[derive(Default)]
struct Catalog {
    plain: BTreeMap<String, String>,
    patterns: BTreeMap<String, Vec<Pattern>>,
    invalid: Vec<String>,
}

impl Catalog {
    #[cfg(test)]
    fn from_json(json: &str) -> Self {
        Self::with_arguments(json, "{}")
    }

    fn with_arguments(json: &str, arguments: &str) -> Self {
        let Ok(plain) = serde_json::from_str::<BTreeMap<String, String>>(json) else {
            return Self { invalid: vec!["invalid diagnostic catalog JSON".into()], ..Self::default() };
        };
        let mut catalog = Self { plain, ..Self::default() };
        let Ok(arguments) = serde_json::from_str::<BTreeMap<String, Vec<String>>>(arguments) else {
            catalog.invalid.push("invalid diagnostic arguments JSON".into());
            return catalog;
        };
        for source in arguments.keys().filter(|source| !catalog.plain.contains_key(*source)) {
            catalog.invalid.push(format!("unknown diagnostic arguments template: {source}"));
        }
        for (source, target) in &catalog.plain {
            let Some((parts, slots)) = parse(source) else { continue };
            if slots.is_empty() {
                continue;
            }
            let Some((target_parts, target_slots)) = parse(target) else {
                catalog.invalid.push(source.clone());
                continue;
            };
            if slots.len() > MAX_SLOTS || slots.len() != target_slots.len() || slots.iter().any(|name| !target_slots.contains(name)) {
                catalog.invalid.push(source.clone());
                continue;
            }
            let mut target_parts = target_parts;
            for part in &mut target_parts {
                if let Part::Slot(index) = part {
                    let Some(name) = target_slots.get(*index) else { continue };
                    let Some(mapped) = slots.iter().position(|slot| slot == name) else { continue };
                    *index = mapped;
                }
            }
            let weight = parts
                .iter()
                .map(|part| match part {
                    Part::Text(text) => text.len(),
                    Part::Slot(_) => 0,
                })
                .sum();
            let prefix = match parts.first() {
                Some(Part::Text(text)) => short_prefix(text),
                _ => "",
            };
            let wanted = arguments.get(source).map(Vec::as_slice).unwrap_or_default();
            let field_name = |token: &str| token.split(':').next().unwrap_or_default().trim_start_matches('@').to_string();
            for name in wanted.iter().filter(|name| !slots.iter().any(|token| field_name(token) == **name)) {
                catalog.invalid.push(format!("unknown diagnostic field {name}: {source}"));
            }
            let prose = slots.iter().map(|token| wanted.contains(&field_name(token))).collect();
            let wordy = parts.iter().any(|part| matches!(part, Part::Text(text) if text.chars().any(|ch| ch.is_ascii_alphabetic())));
            catalog.patterns.entry(prefix.to_string()).or_default().push(Pattern {
                source: parts,
                target: target_parts,
                slots: slots.len(),
                weight,
                prose,
                wordy,
            });
        }
        // Field names disappear in a rendered message. Equivalent forms therefore
        // share the most conservative field policy across all their producers.
        let mut policies: BTreeMap<Vec<Part>, Vec<bool>> = BTreeMap::new();
        for pattern in catalog.patterns.values().flatten() {
            let policy = policies.entry(pattern.source.clone()).or_insert_with(|| pattern.prose.clone());
            for (allowed, other) in policy.iter_mut().zip(&pattern.prose) {
                *allowed &= *other;
            }
        }
        for patterns in catalog.patterns.values_mut() {
            for pattern in patterns.iter_mut() {
                if let Some(policy) = policies.get(&pattern.source) {
                    pattern.prose.clone_from(policy);
                }
            }
            patterns.sort_by_key(|pattern| std::cmp::Reverse(pattern.weight));
        }
        catalog
    }

    fn translate(&self, source: &str) -> Option<String> {
        let mut steps = MAX_STEPS;
        self.translate_depth(source, 8, false, &mut steps)
    }

    fn translate_depth(&self, source: &str, remaining: usize, nested_prose: bool, steps: &mut usize) -> Option<String> {
        if source.len() > MAX_BYTES || remaining == 0 {
            return None;
        }
        if let Some(target) = self.plain.get(source) {
            return Some(target.clone());
        }
        // Each pattern is indexed by its first eight literal bytes (or fewer). Only
        // prefixes of this message can match; no scan through the whole catalog.
        for end in (0..=source.len().min(PREFIX_BYTES)).rev().filter(|end| source.is_char_boundary(*end)) {
            let Some(prefix) = source.get(..end) else { continue };
            let Some(patterns) = self.patterns.get(prefix) else { continue };
            for pattern in patterns {
                // Captured data and punctuation alone do not identify a diagnostic.
                // Such templates are used only in an audited nested prose field.
                if !pattern.wordy && !nested_prose {
                    continue;
                }
                let mut values = vec![None; pattern.slots];
                let mut assignments = Vec::new();
                matches(&pattern.source, source, &mut values, steps, &mut assignments);
                if assignments.len() > 1 || *steps == 0 {
                    return None;
                }
                if let Some(values) = assignments.first() {
                    let mut output = String::with_capacity(source.len());
                    for part in &pattern.target {
                        match part {
                            Part::Text(text) => output.push_str(text),
                            Part::Slot(index) => {
                                let value = values.get(*index).copied().unwrap_or_default();
                                if pattern.prose.get(*index).copied().unwrap_or(false) {
                                    output.push_str(&self.translate_depth(value, remaining - 1, true, steps).unwrap_or_else(|| value.to_string()));
                                } else {
                                    output.push_str(value);
                                }
                            }
                        }
                    }
                    return Some(output);
                }
                if *steps == 0 {
                    return None;
                }
            }
        }
        None
    }
}

fn short_prefix(text: &str) -> &str {
    let mut end = text.len().min(PREFIX_BYTES);
    while !text.is_char_boundary(end) {
        end = end.saturating_sub(1);
    }
    text.get(..end).unwrap_or_default()
}

/// Rust format fields are treated as opaque tokens, including formatting modifiers.
/// Anonymous fields get distinct positions. Escaped braces are literal braces.
fn parse(text: &str) -> Option<(Vec<Part>, Vec<String>)> {
    let mut parts = Vec::new();
    let mut slots = Vec::new();
    let mut literal = String::new();
    let mut rest = text;
    let mut anonymous = 0;
    while !rest.is_empty() {
        if let Some(after) = rest.strip_prefix("{{") {
            literal.push('{');
            rest = after;
        } else if let Some(after) = rest.strip_prefix("}}") {
            literal.push('}');
            rest = after;
        } else if let Some(after) = rest.strip_prefix('{') {
            let (field, tail) = after.split_once('}')?;
            let name = field.split(':').next().unwrap_or_default();
            if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.') || field.contains(['{', '\n', '\r']) {
                return None;
            }
            if !literal.is_empty() {
                parts.push(Part::Text(std::mem::take(&mut literal)));
            } else if matches!(parts.last(), Some(Part::Slot(_))) {
                // Adjacent captures have no observable boundary; leave the diagnostic
                // canonical rather than inventing a partition of user-provided text.
                return None;
            }
            let token = if name.is_empty() {
                let token = format!("@{anonymous}{field}");
                anonymous += 1;
                token
            } else {
                field.to_string()
            };
            let index = slots.iter().position(|slot| slot == &token).unwrap_or_else(|| {
                slots.push(token);
                slots.len() - 1
            });
            parts.push(Part::Slot(index));
            rest = tail;
        } else if rest.starts_with('}') {
            return None;
        } else {
            let ch = rest.chars().next()?;
            literal.push(ch);
            rest = rest.get(ch.len_utf8()..)?;
        }
    }
    if !literal.is_empty() {
        parts.push(Part::Text(literal));
    }
    Some((parts, slots))
}

fn matches<'a>(parts: &[Part], text: &'a str, values: &mut [Option<&'a str>], steps: &mut usize, assignments: &mut Vec<Vec<&'a str>>) {
    if *steps == 0 || assignments.len() > 1 {
        return;
    }
    *steps -= 1;
    let Some((first, tail)) = parts.split_first() else {
        if text.is_empty() && values.iter().all(Option::is_some) {
            let assignment: Vec<_> = values.iter().filter_map(|value| *value).collect();
            if !assignments.contains(&assignment) {
                assignments.push(assignment);
            }
        }
        return;
    };
    match first {
        Part::Text(literal) => {
            if let Some(text) = text.strip_prefix(literal) {
                matches(tail, text, values, steps, assignments);
            }
        }
        Part::Slot(index) => {
            let Some(previous) = values.get(*index).copied() else { return };
            if let Some(value) = previous {
                if let Some(text) = text.strip_prefix(value) {
                    matches(tail, text, values, steps, assignments);
                }
                return;
            }
            if tail.is_empty() {
                if let Some(slot) = values.get_mut(*index) {
                    *slot = Some(text);
                }
                matches(tail, "", values, steps, assignments);
                if let Some(slot) = values.get_mut(*index) {
                    *slot = None;
                }
                return;
            }
            let Some(Part::Text(next)) = tail.first() else { return };
            let saved = values.to_vec();
            for (end, _) in text.match_indices(next) {
                if *steps == 0 || assignments.len() > 1 {
                    break;
                }
                let (Some(value), Some(rest)) = (text.get(..end), text.get(end..)) else { continue };
                values.copy_from_slice(&saved);
                if let Some(slot) = values.get_mut(*index) {
                    *slot = Some(value);
                }
                matches(tail, rest, values, steps, assignments);
            }
            values.copy_from_slice(&saved);
        }
    }
}

fn catalog() -> &'static Catalog {
    static CATALOG: OnceLock<Catalog> = OnceLock::new();
    CATALOG.get_or_init(|| {
        let catalog = Catalog::with_arguments(SOURCE, ARGUMENTS);
        for error in &catalog.invalid {
            eprintln!("invalid embedded diagnostic translation: {error}");
        }
        catalog
    })
}

pub fn translate(source: &str) -> String {
    catalog().translate(source).unwrap_or_else(|| source.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_templates_keep_their_format_fields() {
        assert!(catalog().invalid.is_empty(), "invalid diagnostic templates: {:?}", catalog().invalid);
    }

    #[test]
    fn values_and_unknown_messages_stay_literal() {
        let catalog = Catalog::from_json(
            r#"{"body {id} named `{name}` is missing":"não existe o corpo {id} chamado `{name}`","{name}: face {face} failed":"falha na face {face}: {name}","got {} instead of {}":"recebido {} em vez de {}"}"#,
        );
        assert_eq!(catalog.translate("body 17 named `Body 8: {name}` is missing").as_deref(), Some("não existe o corpo 17 chamado `Body 8: {name}`"));
        assert_eq!(catalog.translate("Boss: face 21 failed").as_deref(), Some("falha na face 21: Boss"));
        assert_eq!(catalog.translate("got 3 instead of 4").as_deref(), Some("recebido 3 em vez de 4"));
        assert_eq!(catalog.translate("OS denied access to Body 8"), None);
    }

    #[test]
    fn escaped_braces_and_repeated_fields_match_without_rewriting_data() {
        let catalog = Catalog::from_json(r#"{"invalid {{\"id\":{id}}} for {id}":"inválido {{\"id\":{id}}} para {id}"}"#);
        assert_eq!(catalog.translate("invalid {\"id\":23} for 23").as_deref(), Some("inválido {\"id\":23} para 23"));
        assert_eq!(catalog.translate("invalid {\"id\":23} for 24"), None);
    }

    #[test]
    fn a_mismatched_catalog_or_hostile_input_falls_back() {
        let catalog = Catalog::from_json(r#"{"expected {number}":"esperado {other}","value {a}, value {b}!":"valor {a}, valor {b}!"}"#);
        assert_eq!(catalog.invalid.len(), 1);
        assert_eq!(catalog.translate("expected 8"), None);
        assert_eq!(catalog.translate(&format!("value {}", "é".repeat(MAX_BYTES))), None);
        assert_eq!(catalog.translate(&format!("value {}", "value ".repeat(5000))), None);
    }

    #[test]
    fn only_audited_prose_arguments_are_translated() {
        let catalog = Catalog::with_arguments(
            r#"{"body `{name}` failed: {error}":"falha no corpo `{name}`: {error}","division by zero":"divisão por zero"}"#,
            r#"{"body `{name}` failed: {error}":["error"]}"#,
        );
        assert!(catalog.invalid.is_empty());
        assert_eq!(
            catalog.translate("body `division by zero` failed: division by zero").as_deref(),
            Some("falha no corpo `division by zero`: divisão por zero")
        );
        assert_eq!(
            catalog.translate("body `division by zero` failed: OS error (code 5)").as_deref(),
            Some("falha no corpo `division by zero`: OS error (code 5)")
        );
    }

    #[test]
    fn ambiguous_delimiters_in_authored_names_preserve_the_whole_diagnostic() {
        let catalog = Catalog::from_json(
            r#"{"rule `{name}` references missing parameter `{parameter}`":"a regra `{name}` referencia o parâmetro ausente `{parameter}`"}"#,
        );
        let source = "rule `A` references missing parameter `x` references missing parameter `missing`";
        assert_eq!(catalog.translate(source), None);
        assert_eq!(
            catalog.translate("rule `A` references missing parameter `missing`").as_deref(),
            Some("a regra `A` referencia o parâmetro ausente `missing`")
        );
    }

    #[test]
    fn equivalent_rendered_forms_use_the_intersection_of_prose_policies() {
        let catalog = Catalog::with_arguments(
            r#"{"{what} no longer exists":"{what} não existe mais","{reference} no longer exists":"{reference} não existe mais","the edge":"a aresta"}"#,
            r#"{"{what} no longer exists":["what"]}"#,
        );
        assert_eq!(catalog.translate("the edge no longer exists").as_deref(), Some("the edge não existe mais"));
    }
}
