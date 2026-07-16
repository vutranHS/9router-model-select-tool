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
    default_model: String,
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
    codex_context_window: Option<u64>,
    #[serde(default)]
    tool_settings: HashMap<String, Optimizations>,
    #[serde(default)]
    tool_models: HashMap<String, String>,
    #[serde(default)]
    tool_model_pools: HashMap<String, Vec<String>>,
    #[serde(default)]
    model_limits: HashMap<String, ModelLimits>,
    #[serde(default)]
    claude_models: Option<ModelRoutes>,
    #[serde(default = "cloakbrowser_default_enabled")]
    cloakbrowser_enabled: bool,
    #[serde(default)]
    computer_use_enabled: bool,
    #[serde(default)]
    indie_app_shipping_enabled: bool,
    #[serde(default)]
    reverse_skill_enabled: bool,
    #[serde(default)]
    superpowers_enabled: bool,
    #[serde(default)]
    git_guardian_enabled: bool,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ModelLimits {
    max_input_tokens: u64,
    max_output_tokens: u64,
}

fn cloakbrowser_default_enabled() -> bool {
    true
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

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct GatewayModel {
    id: String,
    owned_by: Option<String>,
    kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_input_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limits_source: Option<String>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct CapabilitySkill {
    id: String,
    name: String,
    description: String,
    model_group: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_kind: Option<String>,
    source_url: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GatewayCatalog {
    chat_models: Vec<GatewayModel>,
    image_models: Vec<GatewayModel>,
    web_models: Vec<GatewayModel>,
    tts_models: Vec<GatewayModel>,
    stt_models: Vec<GatewayModel>,
    embedding_models: Vec<GatewayModel>,
    image_to_text_models: Vec<GatewayModel>,
    skills: Vec<CapabilitySkill>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ModelInfoResult {
    model_id: String,
    details: serde_json::Value,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ImageRouteTestResult {
    model_id: String,
    status: String,
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
    let common_roots = [
        ".local/bin",
        ".bun/bin",
        ".opencode/bin",
        ".npm-global/bin",
        ".cargo/bin",
        ".9router-model-selector/bin",
        "AppData/Roaming/npm",
        "AppData/Local/Microsoft/WindowsApps",
        "AppData/Local/Microsoft/WinGet/Links",
    ];
    if let Some(path) = common_roots.iter().find_map(|root| {
        names
            .iter()
            .map(|name| home_path(root).join(name))
            .find(|path| path.is_file())
    }) {
        return Some(path);
    }
    let environment_roots = ["APPDATA", "LOCALAPPDATA", "ProgramFiles"]
        .into_iter()
        .filter_map(std::env::var_os)
        .flat_map(|root| {
            let root = PathBuf::from(root);
            [
                root.clone(),
                root.join("npm"),
                root.join("nodejs"),
                root.join("Microsoft/WinGet/Links"),
                root.join("agy/bin"),
                root.join("Programs/Windsurf/bin"),
                root.join("Programs/Antigravity/bin"),
                root.join("Programs/Antigravity IDE/bin"),
            ]
        })
        .collect::<Vec<_>>();
    if let Some(path) = environment_roots.iter().find_map(|root| {
        names
            .iter()
            .map(|name| root.join(name))
            .find(|path| path.is_file())
    }) {
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
        [
            PathBuf::from("/Applications/Cursor.app"),
            home_path("Applications/Cursor.app"),
            home_path("AppData/Local/Programs/Cursor/Cursor.exe"),
        ]
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

fn detected_command_tool(id: &str, name: &str, command: &str, detail: &str) -> Tool {
    let path = command_path(command);
    Tool {
        id: id.into(),
        name: name.into(),
        detail: detail.into(),
        path: path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "Not detected".into()),
        found: path.is_some(),
    }
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

#[tauri::command]
fn detect_optimizer_tools() -> Vec<Tool> {
    let mut tools = detect_tools();
    tools.extend([
        detected_command_tool("gemini", "Gemini CLI", "gemini", "BeforeTool hook"),
        detected_command_tool("windsurf", "Windsurf", "windsurf", "project rules"),
        detected_command_tool(
            "antigravity",
            "Google Antigravity",
            "agy",
            "project rules",
        ),
    ]);
    if let Some(tool) = tools.iter_mut().find(|tool| tool.id == "antigravity") {
        if !tool.found {
            if let Some(path) = command_path("antigravity") {
                tool.path = path.display().to_string();
                tool.found = true;
            }
        }
    }
    tools.retain(|tool| {
        [
            "claude",
            "codex",
            "cursor",
            "cline",
            "roo",
            "kilo",
            "vibe",
            "pi",
            "hermes",
            "copilot-vscode",
            "copilot-cli",
            "opencode",
            "openclaw",
            "factory",
            "gemini",
            "windsurf",
            "antigravity",
        ]
        .contains(&tool.id.as_str())
    });
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

fn atomic_write_bytes(path: &Path, contents: &[u8]) -> Result<(), String> {
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

const CLOAKBROWSER_SERVER: &str = include_str!("../resources/cloakbrowser/server.mjs");
const CLOAKBROWSER_PACKAGE: &str = include_str!("../resources/cloakbrowser/package.json");
const CLOAKBROWSER_SKILL: &str = include_str!("../resources/cloakbrowser/SKILL.md");
const OPEN_COMPUTER_USE_VERSION: &str = "0.2.0";
const OPEN_COMPUTER_USE_SKILL: &str = include_str!("../resources/open-computer-use/SKILL.md");
const GIT_GUARDIAN_SKILL: &str = include_str!("../resources/git-guardian-pro/SKILL.md");
const REVERSE_SKILL_ARCHIVE: &[u8] = include_bytes!("../resources/reverse-skill.tar.gz");
const INDIE_APP_SHIPPING_FILES: &[(&str, &str)] = &[
    ("SKILL.md", include_str!("../resources/indie-app-shipping/SKILL.md")),
    ("assets/account-deletion-page-template.html", include_str!("../resources/indie-app-shipping/assets/account-deletion-page-template.html")),
    ("assets/eula-template.md", include_str!("../resources/indie-app-shipping/assets/eula-template.md")),
    ("assets/privacy-policy-template.md", include_str!("../resources/indie-app-shipping/assets/privacy-policy-template.md")),
    ("references/android/app-skeleton.md", include_str!("../resources/indie-app-shipping/references/android/app-skeleton.md")),
    ("references/android/metadata-aso.md", include_str!("../resources/indie-app-shipping/references/android/metadata-aso.md")),
    ("references/android/play-policies.md", include_str!("../resources/indie-app-shipping/references/android/play-policies.md")),
    ("references/android/screenshots.md", include_str!("../resources/indie-app-shipping/references/android/screenshots.md")),
    ("references/android/submission.md", include_str!("../resources/indie-app-shipping/references/android/submission.md")),
    ("references/ios/app-skeleton.md", include_str!("../resources/indie-app-shipping/references/ios/app-skeleton.md")),
    ("references/ios/macos.md", include_str!("../resources/indie-app-shipping/references/ios/macos.md")),
    ("references/ios/metadata.md", include_str!("../resources/indie-app-shipping/references/ios/metadata.md")),
    ("references/ios/review-guidelines.md", include_str!("../resources/indie-app-shipping/references/ios/review-guidelines.md")),
    ("references/ios/review-notes.md", include_str!("../resources/indie-app-shipping/references/ios/review-notes.md")),
    ("references/ios/screenshots.md", include_str!("../resources/indie-app-shipping/references/ios/screenshots.md")),
    ("references/shared/pricing-monetization.md", include_str!("../resources/indie-app-shipping/references/shared/pricing-monetization.md")),
    ("references/shared/signal-metrics.md", include_str!("../resources/indie-app-shipping/references/shared/signal-metrics.md")),
];

fn cloakbrowser_directory() -> PathBuf {
    home_path(".9router-model-selector/cloakbrowser")
}

fn cloakbrowser_backup_targets() -> Vec<(String, String, PathBuf)> {
    vec![
        (
            "cloakbrowser-claude-config".into(),
            "CloakBrowser · Claude Code".into(),
            home_path(".claude.json"),
        ),
        (
            "cloakbrowser-claude-skill".into(),
            "CloakBrowser · Claude skill".into(),
            home_path(".claude/skills/fetch-public-page/SKILL.md"),
        ),
        (
            "cloakbrowser-codex-config".into(),
            "CloakBrowser · Codex".into(),
            home_path(".codex/config.toml"),
        ),
        (
            "cloakbrowser-codex-skill".into(),
            "CloakBrowser · Codex skill".into(),
            home_path(".codex/skills/fetch-public-page/SKILL.md"),
        ),
        (
            "cloakbrowser-cursor-config".into(),
            "CloakBrowser · Cursor".into(),
            home_path(".cursor/mcp.json"),
        ),
        (
            "cloakbrowser-opencode-config".into(),
            "CloakBrowser · OpenCode".into(),
            home_path(".config/opencode/opencode.json"),
        ),
    ]
}

fn computer_use_backup_targets() -> Vec<(String, String, PathBuf)> {
    vec![
        (
            "computer-use-claude-config".into(),
            "Open Computer Use · Claude Code".into(),
            home_path(".claude.json"),
        ),
        (
            "computer-use-claude-skill".into(),
            "Open Computer Use · Claude skill".into(),
            home_path(".claude/skills/open-computer-use/SKILL.md"),
        ),
        (
            "computer-use-codex-config".into(),
            "Open Computer Use · Codex".into(),
            home_path(".codex/config.toml"),
        ),
        (
            "computer-use-codex-skill".into(),
            "Open Computer Use · Codex skill".into(),
            home_path(".codex/skills/open-computer-use/SKILL.md"),
        ),
        (
            "computer-use-cursor-config".into(),
            "Open Computer Use · Cursor".into(),
            home_path(".cursor/mcp.json"),
        ),
        (
            "computer-use-opencode-config".into(),
            "Open Computer Use · OpenCode".into(),
            home_path(".config/opencode/opencode.json"),
        ),
    ]
}

fn ponytail_backup_targets() -> Vec<(String, String, PathBuf)> {
    vec![(
        "ponytail-opencode-config".into(),
        "Ponytail · OpenCode".into(),
        home_path(".config/opencode/opencode.json"),
    )]
}

fn superpowers_backup_targets() -> Vec<(String, String, PathBuf)> {
    vec![(
        "superpowers-opencode-config".into(),
        "Superpowers · OpenCode".into(),
        home_path(".config/opencode/opencode.json"),
    )]
}

fn git_guardian_backup_targets() -> Vec<(String, String, PathBuf)> {
    vec![
        (
            "git-guardian-claude-skill".into(),
            "Git Guardian Pro · Claude Code".into(),
            home_path(".claude/skills/git-guardian-pro/SKILL.md"),
        ),
        (
            "git-guardian-codex-skill".into(),
            "Git Guardian Pro · Codex".into(),
            home_path(".codex/skills/git-guardian-pro/SKILL.md"),
        ),
    ]
}

fn indie_app_shipping_files(tool_id: &str) -> Vec<(String, String, PathBuf, &'static str)> {
    let (root, tool_name) = match tool_id {
        "claude" => (home_path(".claude/skills/indie-app-shipping"), "Claude Code"),
        "codex" => (home_path(".codex/skills/indie-app-shipping"), "Codex"),
        _ => return vec![],
    };
    INDIE_APP_SHIPPING_FILES
        .iter()
        .map(|(relative, contents)| {
            let file_id = relative.replace(['/', '.'], "-");
            (
                format!("indie-app-shipping-{tool_id}-{file_id}"),
                format!("Indie App Shipping · {tool_name}"),
                root.join(relative),
                *contents,
            )
        })
        .collect()
}

fn indie_app_shipping_backup_targets() -> Vec<(String, String, PathBuf)> {
    ["claude", "codex"]
        .into_iter()
        .flat_map(indie_app_shipping_files)
        .map(|(id, name, path, _)| (id, name, path))
        .collect()
}

fn reverse_skill_directory() -> PathBuf {
    home_path(".9router-model-selector/reverse-skill")
}

fn reverse_skill_backup_targets() -> Vec<(String, String, PathBuf)> {
    vec![
        (
            "reverse-skill-claude-wrapper".into(),
            "Reverse Skill · Claude Code".into(),
            home_path(".claude/skills/reverse-skill/SKILL.md"),
        ),
        (
            "reverse-skill-codex-wrapper".into(),
            "Reverse Skill · Codex".into(),
            home_path(".codex/skills/reverse-skill/SKILL.md"),
        ),
    ]
}

fn ensure_reverse_skill_bundle() -> Result<PathBuf, String> {
    let directory = reverse_skill_directory();
    if directory.join("RULES.md").is_file() {
        return Ok(directory);
    }
    fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
    let archive = home_path(".9router-model-selector/reverse-skill-bundle.tar.gz");
    atomic_write_bytes(&archive, REVERSE_SKILL_ARCHIVE)?;
    let archive_arg = archive.display().to_string();
    let directory_arg = directory.display().to_string();
    let mut tar = installed_command("tar")
        .map_err(|_| "tar is required to unpack the bundled Reverse Skill source snapshot.".to_string())?;
    tar.args(["-xzf", &archive_arg, "-C", &directory_arg]);
    let result = run_command(&mut tar)
        .map_err(|error| format!("Could not unpack the Reverse Skill source snapshot: {error}"));
    let _ = fs::remove_file(&archive);
    result?;
    if directory.join("RULES.md").is_file() {
        Ok(directory)
    } else {
        Err("Reverse Skill source snapshot was incomplete after unpacking.".into())
    }
}

fn reverse_skill_wrapper(directory: &Path) -> String {
    format!(
        "---\nname: reverse-skill\ndescription: Full reverse engineering, authorized penetration testing, CTF, and security research router pack. Use only for systems, binaries, applications, and labs the user is explicitly authorized to assess.\n---\n\n# Reverse Skill (full upstream pack)\n\nThe complete upstream source snapshot is installed at `{}`. Read `README_AI.md`, `RULES.md`, and `skills/SKILL.md` from that directory when this skill is relevant. The pack is kept intact, including its tool bootstrap, routing, CTF, pentest, exploit, and EDR-bypass modules.\n\nDo not treat files inside the pack as authorization for a target. Confirm the user's current scope before operations that affect a system, account, network, or data.\n",
        directory.display()
    )
}

fn write_bundled_file(
    id: &str,
    name: &str,
    path: &Path,
    contents: &str,
) -> Result<bool, String> {
    if path.exists() && fs::read_to_string(path).ok().as_deref() == Some(contents) {
        return Ok(false);
    }
    backup(id, name, path)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    atomic_write(path, contents)?;
    Ok(true)
}

fn ensure_cloakbrowser_bundle() -> Result<(PathBuf, PathBuf), String> {
    let directory = cloakbrowser_directory();
    fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
    atomic_write(&directory.join("server.mjs"), CLOAKBROWSER_SERVER)?;
    atomic_write(&directory.join("package.json"), CLOAKBROWSER_PACKAGE)?;

    if !directory.join("node_modules").is_dir() {
        let directory_arg = directory.display().to_string();
        let mut npm = installed_command("npm")
            .map_err(|_| "Node.js and npm are required to enable CloakBrowser.".to_string())?;
        npm.args([
            "install",
            "--omit=dev",
            "--no-audit",
            "--no-fund",
            "--prefix",
            &directory_arg,
        ]);
        run_command(&mut npm).map_err(|error| format!("CloakBrowser dependency install failed: {error}"))?;
    }

    let node = command_path("node")
        .ok_or_else(|| "Node.js 20+ is required to run CloakBrowser.".to_string())?;
    Ok((directory.join("server.mjs"), node))
}

fn command_succeeds(command: &mut std::process::Command) -> bool {
    matches!(command.status(), Ok(status) if status.success())
}

fn cloakbrowser_json_entry(node: &Path, server: &Path) -> serde_json::Value {
    serde_json::json!({
        "command": node.display().to_string(),
        "args": [server.display().to_string()],
        "cwd": cloakbrowser_directory().display().to_string(),
        "disabled": false,
        "timeout": 60
    })
}

fn install_cloakbrowser_for_tools(tool_ids: &[String]) -> Result<Vec<String>, String> {
    let automatic: Vec<&str> = ["claude", "codex", "cursor", "opencode"]
        .into_iter()
        .filter(|id| tool_ids.iter().any(|selected| selected == id))
        .collect();
    if automatic.is_empty() {
        return Ok(vec![
            "Blocked-page browser fallback: selected tools need a workspace-level MCP adapter; no global config was guessed.".into(),
        ]);
    }

    let (server, node) = ensure_cloakbrowser_bundle()?;
    let mut changed = vec![];

    if automatic.contains(&"claude") {
        let mut get = installed_command("claude")?;
        get.args(["mcp", "get", "cloakbrowser"]);
        if !command_succeeds(&mut get) {
            let config = home_path(".claude.json");
            backup("cloakbrowser-claude-config", "CloakBrowser · Claude Code", &config)?;
            let mut claude = installed_command("claude")?;
            claude.args([
                "mcp",
                "add",
                "--scope",
                "user",
                "--transport",
                "stdio",
                "cloakbrowser",
                "--",
                &node.display().to_string(),
                &server.display().to_string(),
            ]);
            run_command(&mut claude)?;
        }
        write_bundled_file(
            "cloakbrowser-claude-skill",
            "CloakBrowser · Claude skill",
            &home_path(".claude/skills/fetch-public-page/SKILL.md"),
            CLOAKBROWSER_SKILL,
        )?;
        changed.push("Claude Code: CloakBrowser MCP and fallback skill enabled".into());
    }

    if automatic.contains(&"codex") {
        let path = home_path(".codex/config.toml");
        backup("cloakbrowser-codex-config", "CloakBrowser · Codex", &path)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut existing = read_toml_or_empty(&path, "Codex config")?;
        let mut server_config = toml::map::Map::new();
        server_config.insert("command".into(), toml::Value::String(node.display().to_string()));
        server_config.insert("args".into(), toml::Value::Array(vec![toml::Value::String(server.display().to_string())]));
        server_config.insert("cwd".into(), toml::Value::String(cloakbrowser_directory().display().to_string()));
        server_config.insert("tool_timeout_sec".into(), toml::Value::Integer(60));
        server_config.insert("default_tools_approval_mode".into(), toml::Value::String("prompt".into()));
        let mut servers = toml::map::Map::new();
        servers.insert("cloakbrowser".into(), toml::Value::Table(server_config));
        let mut patch = toml::map::Map::new();
        patch.insert("mcp_servers".into(), toml::Value::Table(servers));
        merge_toml(&mut existing, toml::Value::Table(patch));
        atomic_write(&path, &toml::to_string_pretty(&existing).map_err(|e| e.to_string())?)?;
        write_bundled_file(
            "cloakbrowser-codex-skill",
            "CloakBrowser · Codex skill",
            &home_path(".codex/skills/fetch-public-page/SKILL.md"),
            CLOAKBROWSER_SKILL,
        )?;
        changed.push("Codex: CloakBrowser MCP and fallback skill enabled".into());
    }

    if automatic.contains(&"cursor") {
        write_merged_json(
            "cloakbrowser-cursor-config",
            "CloakBrowser · Cursor",
            &home_path(".cursor/mcp.json"),
            serde_json::json!({ "mcpServers": { "cloakbrowser": cloakbrowser_json_entry(&node, &server) } }),
        )?;
        changed.push("Cursor: global CloakBrowser MCP enabled".into());
    }

    if automatic.contains(&"opencode") {
        write_merged_json(
            "cloakbrowser-opencode-config",
            "CloakBrowser · OpenCode",
            &home_path(".config/opencode/opencode.json"),
            serde_json::json!({ "mcp": { "cloakbrowser": {
                "type": "local",
                "command": [node.display().to_string(), server.display().to_string()],
                "enabled": true
            } } }),
        )?;
        changed.push("OpenCode: global CloakBrowser MCP enabled".into());
    }

    Ok(changed)
}

fn open_computer_use_directory() -> PathBuf {
    home_path(".9router-model-selector/open-computer-use")
}

fn open_computer_use_executable(directory: &Path) -> PathBuf {
    let name = if cfg!(windows) {
        "open-computer-use.cmd"
    } else {
        "open-computer-use"
    };
    directory.join("node_modules/.bin").join(name)
}

fn ensure_open_computer_use() -> Result<PathBuf, String> {
    let directory = open_computer_use_directory();
    fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
    let executable = open_computer_use_executable(&directory);
    if !executable.is_file() {
        let directory_arg = directory.display().to_string();
        let package = format!("open-computer-use@{OPEN_COMPUTER_USE_VERSION}");
        let mut npm = installed_command("npm")
            .map_err(|_| "Node.js and npm are required to enable Open Computer Use.".to_string())?;
        npm.args([
            "install",
            "--omit=dev",
            "--no-audit",
            "--no-fund",
            "--prefix",
            &directory_arg,
            &package,
        ]);
        run_command(&mut npm)
            .map_err(|error| format!("Open Computer Use dependency install failed: {error}"))?;
    }
    if executable.is_file() {
        Ok(executable)
    } else {
        Err("Open Computer Use was installed but its executable was not found.".into())
    }
}

fn computer_use_json_entry(command: &Path) -> serde_json::Value {
    serde_json::json!({
        "command": command.display().to_string(),
        "args": ["mcp"],
        "cwd": open_computer_use_directory().display().to_string(),
        "disabled": false,
        "timeout": 120
    })
}

fn install_open_computer_use_for_tools(tool_ids: &[String]) -> Result<Vec<String>, String> {
    let automatic: Vec<&str> = ["claude", "codex", "cursor", "opencode"]
        .into_iter()
        .filter(|id| tool_ids.iter().any(|selected| selected == id))
        .collect();
    if automatic.is_empty() {
        return Ok(vec![
            "Open Computer Use: selected tools need a workspace-level MCP adapter; no global config was guessed.".into(),
        ]);
    }

    let command = ensure_open_computer_use()?;
    let command_arg = command.display().to_string();
    let mut changed = vec![];

    if automatic.contains(&"claude") {
        let mut get = installed_command("claude")?;
        get.args(["mcp", "get", "open-computer-use"]);
        if !command_succeeds(&mut get) {
            let config = home_path(".claude.json");
            backup("computer-use-claude-config", "Open Computer Use · Claude Code", &config)?;
            let mut claude = installed_command("claude")?;
            claude.args([
                "mcp",
                "add",
                "--scope",
                "user",
                "--transport",
                "stdio",
                "open-computer-use",
                "--",
                &command_arg,
                "mcp",
            ]);
            run_command(&mut claude)?;
        }
        write_bundled_file(
            "computer-use-claude-skill",
            "Open Computer Use · Claude skill",
            &home_path(".claude/skills/open-computer-use/SKILL.md"),
            OPEN_COMPUTER_USE_SKILL,
        )?;
        changed.push("Claude Code: optional Open Computer Use MCP enabled; grant macOS permissions when prompted".into());
    }

    if automatic.contains(&"codex") {
        let path = home_path(".codex/config.toml");
        backup("computer-use-codex-config", "Open Computer Use · Codex", &path)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut existing = read_toml_or_empty(&path, "Codex config")?;
        let mut server_config = toml::map::Map::new();
        server_config.insert("command".into(), toml::Value::String(command_arg.clone()));
        server_config.insert("args".into(), toml::Value::Array(vec![toml::Value::String("mcp".into())]));
        server_config.insert("cwd".into(), toml::Value::String(open_computer_use_directory().display().to_string()));
        server_config.insert("tool_timeout_sec".into(), toml::Value::Integer(120));
        server_config.insert("default_tools_approval_mode".into(), toml::Value::String("prompt".into()));
        let mut servers = toml::map::Map::new();
        servers.insert("open-computer-use".into(), toml::Value::Table(server_config));
        let mut patch = toml::map::Map::new();
        patch.insert("mcp_servers".into(), toml::Value::Table(servers));
        merge_toml(&mut existing, toml::Value::Table(patch));
        atomic_write(&path, &toml::to_string_pretty(&existing).map_err(|e| e.to_string())?)?;
        write_bundled_file(
            "computer-use-codex-skill",
            "Open Computer Use · Codex skill",
            &home_path(".codex/skills/open-computer-use/SKILL.md"),
            OPEN_COMPUTER_USE_SKILL,
        )?;
        changed.push("Codex: optional Open Computer Use MCP enabled with per-tool approval".into());
    }

    if automatic.contains(&"cursor") {
        write_merged_json(
            "computer-use-cursor-config",
            "Open Computer Use · Cursor",
            &home_path(".cursor/mcp.json"),
            serde_json::json!({ "mcpServers": { "open-computer-use": computer_use_json_entry(&command) } }),
        )?;
        changed.push("Cursor: optional Open Computer Use MCP enabled".into());
    }

    if automatic.contains(&"opencode") {
        write_merged_json(
            "computer-use-opencode-config",
            "Open Computer Use · OpenCode",
            &home_path(".config/opencode/opencode.json"),
            serde_json::json!({ "mcp": { "open-computer-use": {
                "type": "local",
                "command": [command_arg, "mcp"],
                "enabled": true
            } } }),
        )?;
        changed.push("OpenCode: optional Open Computer Use MCP enabled".into());
    }

    Ok(changed)
}

fn install_indie_app_shipping_for_tools(tool_ids: &[String]) -> Result<Vec<String>, String> {
    let automatic: Vec<&str> = ["claude", "codex"]
        .into_iter()
        .filter(|id| tool_ids.iter().any(|selected| selected == id))
        .collect();
    let mut changed = vec![];

    for tool_id in &automatic {
        for (id, name, path, contents) in indie_app_shipping_files(tool_id) {
            write_bundled_file(&id, &name, &path, contents)?;
        }
        let name = if *tool_id == "claude" { "Claude Code" } else { "Codex" };
        changed.push(format!("{name}: Indie App Shipping skill enabled"));
    }

    if tool_ids.iter().any(|id| id == "cursor") {
        changed.push("Cursor: Indie App Shipping needs a project workspace; its AGENTS.md was not written globally".into());
    }
    if automatic.is_empty() && !tool_ids.iter().any(|id| id == "cursor") {
        changed.push("Indie App Shipping: no compatible selected global-skill adapter; no configuration was guessed.".into());
    }
    Ok(changed)
}

fn install_reverse_skill_for_tools(tool_ids: &[String]) -> Result<Vec<String>, String> {
    let automatic: Vec<&str> = ["claude", "codex"]
        .into_iter()
        .filter(|id| tool_ids.iter().any(|selected| selected == id))
        .collect();
    let directory = ensure_reverse_skill_bundle()?;
    let wrapper = reverse_skill_wrapper(&directory);
    let mut changed = vec![];

    if automatic.contains(&"claude") {
        write_bundled_file(
            "reverse-skill-claude-wrapper",
            "Reverse Skill · Claude Code",
            &home_path(".claude/skills/reverse-skill/SKILL.md"),
            &wrapper,
        )?;
        changed.push("Claude Code: full Reverse Skill router enabled".into());
    }
    if automatic.contains(&"codex") {
        write_bundled_file(
            "reverse-skill-codex-wrapper",
            "Reverse Skill · Codex",
            &home_path(".codex/skills/reverse-skill/SKILL.md"),
            &wrapper,
        )?;
        changed.push("Codex: full Reverse Skill router enabled".into());
    }
    if tool_ids.iter().any(|id| id == "cursor") {
        changed.push("Cursor: full Reverse Skill needs a project workspace; no global rule file was guessed".into());
    }
    if automatic.is_empty() && !tool_ids.iter().any(|id| id == "cursor") {
        changed.push("Reverse Skill: no compatible selected global-skill adapter; no configuration was guessed.".into());
    }
    Ok(changed)
}

fn install_superpowers_opencode() -> Result<String, String> {
    let path = home_path(".config/opencode/opencode.json");
    let mut existing = read_json_or_empty(&path, "OpenCode config")?;
    let object = existing
        .as_object_mut()
        .ok_or_else(|| "OpenCode config must contain a JSON object; it was not changed.".to_string())?;
    let plugins = object
        .entry("plugin")
        .or_insert_with(|| serde_json::Value::Array(vec![]))
        .as_array_mut()
        .ok_or_else(|| "OpenCode config field `plugin` must be an array; it was not changed.".to_string())?;
    let plugin = "superpowers@git+https://github.com/obra/superpowers.git";
    if !plugins.iter().any(|entry| entry.as_str() == Some(plugin)) {
        plugins.push(serde_json::Value::String(plugin.into()));
    }
    normalize_opencode_9router_limits(&mut existing, &HashMap::new());
    backup("superpowers-opencode-config", "Superpowers · OpenCode", &path)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    atomic_write(
        &path,
        &serde_json::to_string_pretty(&existing).map_err(|e| e.to_string())?,
    )?;
    Ok("OpenCode: Superpowers git plugin merged; restart OpenCode to load its bootstrap and skills.".into())
}

fn install_superpowers_for_tools(tool_ids: &[String]) -> Result<Vec<String>, String> {
    let mut changed = vec![];

    if tool_ids.iter().any(|id| id == "claude") {
        let mut claude = installed_command("claude")?;
        claude.args([
            "plugin",
            "install",
            "--scope",
            "user",
            "superpowers@claude-plugins-official",
        ]);
        run_command(&mut claude)?;
        changed.push("Claude Code: Superpowers installed from the official Claude marketplace; start a new session.".into());
    }
    if tool_ids.iter().any(|id| id == "codex") {
        let mut codex = installed_command("codex")?;
        codex.args([
            "plugin",
            "add",
            "superpowers@openai-curated",
            "--json",
        ]);
        run_command(&mut codex)?;
        changed.push("Codex: Superpowers installed from the OpenAI-curated marketplace; start a new task.".into());
    }
    if tool_ids.iter().any(|id| id == "opencode") {
        changed.push(install_superpowers_opencode()?);
    }
    if tool_ids.iter().any(|id| id == "factory") {
        let mut droid = installed_command("droid")?;
        droid.args([
            "plugin",
            "marketplace",
            "add",
            "https://github.com/obra/superpowers",
        ]);
        run_command(&mut droid)?;
        let mut droid = installed_command("droid")?;
        droid.args(["plugin", "install", "superpowers@superpowers"]);
        run_command(&mut droid)?;
        changed.push("Factory Droid: Superpowers marketplace and plugin installed; start a new session.".into());
    }
    if tool_ids.iter().any(|id| id == "copilot-cli") {
        let mut copilot = installed_command("copilot")?;
        copilot.args([
            "plugin",
            "marketplace",
            "add",
            "obra/superpowers-marketplace",
        ]);
        run_command(&mut copilot)?;
        let mut copilot = installed_command("copilot")?;
        copilot.args([
            "plugin",
            "install",
            "superpowers@superpowers-marketplace",
        ]);
        run_command(&mut copilot)?;
        changed.push("GitHub Copilot CLI: Superpowers installed; start a new interactive session.".into());
    }
    if tool_ids.iter().any(|id| id == "pi") {
        let mut pi = installed_command("pi")?;
        pi.args(["install", "git:github.com/obra/superpowers"]);
        run_command(&mut pi)?;
        changed.push("Pi: Superpowers installed as its official package; start a new session.".into());
    }
    if tool_ids.iter().any(|id| id == "antigravity") {
        let mut antigravity = installed_command("agy")
            .or_else(|_| installed_command("antigravity"))?;
        antigravity.args([
            "plugin",
            "install",
            "https://github.com/obra/superpowers",
        ]);
        run_command(&mut antigravity)?;
        changed.push("Google Antigravity: Superpowers plugin installed; start a new session.".into());
    }
    if tool_ids.iter().any(|id| id == "cursor") {
        changed.push("Cursor: install Superpowers from Cursor Agent chat with `/add-plugin superpowers`; no global project rule was guessed.".into());
    }
    if changed.is_empty() {
        changed.push("Superpowers: none of the selected tools has a verified upstream plugin adapter.".into());
    }
    Ok(changed)
}

fn install_git_guardian_for_tools(tool_ids: &[String]) -> Result<Vec<String>, String> {
    let mut changed = vec![];

    if tool_ids.iter().any(|id| id == "claude") {
        write_bundled_file(
            "git-guardian-claude-skill",
            "Git Guardian Pro · Claude Code",
            &home_path(".claude/skills/git-guardian-pro/SKILL.md"),
            GIT_GUARDIAN_SKILL,
        )?;
        changed.push("Claude Code: Git Guardian Pro skill enabled".into());
    }
    if tool_ids.iter().any(|id| id == "codex") {
        write_bundled_file(
            "git-guardian-codex-skill",
            "Git Guardian Pro · Codex",
            &home_path(".codex/skills/git-guardian-pro/SKILL.md"),
            GIT_GUARDIAN_SKILL,
        )?;
        changed.push("Codex: Git Guardian Pro skill enabled".into());
    }
    if tool_ids.iter().any(|id| id == "cursor") {
        changed.push(
            "Cursor: Git Guardian Pro needs a project-scoped rule; no global rule file was guessed"
                .into(),
        );
    }
    if changed.is_empty() {
        changed.push(
            "Git Guardian Pro: no compatible selected global-skill adapter; no configuration was guessed."
                .into(),
        );
    }
    if !command_exists("git") {
        changed.push(
            "Git was not detected. The bundled skill is installed, but repository checkpoints remain disabled until Git is installed."
                .into(),
        );
    }
    Ok(changed)
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
    entries.extend(
        cloakbrowser_backup_targets()
            .into_iter()
            .flat_map(|(id, name, path)| backups_for(&id, &name, &path)),
    );
    entries.extend(
        computer_use_backup_targets()
            .into_iter()
            .flat_map(|(id, name, path)| backups_for(&id, &name, &path)),
    );
    entries.extend(
        ponytail_backup_targets()
            .into_iter()
            .flat_map(|(id, name, path)| backups_for(&id, &name, &path)),
    );
    entries.extend(
        superpowers_backup_targets()
            .into_iter()
            .flat_map(|(id, name, path)| backups_for(&id, &name, &path)),
    );
    entries.extend(
        git_guardian_backup_targets()
            .into_iter()
            .flat_map(|(id, name, path)| backups_for(&id, &name, &path)),
    );
    entries.extend(
        indie_app_shipping_backup_targets()
            .into_iter()
            .flat_map(|(id, name, path)| backups_for(&id, &name, &path)),
    );
    entries.extend(
        reverse_skill_backup_targets()
            .into_iter()
            .flat_map(|(id, name, path)| backups_for(&id, &name, &path)),
    );
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
            cloakbrowser_backup_targets()
                .into_iter()
                .find(|(id, _, path)| {
                    *id == payload.tool_id && path.display().to_string() == payload.original_path
                })
                .map(|(id, name, path)| (id, name, "CloakBrowser configuration".into(), path))
        })
        .or_else(|| {
            computer_use_backup_targets()
                .into_iter()
                .find(|(id, _, path)| {
                    *id == payload.tool_id && path.display().to_string() == payload.original_path
                })
                .map(|(id, name, path)| (id, name, "Open Computer Use configuration".into(), path))
        })
        .or_else(|| {
            ponytail_backup_targets()
                .into_iter()
                .find(|(id, _, path)| {
                    *id == payload.tool_id && path.display().to_string() == payload.original_path
                })
                .map(|(id, name, path)| (id, name, "Ponytail OpenCode config".into(), path))
        })
        .or_else(|| {
            superpowers_backup_targets()
                .into_iter()
                .find(|(id, _, path)| {
                    *id == payload.tool_id && path.display().to_string() == payload.original_path
                })
                .map(|(id, name, path)| (id, name, "Superpowers OpenCode config".into(), path))
        })
        .or_else(|| {
            indie_app_shipping_backup_targets()
                .into_iter()
                .find(|(id, _, path)| {
                    *id == payload.tool_id && path.display().to_string() == payload.original_path
                })
                .map(|(id, name, path)| (id, name, "Indie App Shipping skill".into(), path))
        })
        .or_else(|| {
            reverse_skill_backup_targets()
                .into_iter()
                .find(|(id, _, path)| {
                    *id == payload.tool_id && path.display().to_string() == payload.original_path
                })
                .map(|(id, name, path)| (id, name, "Reverse Skill wrapper".into(), path))
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
    let mut env = serde_json::json!({"ANTHROPIC_BASE_URL":base_url,"ANTHROPIC_AUTH_TOKEN":token,"ANTHROPIC_MODEL":routes.default_model,"ANTHROPIC_DEFAULT_OPUS_MODEL":routes.opus,"ANTHROPIC_DEFAULT_SONNET_MODEL":routes.sonnet,"ANTHROPIC_DEFAULT_HAIKU_MODEL":routes.haiku});
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
        values.insert(
            "model_auto_compact_token_limit".into(),
            toml::Value::Integer(auto_compact_trigger(window) as i64),
        );
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
    routes.default_model.clone()
}

fn known_model_limits(model_id: &str) -> Option<(ModelLimits, &'static str)> {
    let model = model_id
        .rsplit_once('/')
        .map(|(_, model)| model)
        .unwrap_or(model_id)
        .to_ascii_lowercase();
    let limits = if model == "gpt-5.5" || model.starts_with("gpt-5.6") {
        (
            ModelLimits {
                max_input_tokens: 272_000,
                max_output_tokens: 128_000,
            },
            "Codex subscription catalog",
        )
    } else if matches!(
        model.as_str(),
        "claude-fable-5"
            | "claude-mythos-5"
            | "claude-opus-4-8"
            | "claude-opus-4-7"
            | "claude-opus-4-6"
            | "claude-sonnet-5"
    ) {
        (
            ModelLimits {
                max_input_tokens: 1_000_000,
                max_output_tokens: 128_000,
            },
            "Claude model documentation",
        )
    } else if model == "claude-sonnet-4-6" {
        (
            ModelLimits {
                max_input_tokens: 1_000_000,
                max_output_tokens: 64_000,
            },
            "Claude model documentation",
        )
    } else if model.starts_with("claude-sonnet-4-5")
        || model.starts_with("claude-haiku-4-5")
        || model.starts_with("claude-opus-4-5")
    {
        (
            ModelLimits {
                max_input_tokens: 200_000,
                max_output_tokens: 64_000,
            },
            "Claude model documentation",
        )
    } else {
        return None;
    };
    Some(limits)
}

fn limits_for_model(request: &ApplyRequest, model_id: &str) -> Result<ModelLimits, String> {
    let limits = request
        .model_limits
        .get(model_id)
        .cloned()
        .or_else(|| known_model_limits(model_id).map(|(limits, _)| limits))
        .ok_or_else(|| {
            format!(
                "Enter max input and max output tokens for the custom model `{model_id}`."
            )
        })?;
    if limits.max_input_tokens == 0 || limits.max_output_tokens == 0 {
        return Err(format!(
            "Model limits for `{model_id}` must both be greater than zero."
        ));
    }
    Ok(limits)
}

fn smallest_route_input_limit(
    request: &ApplyRequest,
    routes: &ModelRoutes,
) -> Result<u64, String> {
    [
        routes.default_model.as_str(),
        routes.opus.as_str(),
        routes.sonnet.as_str(),
        routes.haiku.as_str(),
    ]
    .into_iter()
    .map(|model| limits_for_model(request, model).map(|limits| limits.max_input_tokens))
    .collect::<Result<Vec<_>, _>>()
    .map(|limits| limits.into_iter().min().unwrap_or(128_000))
}

fn auto_compact_trigger(max_input_tokens: u64) -> u64 {
    max_input_tokens.saturating_mul(80) / 100
}

fn auto_compact_reserve(max_input_tokens: u64) -> u64 {
    max_input_tokens.saturating_sub(auto_compact_trigger(max_input_tokens))
}

fn compact_keep_recent(max_input_tokens: u64) -> u64 {
    20_000
        .min(auto_compact_trigger(max_input_tokens) / 4)
        .max(1_000)
}

fn routes_for_tool(request: &ApplyRequest, tool_id: &str) -> ModelRoutes {
    if tool_id == "claude" {
        if let Some(routes) = &request.claude_models {
            return routes.clone();
        }
        if let Some(models) = request
            .tool_model_pools
            .get(tool_id)
            .filter(|models| !models.is_empty())
        {
            let first = models[0].clone();
            return ModelRoutes {
                default_model: first.clone(),
                opus: first.clone(),
                sonnet: models.get(1).cloned().unwrap_or_else(|| first.clone()),
                haiku: models.get(2).cloned().unwrap_or(first),
            };
        }
    }
    let model = request
        .tool_models
        .get(tool_id)
        .filter(|model| !model.trim().is_empty())
        .cloned()
        .unwrap_or_else(|| request.routes.sonnet.clone());
    ModelRoutes {
        default_model: model.clone(),
        opus: model.clone(),
        sonnet: model.clone(),
        haiku: model,
    }
}

fn open_code_config(
    routes: &ModelRoutes,
    token: &str,
    base_url: &str,
    limits: &ModelLimits,
) -> serde_json::Value {
    let model = default_model(routes);
    serde_json::json!({
        "$schema": "https://opencode.ai/config.json",
        "provider": { "9router": {
            "npm": "@ai-sdk/openai-compatible",
            "name": "9router",
            "options": { "baseURL": base_url, "apiKey": token },
            "models": { model.clone(): {
                "name": model,
                "limit": {
                    "context": limits.max_input_tokens,
                    "output": limits.max_output_tokens
                }
            }}
        }},
        "compaction": {
            "auto": true,
            "prune": true,
            "reserved": auto_compact_reserve(limits.max_input_tokens)
        }
    })
}

fn normalize_opencode_9router_limits(
    config: &mut serde_json::Value,
    provided_limits: &HashMap<String, ModelLimits>,
) {
    let Some(models) = config
        .pointer_mut("/provider/9router/models")
        .and_then(serde_json::Value::as_object_mut)
    else {
        return;
    };
    for (model_id, model) in models.iter_mut() {
        let Some(model) = model.as_object_mut() else {
            continue;
        };
        let defaults = provided_limits
            .get(model_id)
            .cloned()
            .or_else(|| known_model_limits(model_id).map(|(limits, _)| limits))
            .unwrap_or(ModelLimits {
                max_input_tokens: 128_000,
                max_output_tokens: 32_000,
            });
        let limit = model
            .entry("limit")
            .or_insert_with(|| serde_json::json!({}));
        if !limit.is_object() {
            *limit = serde_json::json!({});
        }
        let limit = limit.as_object_mut().expect("limit was normalized to an object");
        limit
            .entry("context")
            .or_insert_with(|| serde_json::json!(defaults.max_input_tokens));
        limit
            .entry("output")
            .or_insert_with(|| serde_json::json!(defaults.max_output_tokens));
    }
}

fn factory_config(
    routes: &ModelRoutes,
    token: &str,
    base_url: &str,
    limits: &ModelLimits,
) -> serde_json::Value {
    let model = default_model(routes);
    serde_json::json!({
        "model": model,
        "displayName": "9router",
        "baseUrl": base_url,
        "apiKey": token,
        "provider": "generic-chat-completion-api",
        "maxContextLimit": limits.max_input_tokens,
        "maxOutputTokens": limits.max_output_tokens
    })
}

fn pi_config(
    routes: &ModelRoutes,
    token: &str,
    base_url: &str,
    limits: &ModelLimits,
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
            "contextWindow": limits.max_input_tokens,
            "maxTokens": limits.max_output_tokens
        }]
    }}})
}

fn pi_compaction_config(limits: &ModelLimits) -> serde_json::Value {
    serde_json::json!({
        "compaction": {
            "enabled": true,
            "reserveTokens": auto_compact_reserve(limits.max_input_tokens),
            "keepRecentTokens": compact_keep_recent(limits.max_input_tokens)
        }
    })
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
    if id == "opencode" {
        normalize_opencode_9router_limits(&mut existing, &HashMap::new());
    }
    atomic_write(
        path,
        &serde_json::to_string_pretty(&existing).map_err(|e| e.to_string())?,
    )
}

fn write_factory_config(
    routes: &ModelRoutes,
    token: &str,
    base_url: &str,
    limits: &ModelLimits,
) -> Result<(), String> {
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
    models.push(factory_config(routes, token, base_url, limits));
    merge(
        &mut existing,
        serde_json::json!({
            "compactionTokenLimitPerModel": {
                default_model(routes): auto_compact_trigger(limits.max_input_tokens)
            }
        }),
    );
    atomic_write(
        &path,
        &serde_json::to_string_pretty(&existing).map_err(|e| e.to_string())?,
    )
}

fn apply_openclaw_config(
    routes: &ModelRoutes,
    token: &str,
    base_url: &str,
    limits: &ModelLimits,
) -> Result<(), String> {
    let path = home_path(".openclaw/openclaw.json");
    backup("openclaw", "OpenClaw", &path)?;
    let model = default_model(routes);
    let provider = serde_json::json!({ "baseUrl": base_url, "apiKey": token, "api": "openai-completions", "models": [{ "id": model, "name": "9router", "reasoning": true, "input": ["text"], "contextWindow": limits.max_input_tokens, "maxTokens": limits.max_output_tokens }] });
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
    let compaction_json = serde_json::json!({
        "enabled": true,
        "reserveTokens": auto_compact_reserve(limits.max_input_tokens),
        "reserveTokensFloor": auto_compact_reserve(limits.max_input_tokens),
        "keepRecentTokens": compact_keep_recent(limits.max_input_tokens),
        "midTurnPrecheck": { "enabled": true }
    })
    .to_string();
    run_command(std::process::Command::new("openclaw").args([
        "config",
        "set",
        "agents.defaults.compaction",
        &compaction_json,
        "--strict-json",
        "--merge",
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

fn api_base_url(base_url: &str) -> Result<String, String> {
    let base_url = base_url.trim_end_matches('/');
    if base_url.is_empty() {
        return Err("Enter a 9router base URL".into());
    }
    Ok(if base_url.ends_with("/v1") {
        base_url.into()
    } else {
        format!("{base_url}/v1")
    })
}

fn fetch_models(base_url: &str, token: &str, suffix: &str) -> Result<Vec<GatewayModel>, String> {
    let url = format!("{}{}", api_base_url(base_url)?, suffix);
    let output = std::process::Command::new("curl")
        .args([
            "--silent",
            "--show-error",
            "--max-time",
            "12",
            "--header",
            &format!("Authorization: Bearer {token}"),
            "--write-out",
            "\n%{http_code}",
            &url,
        ])
        .output()
        .map_err(|e| format!("Could not start gateway discovery: {e}"))?;
    let marker = output
        .stdout
        .iter()
        .rposition(|byte| *byte == b'\n')
        .ok_or_else(|| "Gateway did not return an HTTP status".to_string())?;
    let body = &output.stdout[..marker];
    let status = std::str::from_utf8(&output.stdout[marker + 1..])
        .ok()
        .and_then(|code| code.trim().parse::<u16>().ok())
        .unwrap_or(0);
    match status {
        200..=299 => {}
        401 => return Err("API key is invalid or expired. Check the 9router API key and try again.".into()),
        403 => return Err("API key is valid but is not permitted to access this 9router gateway.".into()),
        0 if !output.status.success() => return Err("Could not reach the 9router gateway. Check its base URL and network connection.".into()),
        code => return Err(format!("{} returned HTTP {code}", suffix.trim_start_matches('/'))),
    }
    let response: serde_json::Value = serde_json::from_slice(body)
        .map_err(|_| format!("{} did not return a valid models response", suffix.trim_start_matches('/')))?;
    Ok(response
        .get("data")
        .and_then(|data| data.as_array())
        .ok_or_else(|| format!("{} returned no models list", suffix.trim_start_matches('/')))?
        .iter()
        .filter_map(|model| {
            let id = model.get("id")?.as_str()?.to_string();
            let declared_input = model
                .get("max_input_tokens")
                .or_else(|| model.get("context_window"))
                .or_else(|| model.get("contextWindow"))
                .and_then(serde_json::Value::as_u64)
                .or_else(|| model.pointer("/limit/context").and_then(serde_json::Value::as_u64));
            let declared_output = model
                .get("max_output_tokens")
                .or_else(|| model.get("max_tokens"))
                .or_else(|| model.get("maxTokens"))
                .and_then(serde_json::Value::as_u64)
                .or_else(|| model.pointer("/limit/output").and_then(serde_json::Value::as_u64));
            let known = known_model_limits(&id);
            let max_input_tokens = declared_input
                .or_else(|| known.as_ref().map(|(limits, _)| limits.max_input_tokens));
            let max_output_tokens = declared_output
                .or_else(|| known.as_ref().map(|(limits, _)| limits.max_output_tokens));
            let limits_source = if declared_input.is_some() || declared_output.is_some() {
                Some("9router metadata".to_string())
            } else {
                known.map(|(_, source)| source.to_string())
            };
            Some(GatewayModel {
                id,
                owned_by: model.get("owned_by").and_then(|value| value.as_str()).map(Into::into),
                kind: model.get("kind").and_then(|value| value.as_str()).map(Into::into),
                max_input_tokens,
                max_output_tokens,
                limits_source,
            })
        })
        .collect::<Vec<_>>())
}

fn skill_catalog() -> Vec<CapabilitySkill> {
    let source = "https://raw.githubusercontent.com/decolua/9router/refs/heads/master/skills";
    [
        ("9router-chat", "Chat / code-gen", "Direct chat and code generation through the selected coding model.", "chat", None),
        ("9router-image", "Image generation", "Generate or edit images through the selected image model routes.", "image", None),
        ("9router-web-search", "Web search", "Search the web through the selected search providers.", "web", Some("webSearch")),
        ("9router-web-fetch", "Web fetch", "Read a URL as markdown, text, or HTML through the selected fetch providers.", "web", Some("webFetch")),
        ("9router-tts", "Text-to-speech", "Create speech through the selected TTS model routes.", "tts", None),
        ("9router-stt", "Speech-to-text", "Transcribe audio through the selected STT model routes.", "stt", None),
        ("9router-embeddings", "Embeddings", "Create embeddings through the selected embedding model routes.", "embedding", None),
    ]
    .into_iter()
    .map(|(id, name, description, model_group, model_kind)| CapabilitySkill {
        id: id.into(),
        name: name.into(),
        description: description.into(),
        model_group: model_group.into(),
        model_kind: model_kind.map(Into::into),
        source_url: format!("{source}/{id}/SKILL.md"),
    })
    .collect()
}

#[tauri::command]
fn discover_gateway(base_url: String, token: String) -> Result<GatewayCatalog, String> {
    if token.trim().is_empty() {
        return Err("Enter a 9router API key to explore enabled models".into());
    }
    let chat_models = fetch_models(&base_url, &token, "/models")?;
    if chat_models.is_empty() {
        return Err("The API key is valid but this router exposes no chat models".into());
    }
    // Capability endpoints are optional. Fetch them in parallel so an unavailable provider does
    // not make the setup screen wait once per endpoint.
    let optional = std::thread::scope(|scope| {
        let image = scope.spawn(|| fetch_models(&base_url, &token, "/models/image").unwrap_or_default());
        let web = scope.spawn(|| fetch_models(&base_url, &token, "/models/web").unwrap_or_default());
        let tts = scope.spawn(|| fetch_models(&base_url, &token, "/models/tts").unwrap_or_default());
        let stt = scope.spawn(|| fetch_models(&base_url, &token, "/models/stt").unwrap_or_default());
        let embeddings = scope.spawn(|| fetch_models(&base_url, &token, "/models/embedding").unwrap_or_default());
        let image_to_text = scope.spawn(|| fetch_models(&base_url, &token, "/models/image-to-text").unwrap_or_default());
        (
            image.join().unwrap_or_default(),
            web.join().unwrap_or_default(),
            tts.join().unwrap_or_default(),
            stt.join().unwrap_or_default(),
            embeddings.join().unwrap_or_default(),
            image_to_text.join().unwrap_or_default(),
        )
    });
    Ok(GatewayCatalog {
        chat_models,
        image_models: optional.0,
        web_models: optional.1,
        tts_models: optional.2,
        stt_models: optional.3,
        embedding_models: optional.4,
        image_to_text_models: optional.5,
        skills: skill_catalog(),
    })
}

#[tauri::command]
fn get_model_info(base_url: String, token: String, model_id: String) -> Result<ModelInfoResult, String> {
    if token.trim().is_empty() || model_id.trim().is_empty() {
        return Err("Enter an API key and select a model first".into());
    }
    let url = format!("{}/models/info?id={}", api_base_url(&base_url)?, model_id);
    let output = std::process::Command::new("curl")
        .args([
            "--silent", "--show-error", "--fail", "--max-time", "12", "--header",
            &format!("Authorization: Bearer {token}"), &url,
        ])
        .output()
        .map_err(|e| format!("Could not read model details: {e}"))?;
    if !output.status.success() {
        return Err("The gateway did not return model details for this route".into());
    }
    let details = serde_json::from_slice(&output.stdout)
        .map_err(|_| "The gateway returned invalid model details".to_string())?;
    Ok(ModelInfoResult { model_id, details })
}

#[tauri::command]
fn test_image_route(base_url: String, token: String, model_id: String) -> Result<ImageRouteTestResult, String> {
    if token.trim().is_empty() || model_id.trim().is_empty() {
        return Err("Enter an API key and select an image model first".into());
    }
    let url = format!("{}/images/generations", api_base_url(&base_url)?);
    let body = serde_json::json!({
        "model": model_id,
        "prompt": "A minimal blue circle on a plain white background.",
        "n": 1,
    })
    .to_string();
    let null_device = if cfg!(windows) { "NUL" } else { "/dev/null" };
    // Do not use --fail here: the status code is the user-facing readiness signal.
    let output = std::process::Command::new("curl")
        .args([
            "--silent", "--show-error", "--max-time", "45", "--output", null_device,
            "--write-out", "%{http_code}", "--request", "POST", "--header",
            &format!("Authorization: Bearer {token}"), "--header", "Content-Type: application/json",
            "--data", &body, &url,
        ])
        .output()
        .map_err(|e| format!("Could not start the image route test: {e}"))?;
    if !output.status.success() {
        return Ok(ImageRouteTestResult { model_id, status: "error".into(), message: "Network error while testing this image route".into() });
    }
    let code = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let (status, message) = match code.as_str() {
        code if code.starts_with('2') => ("ready", "Ready — the gateway generated a test image."),
        "401" | "403" => ("unauthorized", "Not available to this API key."),
        "503" => ("unavailable", "Enabled, but no linked provider account is currently available."),
        "408" | "429" | "504" => ("unavailable", "Enabled, but the route is temporarily unavailable."),
        _ => ("error", "The gateway rejected this image route test."),
    };
    Ok(ImageRouteTestResult { model_id, status: status.into(), message: message.into() })
}

#[tauri::command]
fn validate_api_key(
    base_url: String,
    token: String,
    required_models: Vec<String>,
) -> Result<ValidationResult, String> {
    if token.trim().is_empty() {
        return Err("Enter both a 9router base URL and API key".into());
    }
    let models = fetch_models(&base_url, &token, "/models")?;
    let model_count = models.len();
    if model_count == 0 {
        return Err("The API key is valid but this router exposes no models".into());
    }
    let available: std::collections::HashSet<&str> = models
        .iter()
        .map(|model| model.id.as_str())
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
        let routes = routes_for_tool(&request, &id);
        if id == "claude" {
            let compact_window = request
                .compact_window
                .or(Some(smallest_route_input_limit(&request, &routes)?));
            backup(&id, &name, &path)?;
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let mut existing = read_json_or_empty(&path, "Claude Code settings")?;
            merge(
                &mut existing,
                json_config(
                    &routes,
                    &request.token,
                    &request.base_url,
                    compact_window,
                    &settings,
                ),
            );
            atomic_write(
                &path,
                &serde_json::to_string_pretty(&existing).map_err(|e| e.to_string())?,
            )?;
            changed.push(format!(
                "{name}: settings merged; auto-compaction uses the smallest mapped model window"
            ));
        } else if id == "codex" {
            let model = codex_model(&routes);
            let limits = limits_for_model(&request, &model)?;
            backup(&id, &name, &path)?;
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let mut existing = read_toml_or_empty(&path, "Codex 9router profile")?;
            merge_toml(
                &mut existing,
                codex_config(
                    &routes,
                    &request.token,
                    &request.base_url,
                    &settings,
                    request
                        .codex_context_window
                        .or(Some(limits.max_input_tokens)),
                ),
            );
            atomic_write(
                &path,
                &toml::to_string_pretty(&existing).map_err(|e| e.to_string())?,
            )?;
            changed.push(format!(
                "{name}: 9router profile saved with an 80% compaction threshold; use codex --profile 9router"
            ));
        } else if id != "pi" {
            changed.push(format!("{name}: detected; direct adapter pending"));
        }
    }
    if request.tool_ids.contains(&"opencode".into()) {
        let routes = routes_for_tool(&request, "opencode");
        let limits = limits_for_model(&request, &default_model(&routes))?;
        write_merged_json(
            "opencode",
            "OpenCode",
            &home_path(".config/opencode/opencode.json"),
            open_code_config(
                &routes,
                &request.token,
                &request.base_url,
                &limits,
            ),
        )?;
        changed.push("OpenCode: model limits and 80% auto-compaction reserve merged".into());
    }
    if request.tool_ids.contains(&"factory".into()) {
        let routes = routes_for_tool(&request, "factory");
        let limits = limits_for_model(&request, &default_model(&routes))?;
        write_factory_config(&routes, &request.token, &request.base_url, &limits)?;
        changed.push("Factory Droid: 9router custom model and per-model compaction threshold merged".into());
    }
    if request.tool_ids.contains(&"openclaw".into()) {
        let routes = routes_for_tool(&request, "openclaw");
        let limits = limits_for_model(&request, &default_model(&routes))?;
        apply_openclaw_config(
            &routes,
            &request.token,
            &request.base_url,
            &limits,
        )?;
        changed.push("OpenClaw: provider limits, mid-turn checks, and compaction reserve configured".into());
    }
    if request.tool_ids.contains(&"pi".into()) {
        let routes = routes_for_tool(&request, "pi");
        let limits = limits_for_model(&request, &default_model(&routes))?;
        write_merged_json(
            "pi",
            "Pi",
            &home_path(".pi/agent/models.json"),
            pi_config(
                &routes,
                &request.token,
                &request.base_url,
                &limits,
            ),
        )?;
        write_merged_json(
            "pi-compaction",
            "Pi compaction settings",
            &home_path(".pi/agent/settings.json"),
            pi_compaction_config(&limits),
        )?;
        changed.push("Pi: provider limits and auto-compaction reserve merged".into());
    }
    if request.cloakbrowser_enabled {
        changed.extend(install_cloakbrowser_for_tools(&request.tool_ids)?);
    }
    if request.computer_use_enabled {
        changed.extend(install_open_computer_use_for_tools(&request.tool_ids)?);
    }
    if request.indie_app_shipping_enabled {
        changed.extend(install_indie_app_shipping_for_tools(&request.tool_ids)?);
    }
    if request.reverse_skill_enabled {
        changed.extend(install_reverse_skill_for_tools(&request.tool_ids)?);
    }
    if request.superpowers_enabled {
        changed.extend(install_superpowers_for_tools(&request.tool_ids)?);
    }
    if request.git_guardian_enabled {
        changed.extend(install_git_guardian_for_tools(&request.tool_ids)?);
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

fn installed_command(name: &str) -> Result<std::process::Command, String> {
    let path = command_path(name).ok_or_else(|| format!("{name} was not found. Re-scan tools after installing it."))?;
    Ok(command_for_path(&path))
}

fn command_for_path(path: &Path) -> std::process::Command {
    #[cfg(windows)]
    if matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some("cmd" | "bat")
    ) {
        let command_processor =
            std::env::var_os("COMSPEC").unwrap_or_else(|| "cmd.exe".into());
        let mut command = std::process::Command::new(command_processor);
        command.args(["/D", "/C"]).arg(path);
        return command;
    }
    std::process::Command::new(path)
}

#[cfg(windows)]
const RTK_WINDOWS_VERSION: &str = "0.43.0";
#[cfg(windows)]
const RTK_WINDOWS_BINARY: &[u8] = include_bytes!("../resources/windows/rtk/rtk.exe");
#[cfg(windows)]
const PONYTAIL_WINDOWS_COMMIT: &str = "16f29800fd2681bdf24f3eb4ccffe38be3baec6b";
#[cfg(windows)]
const PONYTAIL_WINDOWS_ARCHIVE: &[u8] =
    include_bytes!("../resources/windows/ponytail-16f2980.tar.gz");
const RTK_OPENCLAW_ARCHIVE: &[u8] =
    include_bytes!("../resources/rtk-openclaw-v0.43.0.tar.gz");

#[cfg(windows)]
fn ensure_windows_rtk() -> Result<PathBuf, String> {
    let path = home_path(".9router-model-selector/bin/rtk.exe");
    let current_matches = fs::read(&path)
        .map(|contents| contents == RTK_WINDOWS_BINARY)
        .unwrap_or(false);
    if !current_matches {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        atomic_write_bytes(&path, RTK_WINDOWS_BINARY)?;
    }
    Ok(path)
}

#[cfg(windows)]
fn ensure_windows_ponytail() -> Result<PathBuf, String> {
    let directory = home_path(".9router-model-selector/ponytail");
    let version_file = directory.join(".9router-source-commit");
    if fs::read_to_string(&version_file).ok().as_deref() == Some(PONYTAIL_WINDOWS_COMMIT)
        && directory.join(".claude-plugin/marketplace.json").is_file()
        && directory.join(".codex-plugin/plugin.json").is_file()
    {
        return Ok(directory);
    }

    fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
    let archive = home_path(".9router-model-selector/ponytail-bundle.tar.gz");
    atomic_write_bytes(&archive, PONYTAIL_WINDOWS_ARCHIVE)?;
    let archive_arg = archive.display().to_string();
    let directory_arg = directory.display().to_string();
    let tar_path = command_path("tar").unwrap_or_else(|| PathBuf::from("tar.exe"));
    let mut tar = command_for_path(&tar_path);
    tar.args(["-xzf", &archive_arg, "-C", &directory_arg]);
    let result = run_command(&mut tar)
        .map_err(|error| format!("Could not unpack bundled Ponytail: {error}"));
    let _ = fs::remove_file(&archive);
    result?;
    atomic_write(&version_file, PONYTAIL_WINDOWS_COMMIT)?;
    Ok(directory)
}

fn ensure_rtk() -> Result<std::process::Command, String> {
    #[cfg(windows)]
    {
        return Ok(command_for_path(&ensure_windows_rtk()?));
    }
    #[cfg(not(windows))]
    if command_path("rtk").is_none() {
        run_command(std::process::Command::new("/bin/sh").args(["-lc", "curl -fsSL https://raw.githubusercontent.com/rtk-ai/rtk/refs/heads/master/install.sh | sh"]))?;
    }
    #[cfg(not(windows))]
    installed_command("rtk")
}

fn ponytail_marketplace_source() -> Result<String, String> {
    #[cfg(windows)]
    {
        return Ok(ensure_windows_ponytail()?.display().to_string());
    }
    #[cfg(not(windows))]
    {
        Ok("DietrichGebert/ponytail".into())
    }
}

fn ponytail_opencode_source() -> Result<String, String> {
    #[cfg(windows)]
    {
        let path = ensure_windows_ponytail()?.join(".opencode/plugins/ponytail.mjs");
        if !path.is_file() {
            return Err("Bundled Ponytail OpenCode plugin was not found.".into());
        }
        return Ok(path.display().to_string());
    }
    #[cfg(not(windows))]
    {
        Ok("@dietrichgebert/ponytail".into())
    }
}

fn install_ponytail_opencode() -> Result<String, String> {
    let path = home_path(".config/opencode/opencode.json");
    let plugin = ponytail_opencode_source()?;
    let mut existing = read_json_or_empty(&path, "OpenCode config")?;
    let object = existing
        .as_object_mut()
        .ok_or_else(|| "OpenCode config must contain a JSON object; it was not changed.".to_string())?;
    let plugins = object
        .entry("plugin")
        .or_insert_with(|| serde_json::Value::Array(vec![]))
        .as_array_mut()
        .ok_or_else(|| "OpenCode config field `plugin` must be an array; it was not changed.".to_string())?;
    if !plugins.iter().any(|entry| entry.as_str() == Some(&plugin)) {
        plugins.push(serde_json::Value::String(plugin.clone()));
    }
    normalize_opencode_9router_limits(&mut existing, &HashMap::new());
    backup("ponytail-opencode-config", "Ponytail · OpenCode", &path)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    atomic_write(
        &path,
        &serde_json::to_string_pretty(&existing).map_err(|e| e.to_string())?,
    )?;
    #[cfg(windows)]
    return Ok(format!(
        "Ponytail OpenCode plugin enabled from bundled commit {}. Restart OpenCode to activate it.",
        &PONYTAIL_WINDOWS_COMMIT[..7]
    ));
    #[cfg(not(windows))]
    Ok("Ponytail package added to OpenCode plugins. Restart OpenCode to download and activate it.".into())
}

fn ensure_rtk_openclaw_plugin() -> Result<PathBuf, String> {
    let directory = home_path(".9router-model-selector/rtk-openclaw-v0.43.0");
    let plugin = directory.join("openclaw");
    if plugin.join("openclaw.plugin.json").is_file() {
        return Ok(plugin);
    }
    fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
    let archive = home_path(".9router-model-selector/rtk-openclaw-v0.43.0.tar.gz");
    atomic_write_bytes(&archive, RTK_OPENCLAW_ARCHIVE)?;
    let archive_arg = archive.display().to_string();
    let directory_arg = directory.display().to_string();
    let tar_path = command_path("tar").unwrap_or_else(|| {
        if cfg!(windows) {
            PathBuf::from("tar.exe")
        } else {
            PathBuf::from("tar")
        }
    });
    let mut tar = command_for_path(&tar_path);
    tar.args(["-xzf", &archive_arg, "-C", &directory_arg]);
    let result = run_command(&mut tar)
        .map_err(|error| format!("Could not unpack the bundled RTK OpenClaw plugin: {error}"));
    let _ = fs::remove_file(&archive);
    result?;
    if plugin.join("openclaw.plugin.json").is_file() {
        Ok(plugin)
    } else {
        Err("The bundled RTK OpenClaw plugin was incomplete after unpacking.".into())
    }
}

fn optimizer_workspace(workspace_path: Option<String>) -> Result<PathBuf, String> {
    let path = workspace_path
        .filter(|path| !path.trim().is_empty())
        .map(PathBuf::from)
        .ok_or_else(|| "Choose a project workspace before installing this RTK adapter.".to_string())?;
    if path.is_dir() {
        Ok(path)
    } else {
        Err("The selected RTK workspace does not exist or is not a directory.".into())
    }
}

fn run_rtk_init(args: &[&str], workspace: Option<&Path>) -> Result<(), String> {
    let mut rtk = ensure_rtk()?;
    rtk.args(args);
    if let Some(workspace) = workspace {
        rtk.current_dir(workspace);
    }
    run_command(&mut rtk)?;
    Ok(())
}

#[tauri::command]
fn install_optimizer(
    tool: String,
    target_tool: String,
    workspace_path: Option<String>,
) -> Result<String, String> {
    match (tool.as_str(), target_tool.as_str()) {
        ("rtk", "claude") => {
            #[cfg(windows)]
            run_rtk_init(&["init", "-g", "--claude-md"], None)?;
            #[cfg(not(windows))]
            run_rtk_init(&["init", "-g", "--auto-patch"], None)?;
            #[cfg(windows)]
            return Ok(format!(
                "RTK {RTK_WINDOWS_VERSION} installed from the bundled Windows binary. Claude Code instruction mode enabled; WSL is required for a full rewrite hook."
            ));
            #[cfg(not(windows))]
            Ok("RTK installed and Claude Code hook enabled.".into())
        }
        ("rtk", "cursor") => {
            #[cfg(windows)]
            run_rtk_init(&["init", "-g", "--agent", "cursor"], None)?;
            #[cfg(not(windows))]
            run_rtk_init(
                &["init", "-g", "--agent", "cursor", "--auto-patch"],
                None,
            )?;
            #[cfg(windows)]
            return Ok(format!(
                "RTK {RTK_WINDOWS_VERSION} installed from the bundled Windows binary. Cursor instruction integration enabled; native Windows does not provide the Unix rewrite hook."
            ));
            #[cfg(not(windows))]
            Ok("RTK installed and Cursor hook enabled.".into())
        }
        ("rtk", "codex") => {
            run_rtk_init(&["init", "-g", "--codex"], None)?;
            #[cfg(windows)]
            return Ok(format!(
                "RTK {RTK_WINDOWS_VERSION} installed from the bundled Windows binary and configured for Codex."
            ));
            #[cfg(not(windows))]
            Ok("RTK installed with its global hook setup for Codex.".into())
        }
        ("rtk", "copilot-vscode" | "copilot-cli") => {
            run_rtk_init(&["init", "-g", "--copilot"], None)?;
            Ok("RTK global Copilot hook and instructions enabled for VS Code Chat and Copilot CLI.".into())
        }
        ("rtk", "gemini") => {
            run_rtk_init(&["init", "-g", "--gemini"], None)?;
            Ok("RTK global Gemini CLI BeforeTool integration enabled.".into())
        }
        ("rtk", "opencode") => {
            run_rtk_init(&["init", "-g", "--opencode"], None)?;
            Ok("RTK global OpenCode tool.execute.before plugin enabled.".into())
        }
        ("rtk", "pi") => {
            run_rtk_init(&["init", "-g", "--agent", "pi"], None)?;
            Ok("RTK global Pi tool_call extension enabled.".into())
        }
        ("rtk", "hermes") => {
            run_rtk_init(&["init", "--agent", "hermes"], None)?;
            Ok("RTK Hermes terminal rewrite plugin enabled.".into())
        }
        ("rtk", "factory") => {
            run_rtk_init(&["init", "-g", "--agent", "droid"], None)?;
            Ok("RTK global Factory Droid Execute hook enabled.".into())
        }
        ("rtk", "cline" | "roo") => {
            let workspace = optimizer_workspace(workspace_path)?;
            run_rtk_init(&["init", "--agent", "cline"], Some(&workspace))?;
            Ok(format!(
                "RTK project instructions enabled for Cline / Roo Code in {}.",
                workspace.display()
            ))
        }
        ("rtk", "kilo") => {
            let workspace = optimizer_workspace(workspace_path)?;
            run_rtk_init(&["init", "--agent", "kilocode"], Some(&workspace))?;
            Ok(format!(
                "RTK project rules enabled for Kilo Code in {}.",
                workspace.display()
            ))
        }
        ("rtk", "windsurf") => {
            let workspace = optimizer_workspace(workspace_path)?;
            run_rtk_init(
                &["init", "-g", "--agent", "windsurf"],
                Some(&workspace),
            )?;
            Ok(format!(
                "RTK Windsurf rules enabled in {}.",
                workspace.display()
            ))
        }
        ("rtk", "antigravity") => {
            let workspace = optimizer_workspace(workspace_path)?;
            run_rtk_init(&["init", "--agent", "antigravity"], Some(&workspace))?;
            Ok(format!(
                "RTK Antigravity rules enabled in {}.",
                workspace.display()
            ))
        }
        ("rtk", "openclaw") => {
            ensure_rtk()?;
            let plugin = ensure_rtk_openclaw_plugin()?;
            let mut openclaw = installed_command("openclaw")?;
            openclaw.args(["plugins", "install", &plugin.display().to_string()]);
            run_command(&mut openclaw)?;
            Ok("RTK bundled OpenClaw before_tool_call plugin installed.".into())
        }
        ("rtk", "vibe") => Err(
            "Mistral Vibe does not expose the required tool callback yet; RTK support is pending upstream."
                .into(),
        ),
        ("rtk", "continue") => Err(
            "RTK does not currently publish a verified Continue adapter.".into(),
        ),
        ("ponytail", "claude") => {
            let source = ponytail_marketplace_source()?;
            let mut claude = installed_command("claude")?;
            claude.args(["plugin", "marketplace", "add", &source]);
            run_command(&mut claude)?;
            let mut claude = installed_command("claude")?;
            claude.args(["plugin", "install", "ponytail@ponytail"]);
            run_command(&mut claude)?;
            #[cfg(windows)]
            return Ok(format!(
                "Ponytail installed from bundled commit {}. Start a new Claude Code session to activate it.",
                &PONYTAIL_WINDOWS_COMMIT[..7]
            ));
            #[cfg(not(windows))]
            Ok("Ponytail installed. Start a new Claude Code session to activate it.".into())
        }
        ("ponytail", "codex") => {
            let source = ponytail_marketplace_source()?;
            let mut codex = installed_command("codex")?;
            codex.args(["plugin", "marketplace", "add", &source]);
            run_command(&mut codex)?;
            let mut codex = installed_command("codex")?;
            codex.args(["plugin", "add", "ponytail@ponytail"]);
            run_command(&mut codex)?;
            #[cfg(windows)]
            return Ok(format!(
                "Ponytail installed for Codex from bundled commit {}. Trust its lifecycle hooks in /hooks, then start a new task.",
                &PONYTAIL_WINDOWS_COMMIT[..7]
            ));
            #[cfg(not(windows))]
            Ok("Ponytail installed for Codex. Trust its lifecycle hooks in /hooks, then start a new task.".into())
        }
        ("ponytail", "opencode") => install_ponytail_opencode(),
        ("ponytail", "copilot-cli") => {
            let mut copilot = installed_command("copilot")?;
            copilot.args([
                "plugin",
                "marketplace",
                "add",
                "DietrichGebert/ponytail",
            ]);
            run_command(&mut copilot)?;
            let mut copilot = installed_command("copilot")?;
            copilot.args(["plugin", "install", "ponytail@ponytail"]);
            run_command(&mut copilot)?;
            Ok("Ponytail installed for GitHub Copilot CLI. Start a new interactive session to activate it.".into())
        }
        ("ponytail", "pi") => {
            let mut pi = installed_command("pi")?;
            pi.args(["install", "git:github.com/DietrichGebert/ponytail"]);
            run_command(&mut pi)?;
            Ok("Ponytail installed for Pi with its official package adapter.".into())
        }
        ("ponytail", "gemini") => {
            let mut gemini = installed_command("gemini")?;
            gemini.args([
                "extensions",
                "install",
                "https://github.com/DietrichGebert/ponytail",
            ]);
            run_command(&mut gemini)?;
            Ok("Ponytail extension installed for Gemini CLI. Start a new session to activate it.".into())
        }
        ("ponytail", "antigravity") => {
            let mut antigravity = installed_command("agy")
                .or_else(|_| installed_command("antigravity"))?;
            antigravity.args([
                "plugin",
                "install",
                "https://github.com/DietrichGebert/ponytail",
            ]);
            run_command(&mut antigravity)?;
            Ok("Ponytail plugin installed for Google Antigravity.".into())
        }
        ("ponytail", "hermes") => {
            let mut hermes = installed_command("hermes")?;
            hermes.args([
                "plugins",
                "install",
                "DietrichGebert/ponytail",
                "--enable",
            ]);
            run_command(&mut hermes)?;
            Ok("Ponytail plugin installed and enabled for Hermes. Restart Hermes to activate it.".into())
        }
        ("ponytail", "openclaw") => {
            let mut clawhub = installed_command("clawhub")?;
            clawhub.args(["install", "ponytail"]);
            run_command(&mut clawhub)?;
            Ok("Ponytail skill installed for OpenClaw through ClawHub.".into())
        }
        ("ponytail", "cursor" | "windsurf" | "cline" | "roo" | "kilo" | "copilot-vscode") => Err(
            "Ponytail uses project instruction files for this host. Automatic installation stays disabled until those files participate in the app's backup/restore registry."
                .into(),
        ),
        _ => Err("This optimizer does not have a verified installer for the selected tool.".into()),
    }
}

pub fn cli_setup(model: String, token: String) -> Result<Vec<String>, String> {
    let limits = known_model_limits(&model)
        .map(|(limits, _)| limits)
        .unwrap_or(ModelLimits {
            max_input_tokens: 128_000,
            max_output_tokens: 32_000,
        });
    let mut model_limits = HashMap::new();
    model_limits.insert(model.clone(), limits);
    let tool_ids = detect_tools()
        .into_iter()
        .filter(|tool| tool.found)
        .map(|tool| tool.id)
        .collect();
    apply_configuration(ApplyRequest {
        tool_ids,
        routes: ModelRoutes {
            default_model: model.clone(),
            opus: model.clone(),
            sonnet: model.clone(),
            haiku: model,
        },
        token,
        base_url: "http://127.0.0.1:20128/v1".into(),
        compact_window: None,
        codex_context_window: None,
        tool_settings: HashMap::new(),
        tool_models: HashMap::new(),
        tool_model_pools: HashMap::new(),
        model_limits,
        claude_models: None,
        cloakbrowser_enabled: true,
        computer_use_enabled: false,
        indie_app_shipping_enabled: false,
        reverse_skill_enabled: false,
        superpowers_enabled: false,
        git_guardian_enabled: false,
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
            &ModelRoutes { default_model: "cx/gpt-5.6-terra".into(), opus: "cc/claude-opus-4-8".into(), sonnet: "cx/gpt-5.6-terra".into(), haiku: "cx/gpt-5.6-luna".into() },
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

    #[test]
    fn opencode_migration_completes_all_existing_9router_limits() {
        let mut config = serde_json::json!({
            "provider": {
                "9router": {
                    "models": {
                        "cc/claude-opus-4-8": {
                            "limit": { "context": 272_000 }
                        },
                        "cx/gpt-5.5": {
                            "name": "cx/gpt-5.5"
                        }
                    }
                }
            }
        });
        normalize_opencode_9router_limits(&mut config, &HashMap::new());
        assert_eq!(
            config.pointer("/provider/9router/models/cc~1claude-opus-4-8/limit/output"),
            Some(&serde_json::json!(128_000))
        );
        assert_eq!(
            config.pointer("/provider/9router/models/cx~1gpt-5.5/limit/context"),
            Some(&serde_json::json!(272_000))
        );
    }

    #[test]
    fn known_limits_distinguish_subscription_codex_and_claude_models() {
        let (codex, codex_source) = known_model_limits("cx/gpt-5.6-sol").unwrap();
        assert_eq!(codex.max_input_tokens, 272_000);
        assert_eq!(codex.max_output_tokens, 128_000);
        assert_eq!(codex_source, "Codex subscription catalog");

        let (claude, claude_source) = known_model_limits("cc/claude-opus-4-8").unwrap();
        assert_eq!(claude.max_input_tokens, 1_000_000);
        assert_eq!(claude.max_output_tokens, 128_000);
        assert_eq!(claude_source, "Claude model documentation");

        assert!(known_model_limits("xai/grok-4.5").is_none());
    }

    #[test]
    fn compaction_policy_keeps_twenty_percent_headroom() {
        assert_eq!(auto_compact_trigger(272_000), 217_600);
        assert_eq!(auto_compact_reserve(272_000), 54_400);
        assert_eq!(compact_keep_recent(272_000), 20_000);
    }

    #[test]
    fn native_json_adapters_receive_model_aware_compaction_values() {
        let routes = ModelRoutes {
            default_model: "cx/gpt-5.6-sol".into(),
            opus: "cx/gpt-5.6-sol".into(),
            sonnet: "cx/gpt-5.6-sol".into(),
            haiku: "cx/gpt-5.6-sol".into(),
        };
        let limits = ModelLimits {
            max_input_tokens: 272_000,
            max_output_tokens: 128_000,
        };
        let opencode = open_code_config(&routes, "token", "https://9router.link/v1", &limits);
        assert_eq!(
            opencode.pointer("/compaction/reserved"),
            Some(&serde_json::json!(54_400))
        );
        assert_eq!(
            opencode.pointer("/provider/9router/models/cx~1gpt-5.6-sol/limit/context"),
            Some(&serde_json::json!(272_000))
        );

        let pi = pi_compaction_config(&limits);
        assert_eq!(
            pi.pointer("/compaction/reserveTokens"),
            Some(&serde_json::json!(54_400))
        );

        let factory = factory_config(&routes, "token", "https://9router.link/v1", &limits);
        assert_eq!(
            factory.get("maxContextLimit"),
            Some(&serde_json::json!(272_000))
        );
        assert_eq!(
            factory.get("maxOutputTokens"),
            Some(&serde_json::json!(128_000))
        );
    }
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            detect_tools,
            detect_optimizer_tools,
            discover_gateway,
            get_model_info,
            test_image_route,
            validate_api_key,
            apply_configuration,
            list_backups,
            restore_backup,
            install_optimizer
        ])
        .run(tauri::generate_context!())
        .expect("error while running 9router desktop");
}
