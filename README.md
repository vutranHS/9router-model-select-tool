# 9router Model Selector

A Tauri desktop model-selection and configuration tool for 9router. It is separate from the 9router proxy itself.

## What it does

- Detects supported tools without treating a generic VS Code extensions folder as proof that a specific extension is installed.
- Detects CLIs by executable path (including common GUI-missing `PATH` locations, NVM, Bun, npm-global and OpenCode installs), not merely because a stale config directory exists.
- Lets users select a usage-oriented scenario such as `daily_coding`, which maps the Opus, Sonnet, and Haiku aliases directly to 9router model IDs.
- Shows the exact generated settings before applying them.
- Creates a timestamped original-state snapshot before updating a configuration file.
- Refuses to overwrite malformed JSON/TOML, uses atomic writes, and validates the required direct model IDs as well as the 9router API key.
- Sets proactive Claude Code auto-compaction from the smallest verified context window in the selected direct-model scenario.
- Provides a clear post-setup status screen.

## Tool support policy

| Tool | Setup mode | Notes |
| --- | --- | --- |
| Claude Code | Automatic | Merges 9router endpoint, token, model aliases, compaction, effort, and permissions into `~/.claude/settings.json`. |
| Codex CLI | Automatic profile | Writes `~/.codex/9router.config.toml`; the app deliberately does not modify global `~/.codex/config.toml`. Start CLI with `codex --profile 9router`. |
| OpenCode | Automatic | Adds an OpenAI-compatible `9router` provider to the global OpenCode config. |
| OpenClaw | Automatic | Adds a `models.providers.9router` OpenAI-compatible provider through its own config command. |
| Factory Droid | Automatic | Adds a 9router BYOK custom model to `~/.factory/settings.json`. |
| Pi | Automatic | Adds an OpenAI-compatible `9router` provider to `~/.pi/agent/models.json`. |
| GitHub Copilot (VS Code) | Guided | Its Model providers UI supports an OpenAI-compatible BYOK provider; its credential-store state is not edited directly. |
| GitHub Copilot CLI | Guided | BYOK uses shell environment variables; the app does not edit shell startup files silently. |
| Cursor | Guided | Its OpenAI base-URL override is global and can disable built-in OpenAI models, so it must be confirmed in Cursor Settings. |
| Cline, Roo Code, Kilo Code | Guided | They support compatible providers, but credentials/settings are extension-managed; the app does not write VS Code extension state directly. |
| Mistral Vibe CLI | Guided | Its provider preset needs both `~/.vibe/config.toml` and a token environment source; the app will not create an incomplete two-file setup. |
| Continue | Guided | It supports an OpenAI-compatible model in its YAML configuration, but extension-owned state is not modified directly. |
| Hermes | Guided | Its `hermes model` Custom endpoint workflow supports an OpenAI-compatible URL, key, model, and context length. |

Gemini CLI has a Gemini-API base-URL override rather than a documented OpenAI-compatible adapter. Windsurf and Google Antigravity have no documented, safe user-level custom-endpoint configuration. They are intentionally not changed.

## Reasoning presets

Claude Code supports `low`, `medium`, `high`, `xhigh`, and `max`. Codex models advertise their supported effort values; current Codex models use `low`, `medium`, `high`, and `xhigh`. The app maps the shared tiers one-to-one and safely caps legacy/Claude-only `max` at Codex `xhigh`.

| Scenario | Claude Code | Codex | Rationale |
| --- | --- | --- | --- |
| Daily Coding | `medium` | `medium` | Best speed/depth balance for regular implementation. |
| Direct Coding | `high` | `high` | Longer-running implementation and debugging. |
| Premium | `high` | `high` | Quality-sensitive work without maximum latency/cost. |
| Claude Only | `high` | `high` | Uses the 1M-context Claude route while retaining strong planning. |
| Codex Only | `high` | `high` | Strong direct Codex coding route. |
| Heavy Reasoning | `max` | `xhigh` | Deep investigations, major refactors, and hard tradeoffs. |

Codex profiles set their own context window and auto-compaction threshold (80%); it is not inherited from the smaller Claude helper-model window. Claude Code retains the conservative smallest selected alias window.

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
