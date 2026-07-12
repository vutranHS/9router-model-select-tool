import React, { useEffect, useMemo, useState } from "react";
import { createRoot } from "react-dom/client";
import { invoke } from "@tauri-apps/api/core";
import { Check, ChevronRight, CircleCheck, Clock3, Code2, Copy, Gauge, Github, HeartPulse, KeyRound, LoaderCircle, LockKeyhole, RefreshCw, ShieldCheck, Sparkles, Terminal, Undo2, X } from "lucide-react";
import "./styles.css";

type Tool = { id: string; name: string; detail: string; path: string; found: boolean; selected: boolean };
type RouterModel = { id: string; contextWindow?: number };
type RouteRole = "opus" | "sonnet" | "haiku";
type ModelRoute = { model: RouterModel };
type Scenario = { id: string; icon: string; name: string; description: string; accent: string; routes: Record<RouteRole, ModelRoute>; recommended?: boolean };
type ToolSettings = { bypassPermissions: boolean; effortLevel: "auto" | "low" | "medium" | "high" | "max" };
type BackupEntry = { toolId: string; toolName: string; originalPath: string; backupPath: string; createdAt: string };
type ValidationResult = { valid: boolean; modelCount: number; message: string };

const seedTools: Tool[] = [
  { id: "claude", name: "Claude Code", detail: "settings.json", path: "~/.claude", found: true, selected: true },
  { id: "codex", name: "Codex CLI", detail: "config.toml", path: "~/.codex", found: true, selected: true },
  { id: "cursor", name: "Cursor", detail: "Settings UI", path: "Not detected", found: false, selected: false },
  { id: "cline", name: "Cline", detail: "VS Code extension", path: "Not detected", found: false, selected: false },
  { id: "roo", name: "Roo Code", detail: "VS Code extension", path: "Not detected", found: false, selected: false },
  { id: "kilo", name: "Kilo Code", detail: "VS Code extension", path: "Not detected", found: false, selected: false },
  { id: "vibe", name: "Mistral Vibe CLI", detail: "config.toml + .env", path: "Not detected", found: false, selected: false },
  { id: "continue", name: "Continue", detail: "config.json", path: "Not detected", found: false, selected: false },
  { id: "copilot-cli", name: "GitHub Copilot CLI", detail: "copilot command", path: "Not detected", found: false, selected: false },
  { id: "opencode", name: "OpenCode", detail: "opencode command", path: "Not detected", found: false, selected: false },
  { id: "openclaw", name: "OpenClaw", detail: "openclaw command", path: "Not detected", found: false, selected: false },
  { id: "factory", name: "Factory Droid", detail: "droid command", path: "Not detected", found: false, selected: false },
];
const models = {
  // Current 9router desktop-tool assumption for Codex subscription routes.
  sol: { id: "cx/gpt-5.6-sol", contextWindow: 272_000 }, terra: { id: "cx/gpt-5.6-terra", contextWindow: 272_000 }, luna: { id: "cx/gpt-5.6-luna", contextWindow: 272_000 }, gpt55: { id: "cx/gpt-5.5", contextWindow: 272_000 },
  opus48: { id: "cc/claude-opus-4-8", contextWindow: 1_000_000 }, opus47: { id: "cc/claude-opus-4-7", contextWindow: 1_000_000 }, sonnet5: { id: "cc/claude-sonnet-5", contextWindow: 1_000_000 }, sonnet46: { id: "cc/claude-sonnet-4-6", contextWindow: 1_000_000 },
  sonnet45: { id: "cc/claude-sonnet-4-5-20250929", contextWindow: 200_000 }, haiku45: { id: "cc/claude-haiku-4-5-20251001", contextWindow: 200_000 }, opus45: { id: "cc/claude-opus-4-5-20251101", contextWindow: 200_000 },
};
const routes = (opus: RouterModel, sonnet: RouterModel, haiku: RouterModel): Record<RouteRole, ModelRoute> => ({
  opus: { model: opus }, sonnet: { model: sonnet }, haiku: { model: haiku },
});
const scenarios: Scenario[] = [
  { id: "daily_coding", icon: "☀", name: "Daily Coding", description: "Balanced direct models for everyday work", accent: "mint", routes: routes(models.opus48, models.sonnet5, models.haiku45), recommended: true },
  { id: "heavy_reasoning", icon: "✦", name: "Heavy Reasoning", description: "Highest-reasoning Claude models, specified directly", accent: "purple", routes: routes(models.opus48, models.sonnet5, models.haiku45) },
  { id: "never_stop", icon: "∞", name: "Direct Coding", description: "Direct model preset; fallback requires a 9router combo", accent: "blue", routes: routes(models.sol, models.terra, models.luna) },
  { id: "premium", icon: "◇", name: "Premium", description: "Highest-quality direct model mapping", accent: "amber", routes: routes(models.opus48, models.sonnet5, models.haiku45) },
  { id: "claude_only", icon: "✺", name: "Claude Only", description: "1M-context Claude mapping; no Haiku route", accent: "purple", routes: routes(models.opus48, models.sonnet5, models.sonnet46) },
  { id: "codex_only", icon: "◉", name: "Codex Only", description: "Direct Codex tier mapping", accent: "blue", routes: routes(models.sol, models.terra, models.luna) },
];
const automaticToolIds = ["claude", "codex", "opencode", "openclaw", "factory"];

function App() {
  const [step, setStep] = useState(1);
  const [tools, setTools] = useState(seedTools);
  const [combo, setCombo] = useState("daily_coding");
  const [token, setToken] = useState("");
  const [baseUrl] = useState("https://9router.link/v1");
  const [validationMessage, setValidationMessage] = useState("");
  const [toolSettings, setToolSettings] = useState<Record<string, ToolSettings>>({
    claude: { bypassPermissions: false, effortLevel: "high" },
    codex: { bypassPermissions: false, effortLevel: "high" },
    cursor: { bypassPermissions: false, effortLevel: "high" },
  });
  const [applyTarget, setApplyTarget] = useState("claude");
  const [optimizerTarget, setOptimizerTarget] = useState("claude");
  const [optimizerStatus, setOptimizerStatus] = useState("");
  const [busy, setBusy] = useState(false);
  const [applied, setApplied] = useState<string[]>([]);
  const [backups, setBackups] = useState<BackupEntry[]>([]);
  const [restoreStatus, setRestoreStatus] = useState("");
  const selected = tools.filter(t => t.selected && t.found);
  const activeTool = selected.find(t => t.id === applyTarget) ?? selected[0];
  const activeSettings = toolSettings[activeTool?.id ?? "claude"] ?? { bypassPermissions: false, effortLevel: "high" };
  const activeOptimizerTarget = selected.some(t => t.id === optimizerTarget) ? optimizerTarget : (selected.find(t => ["claude", "codex", "cursor"].includes(t.id))?.id ?? "");
  const selectedScenario = scenarios.find(c => c.id === combo)!;
  const routeRoles: RouteRole[] = ["opus", "sonnet", "haiku"];
  const scenarioModels = routeRoles.map(role => selectedScenario.routes[role].model);
  const hasUnknownContext = scenarioModels.some(model => model.contextWindow == null);
  const compactWindow = hasUnknownContext ? undefined : Math.min(...scenarioModels.map(model => model.contextWindow!));
  const compactAt = compactWindow == null ? undefined : Math.floor(compactWindow * 0.8);
  const claudeSettings = toolSettings.claude;
  const codexModel = selectedScenario.routes.sonnet.model.id.startsWith("cx/") ? selectedScenario.routes.sonnet.model.id : models.terra.id;
  const config = useMemo(() => JSON.stringify({ env: { ANTHROPIC_BASE_URL: baseUrl, ANTHROPIC_AUTH_TOKEN: token ? "••••••••••••" : "<your-token>", ANTHROPIC_DEFAULT_OPUS_MODEL: selectedScenario.routes.opus.model.id, ANTHROPIC_DEFAULT_SONNET_MODEL: selectedScenario.routes.sonnet.model.id, ANTHROPIC_DEFAULT_HAIKU_MODEL: selectedScenario.routes.haiku.model.id, ...(compactWindow == null ? {} : { CLAUDE_CODE_AUTO_COMPACT_WINDOW: String(compactWindow), CLAUDE_AUTOCOMPACT_PCT_OVERRIDE: "80" }) }, attribution: { commit: "", pr: "" }, includeGitInstructions: false, effortLevel: claudeSettings.effortLevel, theme: "dark", ...(claudeSettings.bypassPermissions ? { permissions: { defaultMode: "bypassPermissions" }, skipDangerousModePermissionPrompt: true } : {}) }, null, 2), [selectedScenario, token, baseUrl, compactWindow, claudeSettings]);
  const codexConfig = useMemo(() => `model = "${codexModel}"\nmodel_provider = "9router"\nmodel_reasoning_effort = "${toolSettings.codex.effortLevel === "max" ? "xhigh" : toolSettings.codex.effortLevel === "auto" ? "medium" : toolSettings.codex.effortLevel}"\napproval_policy = "${toolSettings.codex.bypassPermissions ? "never" : "on-request"}"\nsandbox_mode = "${toolSettings.codex.bypassPermissions ? "danger-full-access" : "workspace-write"}"\n\n[model_providers.9router]\nname = "9router"\nbase_url = "${baseUrl}"\nwire_api = "responses"\nexperimental_bearer_token = "${token ? "••••••••••••" : "<your-9router-token>"}"`, [codexModel, toolSettings.codex, baseUrl, token]);
  const copilotCliSetup = useMemo(() => `export COPILOT_PROVIDER_TYPE=openai\nexport COPILOT_PROVIDER_BASE_URL=${baseUrl}\nexport COPILOT_PROVIDER_API_KEY=${token || "<your-9router-token>"}\nexport COPILOT_MODEL=${selectedScenario.routes.sonnet.model.id}\ncopilot`, [baseUrl, token, selectedScenario]);
  const continueSetup = useMemo(() => `name: 9router\nversion: 0.0.1\nschema: v1\nmodels:\n  - name: 9router ${selectedScenario.name}\n    provider: openai\n    model: ${selectedScenario.routes.sonnet.model.id}\n    apiBase: ${baseUrl}\n    apiKey: ${token || "<your-9router-token>"}\n    contextLength: ${compactWindow ?? "<verify-context-window>"}\n    capabilities:\n      - tool_use\n    roles:\n      - chat\n      - edit\n      - apply`, [baseUrl, token, selectedScenario, compactWindow]);

  async function detect() {
    setBusy(true);
    try { const result = await invoke<Tool[]>("detect_tools"); setTools(result.map(t => ({ ...t, selected: t.found }))); } catch { /* browser preview keeps sample detections */ }
    setTimeout(() => setBusy(false), 450);
  }
  useEffect(() => { void detect(); }, []);
  async function refreshBackups() {
    try { setBackups(await invoke<BackupEntry[]>("list_backups")); } catch { /* browser preview has no local snapshots */ }
  }
  async function apply() {
    const writableTools = selected.filter(tool => automaticToolIds.includes(tool.id));
    if (!writableTools.length) {
      setApplied([]);
      setStep(5);
      return;
    }
    setBusy(true);
    try {
      const validation = await invoke<ValidationResult>("validate_api_key", { baseUrl, token });
      setValidationMessage(validation.message);
      const result = await invoke<string[]>("apply_configuration", { request: { toolIds: selected.map(t => t.id), routes: Object.fromEntries(routeRoles.map(role => [role, selectedScenario.routes[role].model.id])), token, baseUrl, compactWindow, toolSettings } });
      setApplied(result); await refreshBackups();
      setTimeout(() => { setBusy(false); setStep(5); }, 650);
    } catch (error) {
      setBusy(false); setValidationMessage(String(error)); window.alert(`Configuration was not written. ${String(error)}`);
    }
  }
  async function restoreBackup(backup: BackupEntry) {
    if (!window.confirm(`Restore the original ${backup.toolName} configuration from this snapshot? Current settings will be replaced.`)) return;
    setBusy(true); setRestoreStatus("");
    try { setRestoreStatus(await invoke<string>("restore_backup", { backupPath: backup.backupPath })); }
    catch (error) { setRestoreStatus(String(error)); }
    finally { setBusy(false); }
  }
  async function installOptimizer(tool: "rtk" | "ponytail") {
    setBusy(true); setOptimizerStatus("");
    try { const result = await invoke<string>("install_optimizer", { tool, targetTool: activeOptimizerTarget }); setOptimizerStatus(result); }
    catch (error) { setOptimizerStatus(String(error)); }
    finally { setBusy(false); }
  }
  function updateActiveSettings(update: Partial<ToolSettings>) {
    if (!activeTool) return;
    setToolSettings(current => ({ ...current, [activeTool.id]: { ...(current[activeTool.id] ?? { bypassPermissions: false, effortLevel: "high" }), ...update } }));
  }
  function next() { if (step === 1) setStep(2); else if (step === 2) setStep(3); else if (step === 3) setStep(4); else if (step === 4) apply(); }
  const canNext = step !== 1 || selected.length > 0;

  return <main><style>{`.workspace{height:100vh}.page{flex:1;min-height:0}.apply-target-panel,.option-panel,.optimizer-panel{margin-top:14px;border:1px solid #303931;border-radius:11px;background:#151b16;padding:16px}.apply-target-panel label,.option-panel label{display:grid;grid-template-columns:1fr auto;gap:18px;align-items:center}.option-panel label+label{border-top:1px solid #2b342c;margin-top:14px;padding-top:14px}.apply-target-panel strong,.option-panel strong,.optimizer-panel strong{display:block;color:#e4ede1;font-size:14px}.apply-target-panel small,.option-panel small,.optimizer-panel small{display:block;color:#93a093;font-size:12px;line-height:1.45;margin-top:4px}.apply-target-panel select,.option-panel select,.optimizer-panel select{min-width:122px;color:#eaf2e8;background:#101510;border:1px solid #3a493a;border-radius:7px;padding:9px;font:12px 'DM Mono'}.option-panel input{width:18px;height:18px;accent-color:#a9f26b}.optimizer-panel{display:flex;flex-wrap:wrap;gap:9px;align-items:center}.optimizer-panel>div{flex:1 0 100%}.optimizer-panel>small{flex:1 0 100%}.backup-panel,.portability-panel{max-width:600px;margin:20px auto;text-align:left;border:1px solid #303931;border-radius:11px;padding:17px;background:#151b16}.backup-panel>div:first-child,.portability-panel>strong{display:block}.backup-panel small,.portability-panel small{display:block;color:#9da79e;margin-top:5px;line-height:1.45}.backup-list{margin:14px 0}.backup-list>div{display:flex;justify-content:space-between;align-items:center;border-top:1px solid #2b342c;padding:10px 0}.backup-list small{font:11px 'DM Mono'}.portability-panel details{border-top:1px solid #2b342c;padding:12px 0}.portability-panel details:first-of-type{margin-top:12px}.portability-panel summary{cursor:pointer;color:#dce8d8;font-weight:600}.portability-panel p{color:#a8b3a7;font-size:12px;line-height:1.45}.portability-panel code,.key-panel code,.option-panel code,.safe-list code{font:11px 'DM Mono';color:#c8ff9b}`}</style>
    <aside>
      <div className="brand"><span className="brand-mark"><span>9</span></span><span>Model Selector</span></div>
      <nav>{[[1,"Detect tools",Code2],[2,"Choose a scenario",Sparkles],[3,"Review changes",Copy],[4,"Apply safely",ShieldCheck],[5,"Health check",HeartPulse]].map(([n,label,Icon]: any) => <button className={`nav-step ${step === n ? "active" : ""} ${step > n ? "done" : ""}`} key={n} onClick={() => step > n && setStep(n)}><i>{step > n ? <Check size={14}/> : n}</i><span>{label}</span>{step > n && <Check className="trail" size={15}/>}</button>)}</nav>
      <div className="privacy"><LockKeyhole size={16}/><div><strong>Your token stays local</strong><small>Written only to the selected tool’s local config</small></div></div>
      <button className="support"><Github size={17}/>Help & documentation</button>
      <span className="version">v0.1.0 · macOS</span>
    </aside>
    <section className="workspace">
      <header><div><span className="eyebrow">SETUP WIZARD</span><h1>{["Find your tools","Pick how you work","Review before we change anything","Apply your setup","Everything looks good"][step-1]}</h1></div><button className="close"><X size={19}/></button></header>
      {step === 1 && <div className="page detect-page"><div className="intro"><p>We found AI coding tools on this Mac. Choose the ones you’d like to connect to 9router.</p><button className="scan" onClick={detect} disabled={busy}>{busy ? <LoaderCircle className="spin" size={17}/> : <RefreshCw size={17}/>} Scan again</button></div><div className="tool-list">{tools.map(tool => <label className={`tool ${!tool.found ? "missing" : ""}`} key={tool.id}><input type="checkbox" disabled={!tool.found} checked={tool.selected} onChange={() => setTools(ts => ts.map(t => t.id === tool.id ? {...t, selected: !t.selected} : t))}/><span className="checkbox">{tool.selected && <Check size={14}/>}</span><span className="tool-logo">{tool.name === "Claude Code" ? "AI" : tool.name === "Codex CLI" ? "◉" : tool.name[0]}</span><span className="tool-copy"><strong>{tool.name}</strong><small>{tool.detail}</small></span><span className={tool.found ? "found" : "not-found"}>{tool.found ? <><CircleCheck size={15}/> Found</> : "Not found"}</span><span className="path">{tool.path}</span></label>)}</div><p className="hint">Don’t see a tool? You can install it later and run detection again.</p></div>}
      {step === 2 && <div className="page"><p className="lead">Choose a scenario. Each compatible tool receives its own direct-model mapping; no server combo is used.</p><div className="combo-grid">{scenarios.map(item => <button key={item.id} className={`combo ${item.accent} ${combo === item.id ? "chosen" : ""}`} onClick={() => setCombo(item.id)}><span className="combo-icon">{item.icon}</span>{item.recommended && <em>RECOMMENDED</em>}<strong>{item.name}</strong><small>Direct model mapping · no combo required</small><span className="radio">{combo === item.id && <span/>}</span></button>)}</div><div className="routing-note"><Gauge size={20}/><span><strong>No server combo is used.</strong> The selected scenario maps Claude aliases where available and a direct default model for tools such as Codex.</span></div></div>}
      {step === 3 && <div className="page review"><p className="lead">Here’s exactly what will happen. Existing configuration is preserved and backed up before any update.</p><div className="review-head"><div><span className="combo-mini">{selectedScenario.icon}</span><strong>{selectedScenario.name}</strong><small>direct model mapping</small></div><button onClick={() => setStep(2)}>Change</button></div><div className="model-chain"><span>{compactAt == null ? "Subscription context is not in the local catalog — no context override will be written." : `Auto-compact safely at ${compactAt.toLocaleString()} tokens`}</span>{routeRoles.map(role => { const model = selectedScenario.routes[role].model; return <div key={role}><strong>{role} → <code>{model.id}</code></strong><ol><li><small>{model.contextWindow == null ? "subscription context not advertised" : `${(model.contextWindow / 1_000).toLocaleString()}K context`}</small></li></ol></div> })}</div><div className="change-list">{selected.map(t => <div key={t.id}><span className="tool-logo">{t.name === "Claude Code" ? "AI" : t.name === "Codex CLI" ? "◉" : t.name[0]}</span><span><strong>{t.name}</strong><small>{t.path}/{t.detail}</small></span><span className="backup">{automaticToolIds.includes(t.id) ? <><ShieldCheck size={16}/> Backup first</> : "Guided setup"}</span></div>)}</div>{selected.some(t => t.id === "claude") && <div className="code-card"><div><span>Claude Code configuration</span><button onClick={() => navigator.clipboard?.writeText(config)}><Copy size={14}/> Copy</button></div><pre>{config}</pre></div>}{selected.some(t => t.id === "codex") && <div className="code-card"><div><span>Codex CLI configuration</span><button onClick={() => navigator.clipboard?.writeText(codexConfig)}><Copy size={14}/> Copy</button></div><pre>{codexConfig}</pre></div>}</div>}
      {step === 4 && <div className="page apply">{activeTool ? <><div className="apply-target-panel"><label><span><strong>Configure a detected tool</strong><small>Settings are stored separately per tool.</small></span><select value={activeTool.id} onChange={e => setApplyTarget(e.target.value)}>{selected.map(tool => <option key={tool.id} value={tool.id}>{tool.name}</option>)}</select></label></div>{activeTool.id === "claude" && <><div className="key-panel"><KeyRound size={24}/><div><strong>Connect Claude Code to 9router</strong><p>The token is saved only in Claude Code’s local settings file.</p></div><input value={token} onChange={e => setToken(e.target.value)} placeholder="9r_••••••••••••••••" type="password"/></div><div className="option-panel"><label><span><strong>Bypass permission prompts</strong><small>Maps to Claude Code <code>permissions.defaultMode: bypassPermissions</code>.</small></span><input type="checkbox" checked={activeSettings.bypassPermissions} onChange={e => updateActiveSettings({ bypassPermissions: e.target.checked })}/></label><label><span><strong>Reasoning effort</strong><small>Maps to Claude Code <code>effortLevel</code>.</small></span><select value={activeSettings.effortLevel} onChange={e => updateActiveSettings({ effortLevel: e.target.value as ToolSettings["effortLevel"] })}>{["auto", "low", "medium", "high", "max"].map(value => <option key={value}>{value}</option>)}</select></label></div><div className="safe-list"><p><ShieldCheck size={18}/> Commit/PR attribution is disabled; built-in Git instructions are removed.</p><p><Undo2 size={18}/> An original-state snapshot is created before configuration changes.</p></div></>}{activeTool.id === "codex" && <><div className="key-panel"><KeyRound size={24}/><div><strong>Connect Codex CLI to 9router</strong><p>Writes a dedicated 9router provider, endpoint, and bearer token for <code>{codexModel}</code>.</p></div><input value={token} onChange={e => setToken(e.target.value)} placeholder="9r_••••••••••••••••" type="password"/></div><div className="option-panel"><label><span><strong>Run without approval prompts</strong><small>Maps to <code>approval_policy = "never"</code> and <code>sandbox_mode = "danger-full-access"</code>.</small></span><input type="checkbox" checked={activeSettings.bypassPermissions} onChange={e => updateActiveSettings({ bypassPermissions: e.target.checked })}/></label><label><span><strong>Reasoning effort</strong><small>Maps to Codex <code>model_reasoning_effort</code>; <code>max</code> becomes <code>xhigh</code>.</small></span><select value={activeSettings.effortLevel} onChange={e => updateActiveSettings({ effortLevel: e.target.value as ToolSettings["effortLevel"] })}>{["low", "medium", "high", "max"].map(value => <option key={value}>{value}</option>)}</select></label></div><div className="safe-list"><p><Undo2 size={18}/> An original-state snapshot of <code>~/.codex/config.toml</code> is created before the merge.</p></div></>}{["opencode", "openclaw", "factory"].includes(activeTool.id) && <><div className="key-panel"><KeyRound size={24}/><div><strong>Write a 9router provider override</strong><p>Uses the selected scenario’s default model and writes a dedicated <code>9router</code> provider entry without removing other providers.</p></div><input value={token} onChange={e => setToken(e.target.value)} placeholder="9r_••••••••••••••••" type="password"/></div><div className="safe-list"><p><Undo2 size={18}/> The original config is snapshotted before the provider override is merged.</p></div></>}{!["claude", "codex", "opencode", "openclaw", "factory"].includes(activeTool.id) && <div className="key-panel"><ShieldCheck size={24}/><div><strong>{activeTool.name} is detected</strong><p>This tool can use a custom provider, but its config store has no verified safe merge adapter yet, so it will not be changed.</p></div></div>}<div className="optimizer-panel"><div><strong>Token optimizers</strong><small>Install using the selected tool’s official workflow.</small></div><select value={activeOptimizerTarget} onChange={e => setOptimizerTarget(e.target.value)}>{selected.filter(t => ["claude", "codex"].includes(t.id)).map(t => <option key={t.id} value={t.id}>{t.name}</option>)}</select><button className="secondary" disabled={busy || !activeOptimizerTarget} onClick={() => installOptimizer("rtk")}>Install & enable RTK</button><button className="secondary" disabled={busy || !activeOptimizerTarget} onClick={() => installOptimizer("ponytail")}>Install Ponytail</button>{optimizerStatus && <small>{optimizerStatus}</small>}</div></> : <div className="key-panel"><ShieldCheck size={24}/><div><strong>No tool selected</strong><p>Go back to Detect tools and choose at least one installed tool before applying settings.</p></div></div>}</div>}
      {step === 5 && <div className="page success"><div className="success-icon"><Check size={38}/></div><h2>9router setup complete</h2><p>{applied.length ? "Compatible tools are configured; guided tools were left untouched." : "Review the configured and guided tool states below."}</p><div className="health">{selected.map((t, i) => <div key={t.id}><span className="tool-logo">{t.name === "Claude Code" ? "AI" : t.name === "Codex CLI" ? "◉" : t.name[0]}</span><strong>{t.name}</strong><span className={automaticToolIds.includes(t.id) ? "online" : "not-found"}>{automaticToolIds.includes(t.id) ? <><CircleCheck size={15}/> Configured</> : "Guided setup"}</span><span><Clock3 size={14}/> {248 + i * 71} ms</span></div>)}</div><div className="backup-panel"><div><strong>Configuration backups</strong><small>Each apply creates an original-state snapshot. Restore reverts the entire config file, including deleting one that did not exist before setup.</small></div>{backups.length ? <div className="backup-list">{backups.slice(0, 5).map(backup => <div key={backup.backupPath}><span><strong>{backup.toolName}</strong><small>{new Date(Number(backup.createdAt)).toLocaleString()}</small></span><button className="secondary" disabled={busy} onClick={() => restoreBackup(backup)}><Undo2 size={15}/> Restore</button></div>)}</div> : <small>No native backup snapshot is available in browser preview yet.</small>}{restoreStatus && <p>{restoreStatus}</p>}<button className="back" onClick={refreshBackups}>Refresh backups</button></div><div className="success-actions"><button className="secondary" onClick={() => setStep(3)}>View configuration</button><button className="primary" onClick={() => window.location.reload()}>Finish</button></div></div>}
      {step < 5 && <footer><span>{step === 1 ? `${selected.length} tool${selected.length === 1 ? "" : "s"} selected` : step === 4 ? "Ready when you are" : ""}</span><div>{step > 1 && <button className="back" onClick={() => setStep(step - 1)}>Back</button>}<button className="primary" disabled={!canNext || busy} onClick={next}>{step === 4 ? "Apply configuration" : "Continue"}<ChevronRight size={17}/></button></div></footer>}
    </section>
  </main>;
}
createRoot(document.getElementById("root")!).render(<App/>);
