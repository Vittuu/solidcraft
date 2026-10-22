//! JSON-RPC 2.0 framing and the MCP lifecycle / tools / resources methods.

use std::io::{BufRead, Write};

use serde_json::{Value, json};
/// Longest accepted request line, shared with the control server.
use solidcraft_engine::MAX_LINE;
use solidcraft_engine::i18n::Locale;

use solidcraft_engine::io::vfs::{self, PathScope};

use crate::backend::Backend;
use crate::tools::{call_tool, check_arguments, tool_definitions};

/// The newest protocol revision we speak; offered when the client asks for one we don't know.
pub const PROTOCOL_VERSION: &str = "2025-06-18";
pub const SUPPORTED_VERSIONS: &[&str] = &[PROTOCOL_VERSION, "2025-03-26", "2024-11-05"];

const PARSE_ERROR: i64 = -32700;
const INVALID_REQUEST: i64 = -32600;
const METHOD_NOT_FOUND: i64 = -32601;
const INVALID_PARAMS: i64 = -32602;
const INTERNAL_ERROR: i64 = -32603;
const RESOURCE_NOT_FOUND: i64 = -32002;

/// Most messages in one JSON-RPC batch.
const MAX_BATCH: usize = 1000;

const INSTRUCTIONS: &str = "SolidCraft is an independent parametric 3D CAD app. Units are millimetres \
and degrees in expressions (internally radians), Z is up. Model like this: `sketch.create {plane: XY}`, draw \
(`sketch.rectangle.two_point {p0, p1}`, `sketch.circle.center {center, radius}`, `sketch.line {points, closed}`), constrain \
and dimension (`sketch.dimension {entities, value}`; values may name parameters), `sketch.finish`, then features \
(`solid.extrude {distance, operation?: new|join|cut|intersect}`, `solid.revolve`, `solid.fillet {edges: [[x,y,z] point on \
edge], radius}`, `solid.chamfer`, patterns, mirror, primitives). Use `batch` for several steps at once, \
`list_commands` for ids and parameter docs, `describe_command` for a command's schemas, `inspect_design` / `measure` to verify, `body_topology` to find edge points, \
`set_parameter` to drive the design and `screenshot` to look at it.";

pub struct Server {
    backend: Box<dyn Backend>,
    initialized: bool,
    scope: Option<PathScope>,
    locale: Locale,
}

fn response(id: Value, result: Value) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "result": result})
}
fn error(id: Value, code: i64, message: impl Into<String>) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": message.into()}})
}

/// Translate presentation fields only. Names, schema property IDs, enum values and approval
/// annotations keep the same meaning in every language.
fn localize_metadata(value: &mut Value, locale: Locale) {
    match value {
        Value::Object(object) => {
            for (key, field) in object {
                if matches!(key.as_str(), "title" | "description") {
                    if let Value::String(source) = field {
                        *source = locale.protocol_label(source).to_string();
                    }
                } else {
                    localize_metadata(field, locale);
                }
            }
        }
        Value::Array(items) => {
            for item in items {
                localize_metadata(item, locale);
            }
        }
        _ => {}
    }
}

/// Filter against both localized captions and canonical identifiers/source labels.
fn localize_command_reply(reply: &mut Value, locale: Locale, filter: Option<&str>) {
    let Some(content) = reply.get_mut("content").and_then(Value::as_array_mut) else { return };
    for block in content {
        let Some(Value::String(text)) = block.get_mut("text") else { continue };
        let Ok(mut commands) = serde_json::from_str::<Value>(text) else { continue };
        let source = commands.clone();
        locale.command_catalog(&mut commands);
        if let Some(filter) = filter.map(str::to_lowercase).filter(|filter| !filter.is_empty())
            && let (Some(commands), Some(source)) = (commands.as_array_mut(), source.as_array())
        {
            let hit = |command: &Value| {
                ["id", "label", "tab", "panel"]
                    .iter()
                    .any(|field| command.get(*field).and_then(Value::as_str).is_some_and(|value| value.to_lowercase().contains(&filter)))
            };
            let mut index = 0;
            commands.retain(|command| {
                let matched = hit(command) || source.get(index).is_some_and(hit);
                index += 1;
                matched
            });
        }
        *text = commands.to_string();
    }
}

/// Content schemas identify diagnostics; arbitrary strings in returned designs
/// (including names, comments and expressions) are never recursively rewritten.
fn localize_tool_diagnostics(reply: &mut Value, name: &str, args: &Value, locale: Locale) {
    if locale.id() == "en" {
        return;
    }
    let is_error = reply.get("isError").and_then(Value::as_bool) == Some(true);
    let Some(content) = reply.get_mut("content").and_then(Value::as_array_mut) else { return };
    for block in content {
        let Some(Value::String(text)) = block.get_mut("text") else { continue };
        if is_error && name != "batch" {
            *text = locale.protocol_error(text);
            continue;
        }
        let Ok(mut value) = serde_json::from_str::<Value>(text) else {
            if is_error {
                *text = locale.protocol_error(text);
            }
            continue;
        };
        match name {
            "batch" => {
                if let Some(Value::String(error)) = value.get_mut("failed").and_then(|failed| failed.get_mut("error")) {
                    *error = locale.protocol_error(error);
                }
                if let (Some(results), Some(steps)) =
                    (value.get_mut("results").and_then(Value::as_array_mut), args.get("commands").and_then(Value::as_array))
                {
                    for (result, step) in results.iter_mut().zip(steps) {
                        if let Some(command) = step.get("command").and_then(Value::as_str) {
                            locale.command_result(command, result);
                        }
                    }
                }
            }
            "execute" => {
                if let Some(command) = args.get("command").and_then(Value::as_str) {
                    locale.command_result(command, &mut value);
                }
            }
            "inspect_design" => locale.command_result("document.inspect", &mut value),
            "measure" => locale.command_result("inspect.measure", &mut value),
            "screenshot" => {
                if let Some(Value::String(warning)) = value.get_mut("warning")
                    && warning.as_str() == solidcraft_engine::capture::CPU_WARNING
                {
                    *warning = locale.protocol_error(warning);
                }
            }
            "open" => locale.command_result("doc.open", &mut value),
            "set_parameter" => locale.command_result("parameters.change", &mut value),
            "export" => locale.command_result("file.export", &mut value),
            _ => continue,
        }
        *text = value.to_string();
    }
}

fn backend_description(locale: Locale, description: &str) -> String {
    if locale.id() == "pt-BR"
        && let Some((address, port)) = description
            .strip_prefix("connected to the SolidCraft app at ")
            .and_then(|text| {
                text.split_once(
                    " once it runs (the user watches the viewport). It was not running when this server started; until it is (`solidcraft --control ",
                )
            })
            .and_then(|(address, text)| {
                text.strip_suffix("`), tool calls say so").and_then(|port| port.parse::<u16>().ok()).map(|port| (address, port))
            })
    {
        return locale.protocol_format(
            "connected to the SolidCraft app at {addr} once it runs (the user watches the viewport). It was not running \
             when this server started; until it is (`solidcraft --control {port}`), tool calls say so",
            &[("addr", address), ("port", &port.to_string())],
        );
    }
    if locale.id() == "pt-BR"
        && let Some(address) =
            description.strip_prefix("connected to the SolidCraft app at ").and_then(|text| text.strip_suffix(" (the user watches the viewport)"))
    {
        return format!("conectado ao aplicativo SolidCraft em {address} (o usuário acompanha a área de visualização)");
    }
    locale.protocol_label(description).to_string()
}

impl Server {
    fn localized_error(&self, id: Value, code: i64, message: impl Into<String>) -> Value {
        error(id, code, self.locale.protocol_error(&message.into()))
    }

    pub fn new(backend: Box<dyn Backend>) -> Self {
        Self { backend, initialized: false, scope: None, locale: Locale::for_language("en") }
    }
    /// Confine the files tool calls may reach to `scope` (`None`: anywhere). `solidcraft-cli mcp`
    /// always sets one unless started with `--allow-any-path` (docs/mcp.md).
    pub fn with_path_scope(mut self, scope: Option<PathScope>) -> Self {
        self.scope = scope;
        self
    }

    pub fn with_language(mut self, language: &str) -> Self {
        self.locale = Locale::for_language(language);
        self
    }
    pub fn backend(&mut self) -> &mut dyn Backend {
        self.backend.as_mut()
    }
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Serve newline-delimited JSON-RPC until `input` closes. Only protocol messages go to
    /// `output`; diagnostics belong on stderr.
    pub fn serve(&mut self, input: impl BufRead, mut output: impl Write) -> std::io::Result<()> {
        for line in input.lines() {
            let line = match line {
                Ok(l) => l,
                // Not UTF-8: answer with a parse error and keep serving.
                Err(e) if e.kind() == std::io::ErrorKind::InvalidData => {
                    let reply = self.localized_error(Value::Null, PARSE_ERROR, "parse error: input is not UTF-8").to_string();
                    output.write_all(reply.as_bytes())?;
                    output.write_all(b"\n")?;
                    output.flush()?;
                    continue;
                }
                Err(e) => return Err(e),
            };
            if let Some(reply) = self.handle_line(&line) {
                output.write_all(reply.as_bytes())?;
                output.write_all(b"\n")?;
                output.flush()?;
            }
        }
        Ok(())
    }

    /// One line in, at most one line out (notifications get no reply).
    pub fn handle_line(&mut self, line: &str) -> Option<String> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }
        if line.len() > MAX_LINE {
            return Some(self.localized_error(Value::Null, INVALID_REQUEST, "request too large").to_string());
        }
        let reply = match serde_json::from_str::<Value>(line) {
            Ok(Value::Array(batch)) if batch.is_empty() => Some(self.localized_error(Value::Null, INVALID_REQUEST, "empty batch")),
            Ok(Value::Array(batch)) if batch.len() > MAX_BATCH => Some(self.localized_error(Value::Null, INVALID_REQUEST, "batch too large")),
            Ok(Value::Array(batch)) => {
                let replies: Vec<Value> = batch.into_iter().filter_map(|m| self.handle(m)).collect();
                (!replies.is_empty()).then_some(Value::Array(replies))
            }
            Ok(msg) => self.handle(msg),
            Err(e) => Some(self.localized_error(Value::Null, PARSE_ERROR, format!("parse error: {e}"))),
        };
        reply.map(|r| r.to_string())
    }

    pub fn handle(&mut self, msg: Value) -> Option<Value> {
        let Value::Object(o) = &msg else { return Some(self.localized_error(Value::Null, INVALID_REQUEST, "message must be an object")) };
        let id = o.get("id").cloned();
        let Some(method) = o.get("method").and_then(Value::as_str) else {
            // A response to a request of ours (we send none) or junk.
            if o.contains_key("result") || o.contains_key("error") {
                return None;
            }
            return Some(self.localized_error(id.unwrap_or(Value::Null), INVALID_REQUEST, "missing `method`"));
        };
        let params = o.get("params").cloned().unwrap_or(Value::Null);
        let Some(id) = id else {
            if method == "notifications/initialized" {
                self.initialized = true;
            }
            return None;
        };
        Some(match self.request(method, &params) {
            Ok(r) => response(id, r),
            Err((code, m)) => self.localized_error(id, code, m),
        })
    }

    fn request(&mut self, method: &str, params: &Value) -> Result<Value, (i64, String)> {
        match method {
            "initialize" => {
                let asked = params.get("protocolVersion").and_then(Value::as_str).unwrap_or(PROTOCOL_VERSION);
                let version = if SUPPORTED_VERSIONS.contains(&asked) { asked } else { PROTOCOL_VERSION };
                Ok(json!({
                    "protocolVersion": version,
                    "capabilities": {"tools": {"listChanged": false}, "resources": {"listChanged": false}},
                    "serverInfo": {"name": "solidcraft", "title": "SolidCraft", "version": env!("CARGO_PKG_VERSION")},
                    "instructions": format!("{} {}: {}.", self.locale.protocol_label(INSTRUCTIONS), self.locale.protocol_label("Backend"), backend_description(self.locale, &self.backend.describe())),
                }))
            }
            "ping" => Ok(json!({})),
            "tools/list" => {
                let mut tools = json!(tool_definitions());
                localize_metadata(&mut tools, self.locale);
                Ok(json!({"tools": tools}))
            }
            "tools/call" => {
                let name = params.get("name").and_then(Value::as_str).ok_or((INVALID_PARAMS, "missing tool `name`".to_string()))?;
                let mut args = match params.get("arguments") {
                    None | Some(Value::Null) => json!({}),
                    Some(a) => a.clone(),
                };
                check_arguments(name, &args).map_err(|e| (INVALID_PARAMS, e))?;
                let localized_catalog = name == "list_commands" && self.locale.id() != "en";
                let filter = if localized_catalog {
                    args.as_object_mut().and_then(|object| object.remove("filter")).and_then(|filter| filter.as_str().map(str::to_string))
                } else {
                    None
                };
                // The open design's folder counts as it is now (a save-as or open moves it).
                let scope = self.scope.clone().map(|s| s.with_design(self.backend.design_path().as_deref()));
                let backend = self.backend.as_mut();
                let mut reply = vfs::scoped(scope, || call_tool(backend, name, &args));
                if localized_catalog {
                    localize_command_reply(&mut reply, self.locale, filter.as_deref());
                }
                localize_tool_diagnostics(&mut reply, name, &args, self.locale);
                Ok(reply)
            }
            "resources/list" => {
                let mut resources = json!([
                {"uri": "solidcraft://design", "name": "Active design", "description": "document.inspect with measurements", "mimeType": "application/json"},
                {"uri": "solidcraft://commands", "name": "Command catalog", "description": "every command with its parameter docs", "mimeType": "application/json"},
                ]);
                localize_metadata(&mut resources, self.locale);
                if let Some(resources) = resources.as_array_mut() {
                    for resource in resources {
                        if let Some(Value::String(name)) = resource.get_mut("name") {
                            *name = self.locale.protocol_label(name).to_string();
                        }
                    }
                }
                Ok(json!({"resources": resources}))
            }
            "resources/templates/list" => Ok(json!({"resourceTemplates": []})),
            "prompts/list" => Ok(json!({"prompts": []})),
            "resources/read" => {
                let uri = params.get("uri").and_then(Value::as_str).unwrap_or("");
                let mut v = match uri {
                    // The solvecraft:// URIs are what earlier releases listed (compatibility shim,
                    // docs/compatibility.md); clients that saved them still read the resource.
                    "solidcraft://design" | "solvecraft://design" => self.backend.call("document.inspect", json!({"measure": true})),
                    "solidcraft://commands" | "solvecraft://commands" => self.backend.call("engine.commands", json!({})),
                    _ => return Err((RESOURCE_NOT_FOUND, format!("unknown resource `{uri}`"))),
                }
                .map_err(|e| (INTERNAL_ERROR, e))?;
                if matches!(uri, "solidcraft://commands" | "solvecraft://commands") {
                    self.locale.command_catalog(&mut v);
                } else {
                    self.locale.command_result("document.inspect", &mut v);
                }
                Ok(json!({"contents": [{"uri": uri, "mimeType": "application/json", "text": v.to_string()}]}))
            }
            _ => Err((METHOD_NOT_FOUND, format!("method `{method}` not found"))),
        }
    }
}

#[cfg(test)]
mod diagnostic_tests {
    use super::*;

    #[test]
    fn screenshot_localizes_owned_warning_and_preserves_image_and_machine_fields() {
        let image = json!({"type": "image", "data": "iVBORw0KGgo=", "mimeType": "image/png"});
        let info = json!({
            "source": "model", "renderer": "cpu", "path": "Body {name}.png", "width": 640, "height": 480, "bytes": 123,
            "warning": solidcraft_engine::capture::CPU_WARNING,
            "custom": {"warning": solidcraft_engine::capture::CPU_WARNING},
        });
        let mut reply = json!({"content": [image.clone(), {"type": "text", "text": info.to_string()}]});
        localize_tool_diagnostics(&mut reply, "screenshot", &json!({}), Locale::for_language("pt-BR"));
        let actual: Value = serde_json::from_str(reply["content"][1]["text"].as_str().unwrap()).unwrap();
        let mut expected = info.clone();
        expected["warning"] = json!(
            "imagem gerada pelo rasterizador por CPU (nenhum adaptador GPU foi qualificado, ou SOLIDCRAFT_GPU=cpu): cores de análise de superfície, destaques e legendas não são desenhados; `solidcraft-cli gpu` lista os adaptadores"
        );
        assert_eq!(actual, expected);
        assert_eq!(reply["content"][0], image);

        let mut opaque = info;
        opaque["warning"] = json!("unknown command `driver.literal`");
        let mut reply = json!({"content": [{"type": "text", "text": opaque.to_string()}]});
        localize_tool_diagnostics(&mut reply, "screenshot", &json!({}), Locale::for_language("pt-BR"));
        let actual: Value = serde_json::from_str(reply["content"][0]["text"].as_str().unwrap()).unwrap();
        assert_eq!(actual, opaque);
    }

    #[test]
    fn localized_batch_preserves_commands_names_and_opaque_data() {
        let original = json!({
            "ok": false, "completed": 1,
            "failed": {"index": 1, "command": "solid.missing", "error": "unknown command `solid.missing`"},
            "results": [{"name": "nothing to undo", "warning": "unknown command `solid.fake`", "custom": {"error": "unknown command `custom.fake`"}}]
        });
        let mut reply = json!({"isError": true, "content": [{"type": "text", "text": original.to_string()}]});
        let steps = json!({"commands": [{"command": "solid.box"}, {"command": "solid.missing"}]});
        localize_tool_diagnostics(&mut reply, "batch", &steps, Locale::for_language("pt-BR"));
        let value: Value = serde_json::from_str(reply["content"][0]["text"].as_str().unwrap()).unwrap();
        assert_eq!(value["failed"]["error"], "comando desconhecido: `solid.missing`");
        assert_eq!(value["failed"]["command"], original["failed"]["command"]);
        assert_eq!(value["failed"]["index"], 1);
        assert_eq!(value["results"][0]["name"], original["results"][0]["name"]);
        assert_eq!(value["results"][0]["custom"], original["results"][0]["custom"]);
        assert_eq!(value["results"][0]["warning"], "comando desconhecido: `solid.fake`");
        assert_eq!(reply["isError"], true);
    }

    #[test]
    fn inspection_localizes_diagnostics_and_keeps_authored_text() {
        let design = json!({"name": "unknown command `name`", "params": [{"name": "Body 1", "expression": "width / 0", "comment": "unknown command `comment`", "error": "unknown command `solid.fake`"}], "timeline": [{"id": 42, "type": "box", "name": "Boss", "warning": "unknown command `solid.fake`"}]});
        let mut reply = json!({"content": [{"type": "text", "text": design.to_string()}]});
        localize_tool_diagnostics(&mut reply, "inspect_design", &json!({}), Locale::for_language("pt-BR"));
        let value: Value = serde_json::from_str(reply["content"][0]["text"].as_str().unwrap()).unwrap();
        assert_eq!(value["name"], design["name"]);
        assert_eq!(value["params"][0]["expression"], design["params"][0]["expression"]);
        assert_eq!(value["params"][0]["comment"], design["params"][0]["comment"]);
        assert_eq!(value["timeline"][0]["name"], "Boss");
        assert_eq!(value["timeline"][0]["id"], 42);
        assert_eq!(value["params"][0]["error"], "comando desconhecido: `solid.fake`");
        assert_eq!(value["timeline"][0]["warning"], "comando desconhecido: `solid.fake`");
    }
}
