# 9router Model Selector

A Tauri desktop model-selection and configuration tool for 9router. It is separate from the 9router proxy itself.

## What it does

- Detects supported tools without treating a generic VS Code extensions folder as proof that a specific extension is installed.
- Lets users select a usage-oriented scenario such as `daily_coding`, which maps the Opus, Sonnet, and Haiku aliases directly to 9router model IDs.
- Shows the exact generated settings before applying them.
- Creates a timestamped `.bak.<unix-time>` copy before updating a configuration file.
- Merges the 9router settings into existing JSON rather than replacing a file.
- Sets proactive Claude Code auto-compaction from the smallest verified context window in the selected direct-model scenario.
- Provides a clear post-setup status screen.

## Tool support policy

| Tool | Setup mode | Notes |
| --- | --- | --- |
| Claude Code | Automatic | Merges 9router endpoint, token, model aliases, compaction, effort, and permissions into `~/.claude/settings.json`. |
| Codex CLI | Automatic | Adds a `9router` Responses API provider, model, endpoint, bearer token, reasoning effort, sandbox, and approval settings to `~/.codex/config.toml`. |
| OpenCode | Automatic | Adds an OpenAI-compatible `9router` provider to the global OpenCode config. |
| OpenClaw | Automatic | Adds a `models.providers.9router` OpenAI-compatible provider through its own config command. |
| Factory Droid | Automatic | Adds a 9router BYOK custom model to `~/.factory/settings.json`. |
| GitHub Copilot CLI | Guided | BYOK uses shell environment variables; the app does not edit shell startup files silently. |
| Cursor | Guided | Its OpenAI base-URL override is global and can disable built-in OpenAI models, so it must be confirmed in Cursor Settings. |
| Cline, Roo Code, Kilo Code | Guided | They support compatible providers, but credentials/settings are extension-managed; the app does not write VS Code extension state directly. |
| Mistral Vibe CLI | Guided | Its provider preset needs both `~/.vibe/config.toml` and a token environment source; the app will not create an incomplete two-file setup. |

Tools without a documented, user-level custom-provider route are intentionally not detected or changed.

## Run it

```bash
npm install
npm run tauri dev
```

Create a production bundle with:

```bash
npm run tauri build
```

The same native core is available from the CLI:

```bash
cargo run --manifest-path src-tauri/Cargo.toml --bin 9router-model-selector -- setup --model cc/claude-sonnet-5 --token <token>
```

## Project layout

- `src/` — React wizard interface.
- `src-tauri/src/lib.rs` — native detection, backup, JSON merge, and apply commands shared by the GUI boundary.
- `src-tauri/` — Tauri desktop configuration.

## Safety model

The UI validates the 9router token with `/models` before writing an automatic adapter. Existing keys are retained, a full original-state snapshot is created before each write, and API tokens are never displayed in the preview. Codex stores its custom-provider bearer token in its local `config.toml`, which is supported by Codex but should be protected with normal account-level filesystem permissions.
