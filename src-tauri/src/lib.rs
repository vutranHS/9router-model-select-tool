use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Tool {
    id: String,
    name: String,
    detail: String,
    path: String,
    found: bool,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ModelRoutes {
    opus: String,
    sonnet: String,
    haiku: String,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Optimizations {
    bypass_permissions: bool,
    effort_level: String,
}
impl Default for Optimizations {
    fn default() -> Self {
        Self {
            bypass_permissions: false,
            effort_level: "high".into(),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApplyRequest {
    tool_ids: Vec<String>,
    routes: ModelRoutes,
    token: String,
    base_url: String,
    compact_window: Option<u64>,
    #[serde(default)]
    default_model_context_window: Option<u64>,
    #[serde(default)]
    codex_context_window: Option<u64>,
    #[serde(default)]
    tool_settings: HashMap<String, Optimizations>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BackupPayload {
    tool_id: String,
    tool_name: String,
    original_path: String,
    original_existed: bool,
    created_at: String,
    content: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct BackupEntry {
    tool_id: String,
    tool_name: String,
    original_path: String,
    backup_path: String,
    created_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ValidationResult {
    valid: bool,
    model_count: usize,
    message: String,
}

fn home_path(relative: &str) -> PathBuf {
    dirs::home_dir().unwrap_or_default().join(relative)
}
fn candidates() -> Vec<(String, String, String, PathBuf)> {
    vec![
        (
            "claude".into(),
            "Claude Code".into(),
            "settings.json".into(),
            home_path(".claude/settings.json"),
        ),
        (
            "codex".into(),
            "Codex CLI".into(),
            "9router profile".into(),
            home_path(".codex/9router.config.toml"),
        ),
        (
            "cursor".into(),
            "Cursor".into(),
            "Settings UI".into(),
            home_path(".cursor"),
        ),
        (
            "cline".into(),
            "Cline".into(),
            "VS Code extension".into(),
            home_path(".vscode/extensions"),
        ),
        (
            "roo".into(),
            "Roo Code".into(),
            "VS Code extension".into(),
            home_path(".vscode/extensions"),
        ),
        (
            "kilo".into(),
            "Kilo Code".into(),
            "VS Code extension".into(),
            home_path(".vscode/extensions"),
        ),
        (
            "vibe".into(),
            "Mistral Vibe CLI".into(),
            "config.toml + .env".into(),
            home_path(".vibe/config.toml"),
        ),
        (
            "continue".into(),
            "Continue".into(),
            "VS Code extension".into(),
            home_path(".continue/config.yaml"),
        ),
        (
            "pi".into(),
            "Pi".into(),
            "models.json".into(),
            home_path(".pi/agent/models.json"),
        ),
        (
            "hermes".into(),
            "Hermes".into(),
            "config.yaml + .env".into(),
            home_path(".hermes/config.yaml"),
        ),
        (
            "copilot-vscode".into(),
            "GitHub Copilot (VS Code)".into(),
            "VS Code extension".into(),
            home_path(".vscode/extensions"),
        ),
    ]
}

fn executable_names(command: &str) -> Vec<String> {
    let mut names = vec![command.into()];
    if cfg!(windows) {
        names.extend([format!("{command}.exe"), format!("{command}.cmd"), format!("{command}.bat")]);
    }
    names
}

fn command_path(command: &str) -> Option<PathBuf> {
    let names = executable_names(command);
    let path_command = std::env::var_os("PATH")
        .into_iter()
        .flat_map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
        .find_map(|folder| names.iter().map(|name| folder.join(name)).find(|path| path.is_file()));
    if path_command.is_some() {
        return path_command;
    }
    let common_roots = [".local/bin", ".bun/bin", ".opencode/bin", ".npm-global/bin"];
    if let Some(path) = common_roots.iter().find_map(|root| names.iter().map(|name| home_path(root).join(name)).find(|path| path.is_file())) {
        return Some(path);
    }
    if let Some(path) = [
        PathBuf::from("/usr/local/bin"),
        PathBuf::from("/opt/homebrew/bin"),
    ]
    .iter()
    .find_map(|root| names.iter().map(|name| root.join(name)).find(|path| path.is_file())) {
        return Some(path);
    }
    let nvm_versions = home_path(".nvm/versions/node");
    fs::read_dir(nvm_versions)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.flatten())
        .find_map(|version| names.iter().map(|name| version.path().join("bin").join(name)).find(|path| path.is_file()))
}

fn command_exists(command: &str) -> bool {
    command_path(command).is_some()
}

fn cursor_path() -> Option<PathBuf> {
    command_path("cursor").or_else(|| {
        [PathBuf::from("/Applications/Cursor.app"), home_path("Applications/Cursor.app")]
            .into_iter()
            .find(|path| path.exists())
    })
}

fn extension_installed(prefixes: &[&str]) -> bool {
    [
        home_path(".vscode/extensions"),
        home_path(".cursor/extensions"),
    ]
    .iter()
    .any(|root| {
        fs::read_dir(root)
            .ok()
            .into_iter()
            .flat_map(|entries| entries.flatten())
            .any(|entry| {
                let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
                prefixes.iter().any(|prefix| name.starts_with(prefix))
            })
    })
}

fn additional_tools() -> Vec<Tool> {
    vec![
        Tool {
            id: "copilot-cli".into(),
            name: "GitHub Copilot CLI".into(),
            detail: "copilot command".into(),
            path: command_path("copilot").map(|path| path.display().to_string()).unwrap_or_else(|| "Not detected".into()),
            found: command_exists("copilot"),
        },
        Tool {
            id: "opencode".into(),
            name: "OpenCode".into(),
            detail: "opencode command".into(),
            path: command_path("opencode").map(|path| path.display().to_string()).unwrap_or_else(|| "Not detected".into()),
            found: command_exists("opencode"),
        },
        Tool {
            id: "openclaw".into(),
            name: "OpenClaw".into(),
            detail: "openclaw command".into(),
            path: command_path("openclaw").map(|path| path.display().to_string()).unwrap_or_else(|| "Not detected".into()),
            found: command_exists("openclaw"),
        },
        Tool {
            id: "factory".into(),
            name: "Factory Droid".into(),
            detail: "droid command".into(),
            path: command_path("droid").map(|path| path.display().to_string()).unwrap_or_else(|| "Not detected".into()),
            found: command_exists("droid"),
        },
    ]
}

#[tauri::command]
fn detect_tools() -> Vec<Tool> {
    let mut tools: Vec<Tool> = candidates()
        .into_iter()
        .map(|(id, name, detail, path)| {
            let found = match id.as_str() {
                "claude" => command_exists("claude"),
                "codex" => command_exists("codex"),
                "cursor" => cursor_path().is_some(),
                "cline" => extension_installed(&["cline.cline", "saoudrizwan.claude-dev"]),
                "roo" => extension_installed(&["rooveterinaryinc.roo-cline"]),
                "kilo" => extension_installed(&["kilocode.kilo-code", "kilo-code.kilo-code"]),
                "continue" => extension_installed(&["continue.continue"]),
                "vibe" => command_exists("vibe"),
                "pi" => command_exists("pi"),
                "hermes" => command_exists("hermes"),
                "copilot-vscode" => extension_installed(&["github.copilot-chat", "github.copilot"]),
                _ => path.exists(),
            };
            let display_path = match id.as_str() {
                "claude" => command_path("claude"),
                "codex" => command_path("codex"),
                "cursor" => cursor_path(),
                "vibe" => command_path("vibe"),
                "pi" => command_path("pi"),
                "hermes" => command_path("hermes"),
                _ => None,
            }
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| path.parent().unwrap_or(Path::new(".")).display().to_string());
            Tool { id, name, detail, path: display_path, found }
        })
        .collect();
    tools.extend(additional_tools());
    tools
}

fn chrono_stamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .to_string()
}

fn backup(id: &str, name: &str, path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let original_existed = path.exists();
    let content = if original_existed {
        fs::read_to_string(path).map_err(|e| e.to_string())?
    } else {
        String::new()
    };
    let created_at = chrono_stamp();
    let backup_path = path.with_file_name(format!(
        "{}.9router-backup-{}.json",
        path.file_name().unwrap_or_default().to_string_lossy(),
        created_at
    ));
    let payload = BackupPayload {
        tool_id: id.into(),
        tool_name: name.into(),
        original_path: path.display().to_string(),
        original_existed,
        created_at,
        content,
    };
    fs::write(
        backup_path,
        serde_json::to_string_pretty(&payload).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}

fn atomic_write(path: &Path, contents: &str) -> Result<(), String> {
    let file_name = path
        .file_name()
        .ok_or("Configuration path has no file name")?
        .to_string_lossy();
    let temporary = path.with_file_name(format!(".{file_name}.9router-writing-{}", chrono_stamp()));
    fs::write(&temporary, contents).map_err(|e| e.to_string())?;
    fs::rename(&temporary, path).map_err(|e| {
        let _ = fs::remove_file(&temporary);
        e.to_string()
    })
}

fn read_json_or_empty(path: &Path, label: &str) -> Result<serde_json::Value, String> {
    if !path.exists() {
        return Ok(serde_json::json!({}));
    }
    let contents = fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&contents)
        .map_err(|_| format!("{label} is not valid JSON; configuration was not changed"))
}

fn read_toml_or_empty(path: &Path, label: &str) -> Result<toml::Value, String> {
    if !path.exists() {
        return Ok(toml::Value::Table(Default::default()));
    }
    let contents = fs::read_to_string(path).map_err(|e| e.to_string())?;
    contents
        .parse::<toml::Value>()
        .map_err(|_| format!("{label} is not valid TOML; configuration was not changed"))
}

fn backups_for(id: &str, name: &str, path: &Path) -> Vec<BackupEntry> {
    let Some(parent) = path.parent() else {
        return vec![];
    };
    let prefix = format!(
        "{}.9router-backup-",
        path.file_name().unwrap_or_default().to_string_lossy()
    );
    let Ok(entries) = fs::read_dir(parent) else {
        return vec![];
    };
    let mut backups: Vec<BackupEntry> = entries
        .flatten()
        .filter_map(|entry| {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if !file_name.starts_with(&prefix) {
                return None;
            }
            let payload: BackupPayload =
                serde_json::from_str(&fs::read_to_string(entry.path()).ok()?).ok()?;
            if payload.tool_id != id
                || payload.tool_name != name
                || payload.original_path != path.display().to_string()
            {
                return None;
            }
            Some(BackupEntry {
                tool_id: payload.tool_id,
                tool_name: payload.tool_name,
                original_path: payload.original_path,
                backup_path: entry.path().display().to_string(),
                created_at: payload.created_at,
            })
        })
        .collect();
    backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    backups
}

#[tauri::command]
fn list_backups() -> Vec<BackupEntry> {
    let mut entries: Vec<BackupEntry> = candidates()
        .into_iter()
        .flat_map(|(id, name, _, path)| backups_for(&id, &name, &path))
        .collect();
    // Keep global Codex snapshots made by versions before profiles became the safe default restorable.
    entries.extend(backups_for(
        "codex",
        "Codex CLI",
        &home_path(".codex/config.toml"),
    ));
    entries.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    entries
}

#[tauri::command]
fn restore_backup(backup_path: String) -> Result<String, String> {
    let backup_path = PathBuf::from(backup_path);
    let payload: BackupPayload = serde_json::from_str(
        &fs::read_to_string(&backup_path)
            .map_err(|_| "Backup file could not be read".to_string())?,
    )
    .map_err(|_| "Backup file is invalid".to_string())?;
    let supported_path = candidates()
        .into_iter()
        .find(|(id, _, _, path)| {
            *id == payload.tool_id && path.display().to_string() == payload.original_path
        })
        .or_else(|| {
            (payload.tool_id == "codex"
                && payload.original_path == home_path(".codex/config.toml").display().to_string())
            .then(|| {
                (
                    "codex".into(),
                    "Codex CLI".into(),
                    "legacy global config".into(),
                    home_path(".codex/config.toml"),
                )
            })
        });
    let Some((_, name, _, original_path)) = supported_path else {
        return Err("This backup does not belong to a supported configuration".into());
    };
    let expected_prefix = format!(
        "{}.9router-backup-",
        original_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
    );
    if backup_path.parent() != original_path.parent()
        || !backup_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .starts_with(&expected_prefix)
    {
        return Err("Backup location was not recognized".into());
    }
    if payload.original_existed {
        atomic_write(&original_path, &payload.content)?;
    } else if original_path.exists() {
        fs::remove_file(&original_path).map_err(|e| e.to_string())?;
    }
    Ok(format!("{name}: restored the original configuration"))
}

fn json_config(
    routes: &ModelRoutes,
    token: &str,
    base_url: &str,
    compact_window: Option<u64>,
    optimizations: &Optimizations,
) -> serde_json::Value {
    let mut env = serde_json::json!({"ANTHROPIC_BASE_URL":base_url,"ANTHROPIC_AUTH_TOKEN":token,"ANTHROPIC_DEFAULT_OPUS_MODEL":routes.opus,"ANTHROPIC_DEFAULT_SONNET_MODEL":routes.sonnet,"ANTHROPIC_DEFAULT_HAIKU_MODEL":routes.haiku});
    if let Some(window) = compact_window {
        env["CLAUDE_CODE_AUTO_COMPACT_WINDOW"] = serde_json::Value::String(window.to_string());
        env["CLAUDE_AUTOCOMPACT_PCT_OVERRIDE"] = serde_json::Value::String("80".into());
    }
    let effort = claude_effort(&optimizations.effort_level);
    let mut settings = serde_json::json!({"env": env,"attribution":{"commit":"","pr":""},"includeGitInstructions":false,"effortLevel":effort,"theme":"dark"});
    if optimizations.bypass_permissions {
        settings["permissions"] = serde_json::json!({"defaultMode":"bypassPermissions"});
        settings["skipDangerousModePermissionPrompt"] = serde_json::Value::Bool(true);
    }
    settings
}

fn claude_effort(value: &str) -> &str {
    match value {
        // Legacy app configurations used auto, but current Claude Code exposes explicit levels.
        "auto" => "medium",
        value => value,
    }
}

fn codex_effort(value: &str) -> &str {
    match value {
        // Codex exposes xhigh as its top documented tier. Preserve a legacy Claude-style max safely.
        "max" => "xhigh",
        "auto" => "medium",
        value => value,
    }
}

fn codex_model(routes: &ModelRoutes) -> String {
    if routes.sonnet.starts_with("cx/") {
        routes.sonnet.clone()
    } else {
        "cx/gpt-5.6-terra".into()
    }
}

fn codex_config(
    routes: &ModelRoutes,
    token: &str,
    base_url: &str,
    optimizations: &Optimizations,
    context_window: Option<u64>,
) -> toml::Value {
    let effort = codex_effort(&optimizations.effort_level);
    let mut values = toml::map::Map::new();
    values.insert("model".into(), toml::Value::String(codex_model(routes)));
    values.insert(
        "model_provider".into(),
        toml::Value::String("9router".into()),
    );
    values.insert(
        "model_reasoning_effort".into(),
        toml::Value::String(effort.into()),
    );
    if let Some(window) = context_window {
        values.insert("model_context_window".into(), toml::Value::Integer(window as i64));
        values.insert("model_auto_compact_token_limit".into(), toml::Value::Integer((window * 80 / 100) as i64));
    }
    values.insert(
        "approval_policy".into(),
        toml::Value::String(
            if optimizations.bypass_permissions {
                "never"
            } else {
                "on-request"
            }
            .into(),
        ),
    );
    values.insert(
        "sandbox_mode".into(),
        toml::Value::String(
            if optimizations.bypass_permissions {
                "danger-full-access"
            } else {
                "workspace-write"
            }
            .into(),
        ),
    );
    let mut provider = toml::map::Map::new();
    provider.insert("name".into(), toml::Value::String("9router".into()));
    provider.insert("base_url".into(), toml::Value::String(base_url.into()));
    provider.insert("wire_api".into(), toml::Value::String("responses".into()));
    provider.insert(
        "experimental_bearer_token".into(),
        toml::Value::String(token.into()),
    );
    let mut providers = toml::map::Map::new();
    providers.insert("9router".into(), toml::Value::Table(provider));
    values.insert("model_providers".into(), toml::Value::Table(providers));
    toml::Value::Table(values)
}

fn default_model(routes: &ModelRoutes) -> String {
    routes.sonnet.clone()
}

fn open_code_config(
    routes: &ModelRoutes,
    token: &str,
    base_url: &str,
    context_window: Option<u64>,
) -> serde_json::Value {
    let model = default_model(routes);
    serde_json::json!({
        "$schema": "https://opencode.ai/config.json",
        "provider": { "9router": {
            "npm": "@ai-sdk/openai-compatible",
            "name": "9router",
            "options": { "baseURL": base_url, "apiKey": token },
            "models": { model.clone(): { "name": model, "limit": { "context": context_window.unwrap_or(272_000) } } }
        }}
    })
}

fn factory_config(routes: &ModelRoutes, token: &str, base_url: &str) -> serde_json::Value {
    let model = default_model(routes);
    serde_json::json!({ "model": model, "displayName": "9router", "baseUrl": base_url, "apiKey": token, "provider": "generic-chat-completion-api" })
}

fn pi_config(
    routes: &ModelRoutes,
    token: &str,
    base_url: &str,
    context_window: Option<u64>,
) -> serde_json::Value {
    let model = default_model(routes);
    serde_json::json!({ "providers": { "9router": {
        "baseUrl": base_url,
        "api": "openai-completions",
        "apiKey": token,
        "models": [{
            "id": model,
            "name": "9router",
            "reasoning": true,
            "input": ["text"],
            "contextWindow": context_window.unwrap_or(272_000)
        }]
    }}})
}

fn write_merged_json(
    id: &str,
    name: &str,
    path: &Path,
    patch: serde_json::Value,
) -> Result<(), String> {
    backup(id, name, path)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let mut existing = read_json_or_empty(path, name)?;
    merge(&mut existing, patch);
    atomic_write(
        path,
        &serde_json::to_string_pretty(&existing).map_err(|e| e.to_string())?,
    )
}

fn write_factory_config(routes: &ModelRoutes, token: &str, base_url: &str) -> Result<(), String> {
    let path = home_path(".factory/settings.json");
    backup("factory", "Factory Droid", &path)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let mut existing = read_json_or_empty(&path, "Factory settings")?;
    let models = existing
        .as_object_mut()
        .ok_or("Factory settings must be a JSON object")?
        .entry("customModels")
        .or_insert(serde_json::json!([]));
    let models = models
        .as_array_mut()
        .ok_or("Factory customModels must be an array")?;
    models.retain(|model| {
        model.get("displayName").and_then(|value| value.as_str()) != Some("9router")
    });
    models.push(factory_config(routes, token, base_url));
    atomic_write(
        &path,
        &serde_json::to_string_pretty(&existing).map_err(|e| e.to_string())?,
    )
}

fn apply_openclaw_config(
    routes: &ModelRoutes,
    token: &str,
    base_url: &str,
    context_window: Option<u64>,
) -> Result<(), String> {
    let path = home_path(".openclaw/openclaw.json");
    backup("openclaw", "OpenClaw", &path)?;
    let model = default_model(routes);
    let provider = serde_json::json!({ "baseUrl": base_url, "apiKey": token, "api": "openai-completions", "models": [{ "id": model, "name": "9router", "reasoning": true, "input": ["text"], "contextWindow": context_window.unwrap_or(272_000) }] });
    let provider_json = serde_json::to_string(&provider).map_err(|e| e.to_string())?;
    let allowlist_json =
        serde_json::json!({ format!("9router/{}", default_model(routes)): {} }).to_string();
    run_command(std::process::Command::new("openclaw").args([
        "config",
        "set",
        "models.providers.9router",
        &provider_json,
        "--strict-json",
        "--merge",
    ]))?;
    run_command(std::process::Command::new("openclaw").args([
        "config",
        "set",
        "agents.defaults.models",
        &allowlist_json,
        "--strict-json",
        "--merge",
    ]))?;
    run_command(std::process::Command::new("openclaw").args([
        "config",
        "set",
        "agents.defaults.model.primary",
        &format!("9router/{}", default_model(routes)),
        "--strict-json",
    ]))?;
    Ok(())
}

fn merge(a: &mut serde_json::Value, b: serde_json::Value) {
    match (a, b) {
        (serde_json::Value::Object(a), serde_json::Value::Object(b)) => {
            for (k, v) in b {
                merge(a.entry(k).or_insert(serde_json::Value::Null), v)
            }
        }
        (a, b) => *a = b,
    }
}
fn merge_toml(a: &mut toml::Value, b: toml::Value) {
    match (a, b) {
        (toml::Value::Table(a), toml::Value::Table(b)) => {
            for (k, v) in b {
                merge_toml(
                    a.entry(k).or_insert(toml::Value::Table(Default::default())),
                    v,
                )
            }
        }
        (a, b) => *a = b,
    }
}

#[tauri::command]
fn validate_api_key(
    base_url: String,
    token: String,
    required_models: Vec<String>,
) -> Result<ValidationResult, String> {
    let base_url = base_url.trim_end_matches('/');
    if base_url.is_empty() || token.trim().is_empty() {
        return Err("Enter both a 9router base URL and API key".into());
    }
    let url = format!("{base_url}/models");
    let output = std::process::Command::new("curl")
        .args([
            "--silent",
            "--show-error",
            "--fail",
            "--max-time",
            "12",
            "--header",
            &format!("Authorization: Bearer {token}"),
            &url,
        ])
        .output()
        .map_err(|e| format!("Could not start validation: {e}"))?;
    if !output.status.success() {
        return Err("9router rejected the API key or the endpoint is unreachable".into());
    }
    let response: serde_json::Value = serde_json::from_slice(&output.stdout).map_err(|_| {
        "The endpoint did not return an OpenAI-compatible models response".to_string()
    })?;
    let models = response
        .get("data")
        .and_then(|data| data.as_array())
        .ok_or("The endpoint returned no models list")?;
    let model_count = models.len();
    if model_count == 0 {
        return Err("The API key is valid but this router exposes no models".into());
    }
    let available: std::collections::HashSet<&str> = models
        .iter()
        .filter_map(|model| model.get("id").and_then(|id| id.as_str()))
        .collect();
    let missing: Vec<String> = required_models
        .into_iter()
        .filter(|model| !available.contains(model.as_str()))
        .collect();
    if !missing.is_empty() {
        return Err(format!(
            "API key is valid, but the selected tool needs unavailable model(s): {}",
            missing.join(", ")
        ));
    }
    Ok(ValidationResult {
        valid: true,
        model_count,
        message: format!("API key verified · {model_count} models available"),
    })
}

#[tauri::command]
fn apply_configuration(request: ApplyRequest) -> Result<Vec<String>, String> {
    let mut changed = vec![];
    for (id, name, _, path) in candidates() {
        if !request.tool_ids.contains(&id) {
            continue;
        }
        let settings = request.tool_settings.get(&id).cloned().unwrap_or_default();
        if id == "claude" {
            backup(&id, &name, &path)?;
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let mut existing = read_json_or_empty(&path, "Claude Code settings")?;
            merge(
                &mut existing,
                json_config(
                    &request.routes,
                    &request.token,
                    &request.base_url,
                    request.compact_window,
                    &settings,
                ),
            );
            atomic_write(
                &path,
                &serde_json::to_string_pretty(&existing).map_err(|e| e.to_string())?,
            )?;
            changed.push(format!(
                "{name}: original configuration snapshot saved and settings merged"
            ));
        } else if id == "codex" {
            backup(&id, &name, &path)?;
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let mut existing = read_toml_or_empty(&path, "Codex 9router profile")?;
            merge_toml(
                &mut existing,
                codex_config(
                    &request.routes,
                    &request.token,
                    &request.base_url,
                    &settings,
                    request.codex_context_window,
                ),
            );
            atomic_write(
                &path,
                &toml::to_string_pretty(&existing).map_err(|e| e.to_string())?,
            )?;
            changed.push(format!(
                "{name}: 9router profile saved; use codex --profile 9router"
            ));
        } else if id != "pi" {
            changed.push(format!("{name}: detected; direct adapter pending"));
        }
    }
    if request.tool_ids.contains(&"opencode".into()) {
        write_merged_json(
            "opencode",
            "OpenCode",
            &home_path(".config/opencode/opencode.json"),
            open_code_config(
                &request.routes,
                &request.token,
                &request.base_url,
                request.default_model_context_window,
            ),
        )?;
        changed.push("OpenCode: 9router provider override merged".into());
    }
    if request.tool_ids.contains(&"factory".into()) {
        write_factory_config(&request.routes, &request.token, &request.base_url)?;
        changed.push("Factory Droid: 9router custom model merged".into());
    }
    if request.tool_ids.contains(&"openclaw".into()) {
        apply_openclaw_config(
            &request.routes,
            &request.token,
            &request.base_url,
            request.default_model_context_window,
        )?;
        changed.push("OpenClaw: 9router provider and default model configured".into());
    }
    if request.tool_ids.contains(&"pi".into()) {
        write_merged_json(
            "pi",
            "Pi",
            &home_path(".pi/agent/models.json"),
            pi_config(
                &request.routes,
                &request.token,
                &request.base_url,
                request.default_model_context_window,
            ),
        )?;
        changed.push("Pi: 9router OpenAI-compatible provider merged".into());
    }
    let mut configured_ids: Vec<String> =
        candidates().into_iter().map(|(id, _, _, _)| id).collect();
    configured_ids.extend([
        "opencode".into(),
        "factory".into(),
        "openclaw".into(),
        "pi".into(),
    ]);
    for tool in detect_tools()
        .into_iter()
        .filter(|tool| request.tool_ids.contains(&tool.id) && !configured_ids.contains(&tool.id))
    {
        changed.push(format!("{}: detected; safe adapter pending", tool.name));
    }
    Ok(changed)
}

fn run_command(command: &mut std::process::Command) -> Result<String, String> {
    let output = command.output().map_err(|e| e.to_string())?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

#[tauri::command]
fn install_optimizer(tool: String, target_tool: String) -> Result<String, String> {
    match (tool.as_str(), target_tool.as_str()) {
        ("rtk", "claude") => {
            if std::process::Command::new("rtk").arg("--version").output().is_err() {
                run_command(std::process::Command::new("sh").args(["-lc", "curl -fsSL https://raw.githubusercontent.com/rtk-ai/rtk/refs/heads/master/install.sh | sh"]))?;
            }
            run_command(std::process::Command::new("rtk").args(["init", "--global", "--auto-patch"]))?;
            Ok("RTK installed and Claude Code hook enabled.".into())
        }
        ("rtk", "cursor") => {
            if std::process::Command::new("rtk").arg("--version").output().is_err() {
                run_command(std::process::Command::new("sh").args(["-lc", "curl -fsSL https://raw.githubusercontent.com/rtk-ai/rtk/refs/heads/master/install.sh | sh"]))?;
            }
            run_command(std::process::Command::new("rtk").args(["init", "--agent", "cursor", "--global", "--auto-patch"]))?;
            Ok("RTK installed and Cursor hook enabled.".into())
        }
        ("rtk", "codex") => {
            if std::process::Command::new("rtk").arg("--version").output().is_err() {
                run_command(std::process::Command::new("sh").args(["-lc", "curl -fsSL https://raw.githubusercontent.com/rtk-ai/rtk/refs/heads/master/install.sh | sh"]))?;
            }
            run_command(std::process::Command::new("rtk").args(["init", "--codex", "--global"]))?;
            Ok("RTK installed with its global hook setup for Codex.".into())
        }
        ("ponytail", "claude") => {
            run_command(std::process::Command::new("claude").args(["plugin", "marketplace", "add", "DietrichGebert/ponytail"]))?;
            run_command(std::process::Command::new("claude").args(["plugin", "install", "ponytail@ponytail"]))?;
            Ok("Ponytail installed. Start a new Claude Code session to activate it.".into())
        }
        ("ponytail", "codex") => {
            run_command(std::process::Command::new("codex").args(["plugin", "marketplace", "add", "DietrichGebert/ponytail"]))?;
            run_command(std::process::Command::new("codex").args(["plugin", "add", "ponytail@ponytail"]))?;
            Ok("Ponytail installed for Codex. Trust its lifecycle hooks in /hooks, then start a new task.".into())
        }
        ("ponytail", "cursor") => Err("Ponytail for Cursor is instruction-based; the app will not inject its instructions automatically.".into()),
        _ => Err("This optimizer does not have a verified installer for the selected tool.".into()),
    }
}

pub fn cli_setup(model: String, token: String) -> Result<Vec<String>, String> {
    let tool_ids = detect_tools()
        .into_iter()
        .filter(|tool| tool.found)
        .map(|tool| tool.id)
        .collect();
    apply_configuration(ApplyRequest {
        tool_ids,
        routes: ModelRoutes {
            opus: model.clone(),
            sonnet: model.clone(),
            haiku: model,
        },
        token,
        base_url: "http://127.0.0.1:20128/v1".into(),
        compact_window: None,
        default_model_context_window: None,
        codex_context_window: None,
        tool_settings: HashMap::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effort_mapping_preserves_equivalent_tiers() {
        assert_eq!(claude_effort("low"), "low");
        assert_eq!(claude_effort("xhigh"), "xhigh");
        assert_eq!(claude_effort("max"), "max");
        assert_eq!(codex_effort("low"), "low");
        assert_eq!(codex_effort("xhigh"), "xhigh");
        assert_eq!(codex_effort("max"), "xhigh");
    }

    #[test]
    fn codex_profile_has_context_and_compaction_threshold() {
        let config = codex_config(
            &ModelRoutes { opus: "cc/claude-opus-4-8".into(), sonnet: "cx/gpt-5.6-terra".into(), haiku: "cx/gpt-5.6-luna".into() },
            "token",
            "https://9router.link/v1",
            &Optimizations { bypass_permissions: false, effort_level: "max".into() },
            Some(272_000),
        );
        let table = config.as_table().unwrap();
        assert_eq!(table["model_reasoning_effort"].as_str(), Some("xhigh"));
        assert_eq!(table["model_context_window"].as_integer(), Some(272_000));
        assert_eq!(table["model_auto_compact_token_limit"].as_integer(), Some(217_600));
    }
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            detect_tools,
            validate_api_key,
            apply_configuration,
            list_backups,
            restore_backup,
            install_optimizer
        ])
        .run(tauri::generate_context!())
        .expect("error while running 9router desktop");
}
