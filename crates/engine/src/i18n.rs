//! Shared immutable offline message catalogs for desktop, web, CLI and MCP presentation.

use std::collections::BTreeMap;
use std::sync::OnceLock;

mod diagnostics;

pub type Messages = BTreeMap<String, String>;

/// Register locales here after adding their catalogs. Missing translations use English.
pub struct Catalog {
    pub id: &'static str,
    pub name: &'static str,
    pub source: &'static str,
    messages: OnceLock<Messages>,
}

impl Catalog {
    pub const fn new(id: &'static str, name: &'static str, source: &'static str) -> Self {
        Self { id, name, source, messages: OnceLock::new() }
    }

    pub fn messages(&'static self) -> &'static Messages {
        self.messages.get_or_init(|| match serde_json::from_str(self.source) {
            Ok(messages) => messages,
            Err(error) => {
                log::error!("invalid embedded {} message catalog: {error}", self.id);
                Messages::new()
            }
        })
    }
}

pub static ENGLISH: Catalog = Catalog::new("en", "English", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/en.json")));
static CZECH: Catalog = Catalog::new("cs", "Čeština", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/cs.json")));
static DANISH: Catalog = Catalog::new("da", "Dansk", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/da.json")));
static GERMAN: Catalog = Catalog::new("de", "Deutsch", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/de.json")));
static ENGLISH_UK: Catalog =
    Catalog::new("en-GB", "English (UK)", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/en-GB.json")));
static SPANISH: Catalog = Catalog::new("es", "Español (España)", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/es.json")));
static FRENCH: Catalog = Catalog::new("fr", "Français", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/fr.json")));
static GALICIAN: Catalog = Catalog::new("gl", "Galego", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/gl.json")));
static HUNGARIAN: Catalog = Catalog::new("hu", "Magyar", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/hu.json")));
static INDONESIAN: Catalog =
    Catalog::new("id", "Bahasa Indonesia", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/id.json")));
static ITALIAN: Catalog = Catalog::new("it", "Italiano", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/it.json")));
static JAPANESE: Catalog = Catalog::new("ja", "日本語", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/ja.json")));
static KOREAN: Catalog = Catalog::new("ko", "한국어", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/ko.json")));
static MALAY: Catalog = Catalog::new("ms", "Bahasa Melayu", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/ms.json")));
static DUTCH: Catalog = Catalog::new("nl", "Nederlands", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/nl.json")));
static POLISH: Catalog = Catalog::new("pl", "Polski", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/pl.json")));
static PORTUGUESE_BR: Catalog =
    Catalog::new("pt-BR", "Português (Brasil)", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/pt-BR.json")));
static ROMANIAN: Catalog = Catalog::new("ro", "Română", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/ro.json")));
static RUSSIAN: Catalog = Catalog::new("ru", "Русский", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/ru.json")));
static SWEDISH: Catalog = Catalog::new("sv", "Svenska", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/sv.json")));
static TURKISH: Catalog = Catalog::new("tr", "Türkçe", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/tr.json")));
static UKRAINIAN: Catalog = Catalog::new("uk", "Українська", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/uk.json")));
static VIETNAMESE: Catalog = Catalog::new("vi", "Tiếng Việt", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/vi.json")));
static CHINESE: Catalog = Catalog::new("zh", "中文", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/zh.json")));
static CHINESE_TRADITIONAL: Catalog =
    Catalog::new("zh-TW", "繁體中文", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/zh-TW.json")));
static PT_BR_PROTOCOL: Catalog = Catalog::new(
    "pt-BR-protocol",
    "Português (Brasil)",
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/protocol/pt-BR.json")),
);
static PT_BR_COMMANDS: Catalog = Catalog::new(
    "pt-BR-commands",
    "Português (Brasil)",
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/locales/commands/pt-BR.json")),
);
/// English first (the fallback), then by language id, one per line so additions do not conflict.
#[rustfmt::skip]
pub const LOCALES: &[&Catalog] = &[
    &ENGLISH,
    &CZECH,
    &DANISH,
    &GERMAN,
    &ENGLISH_UK,
    &SPANISH,
    &FRENCH,
    &GALICIAN,
    &HUNGARIAN,
    &INDONESIAN,
    &ITALIAN,
    &JAPANESE,
    &KOREAN,
    &MALAY,
    &DUTCH,
    &POLISH,
    &PORTUGUESE_BR,
    &ROMANIAN,
    &RUSSIAN,
    &SWEDISH,
    &TURKISH,
    &UKRAINIAN,
    &VIETNAMESE,
    &CHINESE,
    &CHINESE_TRADITIONAL,
];

pub fn is_supported(id: &str) -> bool {
    LOCALES.iter().any(|catalog| catalog.id == id)
}

/// Immutable locale reference. Catalogs parse once; lookups are logarithmic.
#[derive(Clone, Copy)]
pub struct Locale(pub &'static Catalog);

impl Locale {
    pub fn for_language(id: &str) -> Self {
        Self(LOCALES.iter().copied().find(|catalog| catalog.id == id).unwrap_or(&ENGLISH))
    }

    pub fn name(self) -> &'static str {
        self.0.name
    }

    pub fn id(self) -> &'static str {
        self.0.id
    }

    /// Human-readable protocol descriptions may contain literal JSON examples. These are
    /// plain source-text translations, not interpolated UI message templates.
    pub fn protocol_label(self, source: &str) -> &str {
        if self.0.id == "pt-BR" {
            PT_BR_PROTOCOL.messages().get(source).map_or_else(|| self.label(source), String::as_str)
        } else {
            self.label(source)
        }
    }

    pub fn command_parameters<'a>(self, command: &str, source: &'a str) -> &'a str {
        if self.0.id == "pt-BR" { PT_BR_COMMANDS.messages().get(command).map_or(source, String::as_str) } else { source }
    }

    /// Only prose templates use interpolation; literal protocol/JSON examples use `protocol_label`.
    pub fn protocol_format(self, source: &str, args: &[(&str, &str)]) -> String {
        interpolate(self.protocol_label(source), args)
    }

    /// Translate error wrappers while preserving identifiers and third-party diagnostic details.
    pub fn protocol_error(self, source: &str) -> String {
        let translated = self.protocol_label(source);
        if translated != source || self.id() == "en" {
            return translated.to_string();
        }
        for (prefix, suffix, template, argument) in [
            ("unknown command `", "`", "unknown command `{command}`", "command"),
            ("unknown tool `", "`", "unknown tool `{name}`", "name"),
            ("unknown view `", "`", "unknown view `{v}`", "v"),
            ("unknown resource `", "`", "unknown resource `{uri}`", "uri"),
            ("method `", "` not found", "method `{method}` not found", "method"),
            ("missing required argument `", "`", "missing required argument `{argument}`", "argument"),
            ("unknown argument `", "`", "unknown argument `{argument}`", "argument"),
            ("not supported yet (issue #", ")", "not supported yet (issue #{issue})", "issue"),
        ] {
            if let Some(value) = source.strip_prefix(prefix).and_then(|text| text.strip_suffix(suffix)) {
                return self.protocol_format(template, &[(argument, value)]);
            }
        }
        if let Some((command, detail)) = source.strip_prefix("invalid parameters for `").and_then(|text| text.split_once("`: ")) {
            return self.protocol_format(
                "invalid parameters for `{command}`: {detail}",
                &[("command", command), ("detail", &self.bad_parameter_detail(command, detail))],
            );
        }
        if let Some((command, detail)) = source.strip_prefix('`').and_then(|text| text.split_once("` is not available right now: ")) {
            return self.protocol_format(
                "`{command}` is not available right now: {detail}",
                &[("command", command), ("detail", &self.owned_detail(detail))],
            );
        }
        if let Some(detail) = source.strip_prefix("parse error: ") {
            return self.protocol_format("parse error: {detail}", &[("detail", detail)]);
        }
        if self.id() == "pt-BR" {
            return diagnostics::translate(source);
        }
        source.to_string()
    }

    fn owned_detail(self, source: &str) -> String {
        if self.id() == "pt-BR" { diagnostics::translate(source) } else { self.protocol_label(source).to_string() }
    }

    fn bad_parameter_detail(self, command: &str, source: &str) -> String {
        // check_expr, point_param and section's length reader attach these fixed
        // argument names to a DocError::Expr. Plastic keys come from fields() or
        // literal checked/required calls; sheet edge indices are capped at 1000.
        // Keep this inside BadParams: names and paths in other errors are opaque.
        const FIELDS: &[&str] = &[
            "angle",
            "angle2",
            "at",
            "base x",
            "base y",
            "base z",
            "catch_depth",
            "catch_length",
            "center x",
            "center y",
            "center z",
            "clearance",
            "control radius",
            "corner x",
            "corner y",
            "corner z",
            "count",
            "count1",
            "count2",
            "depth",
            "diameter",
            "distance",
            "distance2",
            "draft",
            "factor",
            "factors",
            "fillet",
            "gap",
            "height",
            "hole_depth",
            "hole_diameter",
            "knife_edge",
            "length",
            "major",
            "margin",
            "max_thickness",
            "mid radius",
            "min_draft",
            "min_thickness",
            "minor",
            "nominal_radius",
            "offset",
            "origin x",
            "origin y",
            "origin z",
            "pitch",
            "radius",
            "radius2",
            "reveal_height",
            "revolutions",
            "rib_length",
            "rib_offset",
            "rib_thickness",
            "ribs",
            "section_size",
            "spacing",
            "spacing1",
            "spacing2",
            "start_angle",
            "start_offset",
            "t",
            "taper",
            "thickness",
            "thickness_variation",
            "tolerance",
            "translate",
            "wall",
            "width",
        ];
        const MARKER: &str = ": expression: ";
        let known_command =
            matches!(command, "inspect.section" | "mesh.convert" | "sketch.rectangle.rounded" | "sketch.fillet" | "sketch.circle.two_tangent")
                || ["solid.", "surface.", "plastic.", "sheet.", "construct."].iter().any(|prefix| command.starts_with(prefix));
        if self.id() == "pt-BR"
            && known_command
            && crate::find_command(command).is_some()
            && source.len() <= 64 * 1024
            && let Some((field, detail)) = source.split_once(MARKER).filter(|(_, detail)| !detail.contains(MARKER))
        {
            let edge = field.strip_prefix("edges[").and_then(|path| path.split_once("].")).is_some_and(|(index, field)| {
                matches!(field, "angle" | "height") && index.parse::<usize>().is_ok_and(|n| n < 1000 && n.to_string() == index)
            });
            if FIELDS.contains(&field) || edge {
                return format!("{field}: {}", self.owned_detail(&format!("expression: {detail}")));
            }
        }
        self.owned_detail(source)
    }

    /// Translate catalog presentation while leaving command IDs and behavioral fields intact.
    pub fn command_catalog(self, commands: &mut serde_json::Value) {
        if self.0.id == "en" {
            return;
        }
        let Some(commands) = commands.as_array_mut() else { return };
        for command in commands {
            let Some(object) = command.as_object_mut() else { continue };
            let id = object.get("id").and_then(serde_json::Value::as_str).unwrap_or_default().to_string();
            for field in ["label", "tab", "panel"] {
                if let Some(serde_json::Value::String(source)) = object.get_mut(field) {
                    *source = self.label(source).to_string();
                }
            }
            if let Some(serde_json::Value::String(source)) = object.get_mut("disabled_reason") {
                *source = self.protocol_error(source);
            }
            if let Some(serde_json::Value::String(source)) = object.get_mut("params") {
                *source = self.command_parameters(&id, source).to_string();
            }
        }
    }

    /// Translate only diagnostic fields defined by command-result schemas. Document
    /// names, parameter expressions, comments, enum values and arbitrary nested data
    /// are deliberately not traversed.
    pub fn command_result(self, command: &str, value: &mut serde_json::Value) {
        if self.id() == "en" {
            return;
        }
        if command == "engine.commands" {
            self.command_catalog(value);
            return;
        }
        if command == "file.save_mesh"
            && let Some(serde_json::Value::String(error)) = value.get_mut("slicer_error")
        {
            *error = self.protocol_error(error);
        }
        self.result_diagnostics(value);
        let rows: &[&str] = match command {
            "document.inspect" => &["params", "timeline", "bodies", "sketches"],
            "inspect.measure" => &["bodies"],
            "parameters.list" => &["parameters"],
            _ => &[],
        };
        for field in rows {
            if let Some(rows) = value.get_mut(*field).and_then(serde_json::Value::as_array_mut) {
                for row in rows {
                    self.result_diagnostics(row);
                    if command == "parameters.list"
                        && let Some(serde_json::Value::String(input)) = row.get_mut("input")
                    {
                        *input = self.label(input).to_string();
                    }
                }
            }
        }
        if command == "timeline.inspect"
            && let Some(inputs) = value.get_mut("inputs").and_then(serde_json::Value::as_array_mut)
        {
            for input in inputs {
                if let Some(serde_json::Value::String(label)) = input.get_mut("label") {
                    *label = self.label(label).to_string();
                }
            }
        }
    }

    fn result_diagnostics(self, value: &mut serde_json::Value) {
        if self.id() == "pt-BR"
            && let Some(report) = value
                .get("boolean_diagnostics")
                .and_then(|report| serde_json::from_value::<solidcraft_kernel::BooleanDiagnostics>(report.clone()).ok())
            && let Some(canonical) = report.warning()
            && let Some(localized) = report.warning_with(|text| self.protocol_error(text))
            && let Some(serde_json::Value::String(warning)) = value.get_mut("warning")
        {
            // Model::evaluate appends this exact typed report, either alone or after
            // an existing warning. No arbitrary message is divided on semicolons.
            if *warning == canonical {
                *warning = localized;
            } else if let Some(prefix) = warning.strip_suffix(&format!("; {canonical}")) {
                *warning = format!("{}; {localized}", self.protocol_error(prefix));
            }
        }
        for field in ["error", "warning"] {
            if let Some(serde_json::Value::String(text)) = value.get_mut(field) {
                *text = self.protocol_error(text);
            }
        }
        if let Some(warnings) = value.get_mut("warnings").and_then(serde_json::Value::as_array_mut) {
            for warning in warnings {
                if let serde_json::Value::String(text) = warning {
                    *text = self.protocol_error(text);
                }
            }
        }
        match value.get_mut("errors") {
            Some(serde_json::Value::Object(errors)) => {
                for text in errors.values_mut() {
                    if let serde_json::Value::String(text) = text {
                        *text = self.protocol_error(text);
                    }
                }
            }
            Some(serde_json::Value::Array(errors)) => {
                for error in errors {
                    if let Some(serde_json::Value::String(text)) = error.get_mut("error") {
                        *text = self.protocol_error(text);
                    }
                }
            }
            _ => {}
        }
    }

    /// Stable message ID → translation → English → ID.
    pub fn text(self, id: &str) -> &str {
        self.0.messages().get(id).or_else(|| ENGLISH.messages().get(id)).map_or(id, String::as_str)
    }

    /// Translate source labels from engine/UI tables, preserving unknown labels.
    /// Also accepts stable IDs while table helpers migrate away from source text.
    pub fn label(self, label: &str) -> &str {
        static LABEL_IDS: OnceLock<BTreeMap<&'static str, &'static str>> = OnceLock::new();
        let ids = LABEL_IDS.get_or_init(|| {
            ENGLISH.messages().iter().filter(|(_, text)| !text.contains('{')).map(|(id, text)| (text.as_str(), id.as_str())).collect()
        });
        ids.get(label).map_or_else(|| self.text(label), |id| self.text(id))
    }

    pub fn format(self, id: &str, args: &[(&str, &str)]) -> String {
        interpolate(self.text(id), args)
    }

    /// The default names SolidCraft gives (`Untitled`, `Sketch1`, `Body2`, `Component3`), translated.
    fn default_name(self, text: &str) -> Option<String> {
        if text == "Untitled" {
            return Some(self.text("ui.untitled").to_string());
        }
        for (prefix, id) in [("Sketch", "browser.sketch_name"), ("Body", "browser.body_name"), ("Component", "browser.component_name")] {
            if let Some(number) = text.strip_prefix(prefix)
                && !number.is_empty()
                && number.chars().all(|c| c.is_ascii_digit())
            {
                return Some(self.format(id, &[("number", number)]));
            }
        }
        None
    }

    /// Preserve names that also match UI labels; default-name patterns follow the locale.
    pub fn given_name(self, text: &str) -> String {
        self.default_name(text).unwrap_or_else(|| text.to_string())
    }

    /// Translate generated browser names and labels, preserving custom names otherwise.
    pub fn dynamic(self, text: &str) -> String {
        if let Some(name) = self.default_name(text) {
            return name;
        }
        for (prefix, id, arg) in [("Units: ", "browser.units", "units"), ("Precision: ", "browser.precision", "precision")] {
            if let Some(value) = text.strip_prefix(prefix) {
                return self.format(id, &[(arg, value)]);
            }
        }
        if let Some(count) = text.strip_prefix("Parameters (").and_then(|s| s.strip_suffix(')')) {
            return self.format("browser.parameters", &[("count", count)]);
        }
        if let Some(count) = text.strip_suffix(" degrees of freedom")
            && !count.is_empty()
            && count.chars().all(|c| c.is_ascii_digit())
        {
            return self.format("sketch.degrees_of_freedom", &[("count", count)]);
        }
        self.label(text).to_string()
    }
}

/// One-pass substitution: argument values are literal, even when they contain braces.
/// Missing arguments remain visible rather than crashing or silently removing text.
pub fn interpolate(template: &str, args: &[(&str, &str)]) -> String {
    let mut output = String::with_capacity(template.len());
    let mut remaining = template;
    while let Some((before, after)) = remaining.split_once('{') {
        output.push_str(before);
        let Some((name, rest)) = after.split_once('}') else {
            output.push('{');
            output.push_str(after);
            return output;
        };
        if let Some((_, value)) = args.iter().find(|(key, _)| *key == name) {
            output.push_str(value);
        } else {
            output.push('{');
            output.push_str(name);
            output.push('}');
        }
        remaining = rest;
    }
    output.push_str(remaining);
    output
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use serde_json::json;
    use solidcraft_geom::Vec3;
    use solidcraft_kernel::KernelError;
    #[test]
    fn portuguese_parameter_help_covers_the_complete_registry() {
        // CLI commands and the book use this registry, including metadata, libraries and rendering.
        let commands = crate::command_specs();
        let docs = PT_BR_COMMANDS.messages();
        let missing: Vec<_> = commands
            .iter()
            .filter(|command| !command.params.is_empty() && docs.get(command.id).is_none_or(|text| text.trim().is_empty()))
            .map(|command| command.id)
            .collect();
        assert!(missing.is_empty(), "missing Portuguese parameter help: {missing:?}");
        let orphaned: Vec<_> = docs.keys().filter(|id| !commands.iter().any(|command| command.id == id.as_str())).collect();
        assert!(orphaned.is_empty(), "Portuguese parameter help without a registry command: {orphaned:?}");
    }

    #[test]
    fn project_errors_translate_and_external_kernel_details_stay_literal() {
        let locale = Locale::for_language("pt-BR");
        let expression = solidcraft_doc::expr::eval_with("1 / 0", &|_| Err(solidcraft_doc::DocError::Unknown("unused".into()))).unwrap_err();
        assert_eq!(locale.protocol_error(&expression.to_string()), "expressão: divisão por zero");
        let invalid = solidcraft_kernel::box_solid(Vec3::ZERO, Vec3::ZERO).unwrap_err();
        let source = invalid.to_string();
        let translated = locale.protocol_error(&source);
        assert_ne!(translated, source);
        assert!(translated.starts_with("entrada inválida:"), "{translated}");
        for error in [KernelError::Failed("division by zero".into()), KernelError::Internal("division by zero".into())] {
            assert!(locale.protocol_error(&error.to_string()).ends_with("division by zero"));
        }
        assert_eq!(
            locale.protocol_error(&KernelError::Invalid("profile: division by zero".into()).to_string()),
            "entrada inválida: perfil: division by zero"
        );
    }

    #[test]
    fn cli_workflow_errors_keep_options_commands_and_external_details_literal() {
        let locale = Locale::for_language("pt-BR");
        assert_eq!(locale.protocol_error("render requires --out FILE.png"), "render requer --out FILE.png");
        assert_eq!(locale.protocol_error("unknown render option: --custom {original}"), "opção de render desconhecida: --custom {original}");
        assert_eq!(
            locale.protocol_error("command 7 (solid.extrude): expression: division by zero"),
            "comando 7 (solid.extrude): expressão: divisão por zero"
        );
        let external = KernelError::Failed("division by zero".into());
        let source = format!("command 7 (solid.extrude): {external}");
        let translated = locale.protocol_error(&source);
        assert!(translated.starts_with("comando 7 (solid.extrude): "), "{translated}");
        assert!(translated.ends_with("division by zero"), "{translated}");
    }

    #[test]
    fn new_face_and_untrim_errors_keep_names_and_external_kernel_details_literal() {
        let locale = Locale::for_language("pt-BR");
        assert_eq!(locale.protocol_error("--json-out needs a FILE"), "--json-out precisa de um FILE");
        assert_eq!(
            locale.protocol_error("no face is named `Face {original}` (`model.faces` lists the names)"),
            "nenhuma face se chama `Face {original}` (`model.faces` lista os nomes)"
        );
        assert_eq!(
            locale.protocol_error(&KernelError::Failed("the face's boundary does not lie on its surface".into()).to_string()),
            "a operação falhou: o contorno da face não está sobre sua superfície"
        );
        assert_eq!(
            locale.protocol_error(&KernelError::Failed("untrimmed face: division by zero".into()).to_string()),
            "a operação falhou: face com recorte desfeito: division by zero"
        );
        assert!(!locale.command_parameters("surface.untrim", "source").contains("planejado"));
    }

    #[test]
    fn collapsed_arc_wall_refusal_translates_its_owned_fallback_explanation() {
        for (detail, translated) in [
            ("the blend is larger than the neighbouring faces", "a concordância é maior que as faces vizinhas"),
            ("the blend would consume a neighbouring face", "a concordância consumiria uma face vizinha"),
        ] {
            let detail = format!("{detail} (the operation failed: not supported yet: only straight edges can be blended)");
            let translated = format!("{translated} (a operação falhou: ainda não compatível: somente arestas retas podem receber concordância)");
            let source = KernelError::TooLarge(detail.clone()).to_string();
            assert_eq!(Locale::for_language("pt-BR").protocol_error(&source), format!("a operação falhou: ainda não compatível: {translated}"));
            assert_eq!(Locale::for_language("en").protocol_error(&source), source);
            assert_eq!(
                Locale::for_language("pt-BR").protocol_error(&format!("not supported yet: {detail}")),
                format!("ainda não compatível: {translated}")
            );
        }
    }

    #[test]
    fn bad_parameter_expression_details_keep_argument_paths_and_authored_names() {
        let locale = Locale::for_language("pt-BR");
        let error = crate::Session::default().execute("solid.box", &json!({"length":"1 / 0", "width":10, "height":10})).unwrap_err().to_string();
        assert_eq!(locale.protocol_error(&error), "parâmetros inválidos para `solid.box`: length: expressão: divisão por zero");
        assert_eq!(Locale::for_language("en").protocol_error(&error), error);
        let source = "invalid parameters for `sheet.flange`: edges[2].height: expression: division by zero";
        assert!(locale.protocol_error(source).ends_with("edges[2].height: expressão: divisão por zero"));
        for (command, field) in [("config.rule", "length"), ("solid.box", "My authored name"), ("sheet.flange", "edges[1000].height")] {
            let source = format!("invalid parameters for `{command}`: {field}: expression: division by zero");
            assert!(locale.protocol_error(&source).ends_with(&format!("{field}: expression: division by zero")));
        }
        let name = "x: expression: division by zero";
        let source = format!("invalid parameters for `solid.box`: length: expression: unknown parameter `{name}`");
        assert!(locale.protocol_error(&source).contains(name), "ambiguous delimiters in authored names are literal");
    }

    #[test]
    fn malformed_coordinates_keep_command_and_argument_identifiers() {
        let error = crate::params::req_vec3("construct.point", &json!({"point":[1,2]}), "point").unwrap_err();
        let source = error.to_string();
        let translated = Locale::for_language("pt-BR").protocol_error(&source);
        assert!(translated.contains("parâmetros inválidos"), "{translated}");
        assert!(translated.contains("`construct.point`") && translated.contains("`point`") && translated.contains("[x, y, z]"), "{translated}");
        assert!(!translated.contains("must be a point"), "{translated}");
        assert_eq!(Locale::for_language("en").protocol_error(&source), source);
    }

    #[test]
    fn configuration_rule_names_with_diagnostic_delimiters_stay_intact() {
        let mut session = crate::Session::default();
        session.execute("parameters.change", &json!({"name":"length","expression":"10 mm"})).unwrap();
        session.execute("config.configure", &json!({"params":["length"]})).unwrap();
        let name = "A` references missing parameter `x";
        let error = session
            .execute(
                "config.rule",
                &json!({"rule":{
                    "id":"limit","name":name,"condition":{"type":"always"},
                    "effect":{"type":"drive_parameter","parameter":"missing","expression":"1 mm"}
                }}),
            )
            .unwrap_err();
        let source = error.to_string();
        assert!(source.contains(name), "{source}");
        let translated = Locale::for_language("pt-BR").protocol_error(&source);
        assert!(translated.contains(name), "{translated}");
        assert!(translated.contains("missing"), "{translated}");
    }

    #[test]
    fn command_result_presentation_preserves_typed_payloads_and_authored_fields() {
        use solidcraft_kernel::{BooleanFallback, BooleanStrategy};
        let report = solidcraft_kernel::BooleanDiagnostics {
            events: vec![
                BooleanFallback {
                    strategy: BooleanStrategy::RefinedTolerance,
                    translation_mm: 0.,
                    rotation_rad: 0.,
                    face_shift_mm: 0.,
                    tolerance_mm: 1e-7,
                },
                BooleanFallback {
                    strategy: BooleanStrategy::GrownBoss,
                    translation_mm: 0.,
                    rotation_rad: 0.,
                    face_shift_mm: 2.5e-6,
                    tolerance_mm: 0.,
                },
                BooleanFallback {
                    strategy: BooleanStrategy::ToolNudge,
                    translation_mm: 1.25e-5,
                    rotation_rad: -1e-6,
                    face_shift_mm: 0.,
                    tolerance_mm: 0.,
                },
            ],
            truncated: true,
        };
        let mut row = json!({"name":"division by zero; Body 8", "warning":report.warning(), "boolean_diagnostics":report});
        let original = row.clone();
        Locale::for_language("pt-BR").command_result("timeline.inspect", &mut row);
        let warning = row["warning"].as_str().unwrap();
        assert!(!warning.contains("tolerance") && !warning.contains("tolerância") && !warning.contains("boss grown"), "{warning}");
        assert!(warning.contains("deslocamento da ferramenta 1.250000e-5 mm") && warning.contains("-1.000000e-6 rad"), "{warning}");
        assert!(warning.contains("1024") && !warning.contains("report capped"), "{warning}");
        assert_eq!(row["name"], original["name"]);
        assert_eq!(row["boolean_diagnostics"], original["boolean_diagnostics"]);
        assert_eq!(
            Locale::for_language("pt-BR").protocol_error("boss grown 2.500000e-6 mm into the material around the bore it fills"),
            "ressalto ampliado 2.500000e-6 mm para dentro do material ao redor do furo que preenche"
        );

        // Only the mesh-save schema's diagnostic is prose; names, paths and nested data are not.
        let name = r"division by zero {slicer}: C:\apps\custom";
        let canonical = json!({
            "path": r"C:\models\division by zero {part}.3mf",
            "slicer": name,
            "bodies": ["Body 8: division by zero"],
            "slicer_error": format!("{name} has no program to start"),
            "custom": {"error": "division by zero", "slicer_error": "division by zero"}
        });
        let mut presented = canonical.clone();
        Locale::for_language("pt-BR").command_result("file.save_mesh", &mut presented);
        let mut expected = canonical.clone();
        expected["slicer_error"] = json!(format!("{name} não tem um programa para iniciar"));
        assert_eq!(presented, expected);
        for (language, command) in [("en", "file.save_mesh"), ("pt-BR", "inspect.measure")] {
            let mut unchanged = canonical.clone();
            Locale::for_language(language).command_result(command, &mut unchanged);
            assert_eq!(unchanged, canonical, "{language}: {command}");
        }
    }

    #[test]
    fn mirror_origin_errors_translate_expression_prose_and_preserve_unknown_fields() {
        let locale = Locale::for_language("pt-BR");
        let mut session = crate::Session::default();
        session.execute("solid.box", &json!({"length":10,"width":10,"height":10})).unwrap();
        for (index, field) in ["origin x", "origin y", "origin z"].into_iter().enumerate() {
            let mut origin = [json!(0), json!(0), json!(0)];
            origin[index] = json!("10 mm / 0");
            let source =
                session.execute("solid.mirror", &json!({"bodies":["Body1"],"plane":{"origin":origin,"normal":[1,0,0]}})).unwrap_err().to_string();
            assert_eq!(locale.protocol_error(&source), format!("parâmetros inválidos para `solid.mirror`: {field}: expressão: divisão por zero"));
            assert_eq!(Locale::for_language("en").protocol_error(&source), source);
        }
        for field in ["origin w", "origin[0]", "Driver {literal}: origin x"] {
            let detail = format!("{field}: expression: division by zero");
            let source = format!("invalid parameters for `solid.mirror`: {detail}");
            assert_eq!(locale.protocol_error(&source), format!("parâmetros inválidos para `solid.mirror`: {detail}"));
        }
    }

    #[test]
    fn recomputed_feature_errors_translate_without_changing_expressions_or_names() {
        let mut session = crate::Session::default();
        session.execute("parameters.change", &json!({"name":"length","expression":"10 mm","comment":"division by zero"})).unwrap();
        session.execute("solid.box", &json!({"name":"Body 8: division by zero","length":"length","width":10,"height":10})).unwrap();
        let error = session.execute("parameters.change", &json!({"name":"length","expression":"10 mm / 0"})).unwrap_err();
        assert!(Locale::for_language("pt-BR").protocol_error(&error.to_string()).contains("divisão por zero"));
        // A document can retain a failed expression from an imported or edited file.
        // Evaluate that source to exercise the real feature/inspection error path.
        session.doc_mut().params.iter_mut().find(|parameter| parameter.name == "length").unwrap().expr = "10 mm / 0".into();
        session.refresh();
        let mut design = session.execute("document.inspect", &json!({})).unwrap();
        let source = design.clone();
        Locale::for_language("pt-BR").command_result("document.inspect", &mut design);
        assert!(design["timeline"][0]["error"].as_str().unwrap().contains("divisão por zero"), "{design}");
        assert_eq!(design["timeline"][0]["name"], source["timeline"][0]["name"]);
        for field in ["name", "expression", "comment"] {
            assert_eq!(design["params"][0][field], source["params"][0][field]);
        }
    }

    #[test]
    fn new_surface_errors_translate_owned_prose_and_keep_data_literal() {
        let locale = Locale::for_language("pt-BR");
        let mut session = crate::Session::default();
        let body = r"Body {literal}: C:\models\division by zero";
        session.execute("solid.box", &json!({"body_name":body,"length":10,"width":10,"height":10})).unwrap();
        let value = r"division by zero {literal}: c:\models\boss";
        for (command, detail) in [
            ("surface.ruled", format!("modo desconhecido `{value}` (normal, tangent, angle ou direction)")),
            ("surface.extend", format!("`mode` deve ser natural, tangent ou curvature, não `{value}`")),
        ] {
            let source = session.execute(command, &json!({"body":body,"edges":[[5,0,10]],"distance":2,"mode":value})).unwrap_err().to_string();
            assert_eq!(locale.protocol_error(&source), format!("parâmetros inválidos para `{command}`: {detail}"));
            assert_eq!(Locale::for_language("en").protocol_error(&source), source);
        }
        let solid = solidcraft_kernel::box_solid(Vec3::ZERO, Vec3::new(10.0, 10.0, 10.0)).unwrap();
        let settings = solidcraft_kernel::Ruled {
            distance: 1.0,
            mode: solidcraft_kernel::RuledMode::Direction(Vec3::ZERO),
            flip: false,
            corners: solidcraft_kernel::RuledCorners::Mitre,
            face: None,
        };
        let source = solidcraft_kernel::ruled_surface(&solid, &[Vec3::ZERO], &settings).unwrap_err().to_string();
        assert_eq!(locale.protocol_error(&source), "entrada inválida: a direção não tem comprimento");
        let source = KernelError::TooLarge("extending a face on this kind of surface".into()).to_string();
        assert_eq!(locale.protocol_error(&source), "a operação falhou: ainda não compatível: estender uma face neste tipo de superfície");
        let fi = 37;
        let source =
            KernelError::Failed(format!("not supported yet: tessellation: face {fi}: boundary winds around the surface both ways")).to_string();
        assert_eq!(
            locale.protocol_error(&source),
            format!("a operação falhou: ainda não compatível: tesselação: face {fi}: o contorno envolve a superfície em ambas as direções")
        );
        let point = Vec3::new(1.25, -2.0, 3.5);
        let source = KernelError::Invalid(format!("the strips cross at the corner at {point:?}")).to_string();
        assert_eq!(locale.protocol_error(&source), format!("entrada inválida: as faixas se cruzam no canto em {point:?}"));
        let external = r"division by zero; Vendor {id}: C:\models\Boss";
        let source = KernelError::Failed(format!("extend: a side edge: {external}")).to_string();
        assert_eq!(locale.protocol_error(&source), format!("a operação falhou: estender: uma aresta lateral: {external}"));
        assert_eq!(locale.protocol_error(&KernelError::Failed(external.into()).to_string()), format!("a operação falhou: {external}"));
    }
}
