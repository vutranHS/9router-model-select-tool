# 9router Model Selector

A Tauri desktop model-selection and configuration tool for 9router. It is separate from the 9router proxy itself.

## What it does

- Detects supported tools on macOS (Claude Code, Codex CLI, Cursor, Cline, Roo Code, Continue).
- Lets users select a usage-oriented scenario such as `daily_coding`, which maps the Opus, Sonnet, and Haiku aliases directly to 9router model IDs.
- Shows the exact generated settings before applying them.
- Creates a timestamped `.bak.<unix-time>` copy before updating a configuration file.
- Merges the 9router settings into existing JSON rather than replacing a file.
- Sets proactive Claude Code auto-compaction from the smallest verified context window in the selected direct-model scenario.
- Provides a clear post-setup status screen.

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

The UI previews configuration before applying it. Existing JSON keys are retained, only 9router keys are merged, and a backup is made before each change. API tokens are never displayed in the preview. Production keychain storage and live API health checks are intentionally represented in the UI but should be wired to the deployment’s chosen platform credential store and 9router health endpoint before release.
