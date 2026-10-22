use std::process::Command;

#[test]
fn portuguese_help_accepts_flag_and_environment_and_keeps_command_examples() {
    let binary = env!("CARGO_BIN_EXE_solidcraft-cli");
    for use_environment in [false, true] {
        let mut command = Command::new(binary);
        command.env_remove("SOLIDCRAFT_LANGUAGE");
        command.env_remove("SOLVECRAFT_LANGUAGE");
        if use_environment {
            command.env("SOLIDCRAFT_LANGUAGE", "pt-BR").arg("--help");
        } else {
            command.args(["--help", "--language", "pt-BR"]);
        }
        let output = command.output().unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        let help = String::from_utf8(output.stdout).unwrap();
        assert!(help.contains("uso:"), "{help}");
        assert!(help.contains("solidcraft-cli exec <command> [json-params]"), "{help}");
        assert!(help.contains("SOLIDCRAFT_LANGUAGE"), "{help}");
    }
}

#[test]
fn language_option_leaves_model_json_identical() {
    let binary = env!("CARGO_BIN_EXE_solidcraft-cli");
    let run = |language: &str| {
        let output = Command::new(binary)
            .env_remove("SOLIDCRAFT_LANGUAGE")
            .env_remove("SOLVECRAFT_LANGUAGE")
            .args(["--language", language, "exec", "solid.box", r#"{"length":10,"width":20,"height":30}"#])
            .output()
            .unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        let mut result = serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap();
        // Wall-clock timings vary between invocations; model IDs, names and data must not.
        result.as_object_mut().unwrap().remove("ms");
        result
    };
    assert_eq!(run("en"), run("pt-BR"));
    let invalid = Command::new(binary).args(["--language", "invalid", "commands"]).output().unwrap();
    assert!(!invalid.status.success());
    assert!(invalid.stdout.is_empty());
}

#[test]
fn portuguese_crash_list_and_option_values_keep_their_meaning() {
    let binary = env!("CARGO_BIN_EXE_solidcraft-cli");
    let unique = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let dir = std::env::temp_dir().join(format!("solidcraft-language-{}-{unique}", std::process::id()));
    std::fs::create_dir(&dir).unwrap();
    let command = || {
        let mut command = Command::new(binary);
        command
            .current_dir(&dir)
            .env("SOLIDCRAFT_LANGUAGE", "pt-BR")
            .env_remove("SOLVECRAFT_LANGUAGE")
            .env("LOCALAPPDATA", &dir)
            .env("APPDATA", &dir)
            .env("XDG_DATA_HOME", &dir)
            .env("SOLIDCRAFT_CRASH_DIR", dir.join("crashes"));
        command
    };
    let output = command().args(["crashes", "list"]).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let listing = String::from_utf8(output.stdout).unwrap();
    assert!(listing.contains("relatórios de falha em"), "{listing}");
    assert!(listing.contains("(nenhum)"), "{listing}");
    std::fs::write(dir.join("--language"), r#"{"commands":[]}"#).unwrap();
    let output = command().args(["run", "--in", "--language", "--quiet"]).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(output.stdout.is_empty());
    let output = command().args(["run", "--in", "--language", "--quiet", "--json-out", "results.json"]).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(output.stdout.is_empty());
    let results: serde_json::Value = serde_json::from_slice(&std::fs::read(dir.join("results.json")).unwrap()).unwrap();
    assert_eq!(results["body_count"], 0);
    let output = command().args(["bench-agent", "--script-out", "--language", "--scale", "0.03"]).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(output.stdout.is_empty());
    let script: serde_json::Value = serde_json::from_slice(&std::fs::read(dir.join("--language")).unwrap()).unwrap();
    assert!(!script["commands"].as_array().unwrap().is_empty());
    let output = command().args(["crashes", "clear"]).output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap().trim(), "relatórios excluídos: 0");
    std::fs::remove_dir_all(dir).unwrap();
}
