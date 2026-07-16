# 9router Model Selector

Desktop companion app for configuring AI coding tools to use models exposed by a 9router gateway.

This project is **not** the 9router proxy. It discovers the coding tools installed on the current machine, explores the models enabled for a 9router API key, and safely writes the supported client configurations.

## Install from GitHub Releases

Download the newest installer from:

**[Latest GitHub Release](https://github.com/vutranHS/9router-model-select-tool/releases/latest)**

### macOS Apple Silicon

Supported architecture: Apple Silicon (`arm64`, including M1, M2, M3, and M4).

1. Download the file ending in `_aarch64.dmg`.
2. Open the DMG.
3. Drag **9router Model Selector** into **Applications**.
4. Open the app from Applications.

Release builds are signed and notarized when the Apple signing secrets are available to the release workflow. If macOS blocks an older or unsigned build, open **System Settings → Privacy & Security** and review the blocked-app message before choosing **Open Anyway**.

An Intel macOS installer is not currently published.

### Windows

Supported architecture: Windows x64.

1. Download the file ending in `_x64-setup.exe`.
2. Run the installer.
3. Launch **9router Model Selector** from the Start menu.

The Windows installer may show a Microsoft Defender SmartScreen warning until a Windows code-signing certificate is added to the release pipeline. Only continue after confirming that the installer came from this repository's official Releases page.

All published versions and release notes are available on the [Releases page](https://github.com/vutranHS/9router-model-select-tool/releases).

## How it works

The app uses a guided setup flow:

1. **Detect tools** — finds installed coding tools using executable paths and exact editor extension IDs.
2. **Connect gateway** — validates the 9router base URL and API key.
3. **Explore models** — fetches the enabled chat and capability-specific model endpoints.
4. **Assign models** — selects a direct model for each coding tool. It does not depend on a 9router combo.
5. **Configure limits** — reviews or edits maximum input and output tokens so each tool can compact before the selected model overflows.
6. **Route capabilities** — assigns one or more available models to image generation, web search/fetch, speech, transcription, embeddings, and other supported skills.
7. **Choose integrations** — optionally installs local skills and token-saving adapters.
8. **Review and apply** — previews changes, validates the key and selected model IDs again, creates backups, and writes the supported configurations.
9. **Restore if needed** — restores a timestamped original-state snapshot from the app.

Known Claude and Codex model limits are prefilled. Unknown or newly added models remain editable, so a user can provide the correct input and output limits without waiting for an app update. Automatic compaction targets roughly 80% of the effective context window.

## Claude Code mapping

Claude Code exposes more routing controls than most coding tools. The app therefore lets the user map all four entries independently:

- Default model
- Opus alias
- Sonnet alias
- Haiku alias

Each entry may point directly to any compatible model returned by the gateway, including a Codex model. Claude Code's compaction limit is based on the smallest configured route so that helper-model calls do not overflow.

Other supported coding tools generally use one selected default model plus that tool's own context and compaction settings.

## Tool support

| Tool | Setup mode | Current behavior |
| --- | --- | --- |
| Claude Code | Automatic | Merges the endpoint, API key, default/Opus/Sonnet/Haiku routes, context-aware compaction, effort, permissions, and attribution preferences into `~/.claude/settings.json`. |
| Codex CLI | Automatic profile | Writes `~/.codex/9router.config.toml` and keeps the global config untouched. Start it with `codex --profile 9router`. |
| OpenCode | Automatic | Merges an OpenAI-compatible `9router` provider, selected model, model limits, and compaction settings into `~/.config/opencode/opencode.json`. |
| OpenClaw | Automatic | Configures the provider, primary model, context/output limits, and compaction through OpenClaw's configuration interface. |
| Factory Droid | Automatic | Adds a 9router BYOK model with context, output, and compaction limits to `~/.factory/settings.json`. |
| Pi | Automatic | Adds the provider and model metadata to `~/.pi/agent/models.json` and selects it in `~/.pi/agent/settings.json`. |
| Cursor | Guided | Shows the required OpenAI-compatible endpoint/model settings without silently replacing Cursor-managed credentials. |
| Cline, Roo Code, Kilo Code | Guided | Provides the selected endpoint and model values; extension-owned secret storage is not edited directly. |
| GitHub Copilot for VS Code | Guided | Uses the documented BYOK/model-provider UI instead of modifying VS Code secret storage. |
| GitHub Copilot CLI | Guided | Shows the required environment-based BYOK setup without editing shell startup files. |
| Mistral Vibe CLI, Continue, Hermes | Guided | Provides the selected gateway values while avoiding incomplete or unsafe partial configuration. |

Only installed tools are shown in the apply flow. Tools without a documented and reliable custom OpenAI-compatible endpoint are not automatically overridden.

## Capability routes

When supported by the gateway, the app explores:

- Chat models
- Image generation and image-to-text
- Web search and web fetch
- Text-to-speech
- Speech-to-text
- Embeddings

A route can use one or multiple enabled models. This allows, for example, a coding tool to use a Codex model while image generation is routed to Grok, Gemini, or another image-capable provider.

## Optional integrations

The app can install or configure these separately from the main model routing:

- **RTK** — token-saving command rewriting using the adapter supported by each detected coding tool.
- **Ponytail** — coding-tool integration using Ponytail's supported host-specific setup.
- **CloakBrowser / CloakMCP** — enabled by default for supported hosts to fetch pages that commonly block coding agents. The bundled runner uses an isolated temporary browser profile and cleans it up after success, timeout, or failure.
- **Open Computer Use** — optional browser/computer interaction for visual testing and GUI debugging.
- **Indie App Shipping** — optional mobile-app shipping workflow.
- **Reverse Skill** — optional reverse-engineering workflow.
- **Superpowers** — optional installation through the host's supported marketplace, plugin, or package mechanism.
- **Git Guardian Pro** — optional bundled repository-safety skill for Claude Code and Codex. Installing it does not require Git; without Git it stays advisory and does not attempt repository initialization.

Project-scoped adapters require a workspace folder. The app reports guided steps instead of guessing when an integration has no verified automatic installer for the selected host.

## Safety

- A `401 Unauthorized` response is reported as an invalid or expired API key.
- The API key and every selected direct model are validated before automatic configuration is written.
- Existing JSON and TOML files are parsed before modification; malformed files are rejected.
- Supported adapters merge only the keys managed by the app.
- Writes are atomic.
- A timestamped original-state backup is created before each change.
- Backups can be restored from the app.
- API keys are hidden from previews and sent only to the configured gateway.
- Extension-managed credential stores and shell startup files are not silently modified.

## Run from source

### Requirements

- Node.js 22
- Rust stable
- The [Tauri 2 system prerequisites](https://v2.tauri.app/start/prerequisites/) for the current operating system

Install dependencies and start the desktop app:

```bash
npm ci
npm run tauri dev
```

Build and type-check the frontend:

```bash
npm run build
```

Run the Rust test suite:

```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

Create a local installer:

```bash
# macOS Apple Silicon
npm run build:mac-arm

# Windows x64, run on Windows
npm run build:windows-x64
```

## Command-line setup

The native core also exposes a lower-level CLI for a direct Claude Code setup:

```bash
cargo run --manifest-path src-tauri/Cargo.toml \
  --bin 9router-model-selector -- \
  setup --model cc/claude-sonnet-5 --token <9router-api-key>
```

The desktop app is recommended because it performs model discovery, per-tool routing, limit review, optional integration setup, and restore management.

## Release process

The GitHub Actions workflow builds:

- A signed/notarized Apple Silicon DMG when the Apple secrets are configured.
- A Windows x64 NSIS installer.

Pushing a version tag triggers both builds and publishes their artifacts to a GitHub Release:

```bash
git tag v0.x.y
git push origin v0.x.y
```

Keep the versions in `package.json`, `src-tauri/Cargo.toml`, and `src-tauri/tauri.conf.json` in sync before tagging. Apple signing and notarization setup is documented in [`.github/RELEASE.md`](.github/RELEASE.md).

## Project layout

- `src/main.tsx` — React setup wizard and client-side setup state.
- `src/styles.css` — desktop UI styling.
- `src-tauri/src/lib.rs` — tool detection, endpoint exploration, validation, backups, configuration merging, integration installation, and restore commands.
- `src-tauri/resources/` — bundled local resources such as the CloakBrowser runner.
- `.github/workflows/build-installers.yml` — macOS ARM and Windows x64 release builds.

## Repository

[github.com/vutranHS/9router-model-select-tool](https://github.com/vutranHS/9router-model-select-tool)
