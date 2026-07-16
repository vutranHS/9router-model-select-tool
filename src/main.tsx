import React, { useEffect, useMemo, useState } from "react";
import { createRoot } from "react-dom/client";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { Check, ChevronRight, CircleCheck, Clock3, Code2, Copy, Database, Eye, Github, HeartPulse, KeyRound, Layers3, LoaderCircle, LockKeyhole, RefreshCw, ShieldCheck, Sparkles, Undo2, X } from "lucide-react";
import "./styles.css";

type Tool = { id: string; name: string; detail: string; path: string; found: boolean; selected: boolean };
type GatewayModel = { id: string; ownedBy?: string; kind?: string; maxInputTokens?: number; maxOutputTokens?: number; limitsSource?: string };
type ModelLimits = { maxInputTokens: number; maxOutputTokens: number };
type CapabilitySkill = { id: string; name: string; description: string; modelGroup: string; modelKind?: string; sourceUrl: string };
type GatewayCatalog = { chatModels: GatewayModel[]; imageModels: GatewayModel[]; webModels: GatewayModel[]; ttsModels: GatewayModel[]; sttModels: GatewayModel[]; embeddingModels: GatewayModel[]; imageToTextModels: GatewayModel[]; skills: CapabilitySkill[] };
type ToolSettings = { bypassPermissions: boolean; effortLevel: "low" | "medium" | "high" | "xhigh" | "max" };
type BackupEntry = { toolId: string; toolName: string; originalPath: string; backupPath: string; createdAt: string };
type ValidationResult = { valid: boolean; modelCount: number; message: string };
type ModelInfoResult = { modelId: string; details: unknown };
type ImageRouteTestResult = { modelId: string; status: "ready" | "unauthorized" | "unavailable" | "error"; message: string };
type ClaudeModelMapping = { defaultModel: string; opus: string; sonnet: string; haiku: string };

const seedTools: Tool[] = [
  { id: "claude", name: "Claude Code", detail: "settings.json", path: "~/.claude", found: true, selected: false },
  { id: "codex", name: "Codex CLI", detail: "9router profile", path: "~/.codex", found: true, selected: false },
  { id: "cursor", name: "Cursor", detail: "Settings UI", path: "Not detected", found: false, selected: false },
  { id: "cline", name: "Cline", detail: "VS Code extension", path: "Not detected", found: false, selected: false },
  { id: "roo", name: "Roo Code", detail: "VS Code extension", path: "Not detected", found: false, selected: false },
  { id: "kilo", name: "Kilo Code", detail: "VS Code extension", path: "Not detected", found: false, selected: false },
  { id: "vibe", name: "Mistral Vibe CLI", detail: "config.toml + .env", path: "Not detected", found: false, selected: false },
  { id: "continue", name: "Continue", detail: "VS Code extension", path: "Not detected", found: false, selected: false },
  { id: "pi", name: "Pi", detail: "models.json", path: "Not detected", found: false, selected: false },
  { id: "hermes", name: "Hermes", detail: "config.yaml + .env", path: "Not detected", found: false, selected: false },
  { id: "copilot-vscode", name: "GitHub Copilot (VS Code)", detail: "VS Code extension", path: "Not detected", found: false, selected: false },
  { id: "copilot-cli", name: "GitHub Copilot CLI", detail: "copilot command", path: "Not detected", found: false, selected: false },
  { id: "opencode", name: "OpenCode", detail: "opencode command", path: "Not detected", found: false, selected: false },
  { id: "openclaw", name: "OpenClaw", detail: "openclaw command", path: "Not detected", found: false, selected: false },
  { id: "factory", name: "Factory Droid", detail: "droid command", path: "Not detected", found: false, selected: false },
];
const automaticToolIds = ["claude", "codex", "opencode", "openclaw", "factory", "pi"];
const rtkSupportedToolIds = ["claude", "codex", "cursor", "cline", "roo", "kilo", "pi", "hermes", "copilot-vscode", "copilot-cli", "opencode", "openclaw", "factory", "gemini", "windsurf", "antigravity"];
const rtkProjectScopedToolIds = ["cline", "roo", "kilo", "windsurf", "antigravity"];
const ponytailSupportedToolIds = ["claude", "codex", "opencode", "copilot-cli", "pi", "gemini", "antigravity", "hermes", "openclaw"];

function App() {
  const [step, setStep] = useState(1);
  const [tools, setTools] = useState(seedTools);
  const [optimizerTools, setOptimizerTools] = useState<Tool[]>([]);
  const [baseUrl, setBaseUrl] = useState("https://9router.link/v1");
  const [token, setToken] = useState("");
  const [catalog, setCatalog] = useState<GatewayCatalog | null>(null);
  const [toolModels, setToolModels] = useState<Record<string, string>>({});
  const [toolModelPools, setToolModelPools] = useState<Record<string, string[]>>({});
  const [modelLimits, setModelLimits] = useState<Record<string, ModelLimits>>({});
  const [claudeModels, setClaudeModels] = useState<ClaudeModelMapping>({ defaultModel: "", opus: "", sonnet: "", haiku: "" });
  const [skillRoutes, setSkillRoutes] = useState<Record<string, string[]>>({});
  const [localCapabilities, setLocalCapabilities] = useState({ cloakbrowser: true, computerUse: false, indieAppShipping: false, reverseSkill: false, superpowers: false, gitGuardian: false });
  const [imageTests, setImageTests] = useState<Record<string, ImageRouteTestResult>>({});
  const [modelInfo, setModelInfo] = useState<ModelInfoResult | null>(null);
  const [toolSettings, setToolSettings] = useState<Record<string, ToolSettings>>({});
  const [applyTarget, setApplyTarget] = useState("");
  const [optimizerTarget, setOptimizerTarget] = useState("");
  const [optimizerWorkspace, setOptimizerWorkspace] = useState("");
  const [busy, setBusy] = useState(false);
  const [message, setMessage] = useState("");
  const [applied, setApplied] = useState<string[]>([]);
  const [backups, setBackups] = useState<BackupEntry[]>([]);

  const foundTools = tools.filter(tool => tool.found);
  const selected = tools.filter(tool => tool.found && tool.selected);
  const activeTool = selected.find(tool => tool.id === applyTarget) ?? selected[0];
  const installedOptimizerTools = optimizerTools.filter(tool => tool.found);
  const activeOptimizerTool = installedOptimizerTools.find(tool => tool.id === optimizerTarget) ?? installedOptimizerTools[0];
  const activeSettings = toolSettings[activeTool?.id ?? ""] ?? { bypassPermissions: false, effortLevel: "medium" };
  const selectedSkills = catalog?.skills.filter(skill => (skillRoutes[skill.id] ?? []).length > 0) ?? [];
  const selectedModelIds = useMemo(() => [...new Set(selected.flatMap(tool => tool.id === "claude"
    ? [claudeModels.defaultModel, claudeModels.opus, claudeModels.sonnet, claudeModels.haiku]
    : [toolModels[tool.id]]).filter(Boolean))], [selected, claudeModels, toolModels]);
  const allModelLimitsConfigured = selectedModelIds.every(modelId => {
    const limits = resolvedModelLimits(modelId);
    return limits.maxInputTokens > 0 && limits.maxOutputTokens > 0;
  });
  const allToolsConfigured = allModelLimitsConfigured && selected.every(tool => tool.id === "claude"
    ? Boolean(claudeModels.defaultModel && claudeModels.opus && claudeModels.sonnet && claudeModels.haiku)
    : Boolean(toolModels[tool.id]));

  function candidatesForTool(tool: Tool) {
    if (!catalog) return [];
    // The gateway is the source of truth. A tool may intentionally use a model
    // from another provider family through 9router, e.g. Claude Code → cx/gpt-5.6-sol.
    return catalog.chatModels;
  }
  function candidatesForSkill(skill: CapabilitySkill) {
    if (!catalog) return [];
    const models = skill.modelGroup === "image" ? catalog.imageModels
      : skill.modelGroup === "web" ? catalog.webModels.filter(model => !skill.modelKind || model.kind === skill.modelKind)
      : skill.modelGroup === "tts" ? catalog.ttsModels
      : skill.modelGroup === "stt" ? catalog.sttModels
      : skill.modelGroup === "embedding" ? catalog.embeddingModels
      : catalog.chatModels;
    return models;
  }
  function toolLogo(tool: Tool) { return tool.name === "Claude Code" ? "AI" : tool.name === "Codex CLI" ? "◉" : tool.name[0]; }
  function resolvedModelLimits(modelId: string): ModelLimits {
    const discovered = catalog?.chatModels.find(model => model.id === modelId);
    return modelLimits[modelId] ?? {
      maxInputTokens: discovered?.maxInputTokens ?? 0,
      maxOutputTokens: discovered?.maxOutputTokens ?? 0,
    };
  }
  function updateModelLimit(modelId: string, field: keyof ModelLimits, rawValue: string) {
    const value = Math.max(0, Number.parseInt(rawValue, 10) || 0);
    setModelLimits(current => ({
      ...current,
      [modelId]: { ...resolvedModelLimits(modelId), ...(current[modelId] ?? {}), [field]: value },
    }));
  }

  async function detect() {
    setBusy(true);
    try {
      const [result, optimizerResult] = await Promise.all([
        invoke<Tool[]>("detect_tools"),
        invoke<Tool[]>("detect_optimizer_tools"),
      ]);
      setTools(result.map(tool => ({ ...tool, selected: false })));
      setOptimizerTools(optimizerResult.map(tool => ({ ...tool, selected: false })));
      setOptimizerTarget(current => optimizerResult.some(tool => tool.found && tool.id === current) ? current : optimizerResult.find(tool => tool.found)?.id ?? "");
    }
    catch { setMessage("Tool scan is only available in the desktop app."); }
    finally { setBusy(false); }
  }
  useEffect(() => { void detect(); }, []);

  async function explore() {
    setBusy(true); setMessage("");
    try {
      const result = await invoke<GatewayCatalog>("discover_gateway", { baseUrl, token });
      setCatalog(result);
      setModelLimits(current => {
        const next = { ...current };
        result.chatModels.forEach(model => {
          if (!next[model.id] && model.maxInputTokens && model.maxOutputTokens) {
            next[model.id] = {
              maxInputTokens: model.maxInputTokens,
              maxOutputTokens: model.maxOutputTokens,
            };
          }
        });
        return next;
      });
      setToolModels(current => {
        const next = { ...current };
        selected.forEach(tool => {
          const models = result.chatModels;
          if (!models.some(model => model.id === next[tool.id])) next[tool.id] = models[0]?.id ?? "";
        });
        return next;
      });
      setToolModelPools(current => {
        const next = { ...current };
        selected.forEach(tool => {
          if (!next[tool.id]?.length) next[tool.id] = result.chatModels[0] ? [result.chatModels[0].id] : [];
        });
        return next;
      });
      setClaudeModels(current => ({
        defaultModel: current.defaultModel || result.chatModels[0]?.id || "",
        opus: current.opus || result.chatModels[0]?.id || "",
        sonnet: current.sonnet || result.chatModels[0]?.id || "",
        haiku: current.haiku || result.chatModels[0]?.id || "",
      }));
      setMessage(`Gateway verified · ${result.chatModels.length} chat model(s) discovered.`);
    } catch (error) { setCatalog(null); setMessage(String(error)); }
    finally { setBusy(false); }
  }
  function toggleSkill(skill: CapabilitySkill) {
    const current = skillRoutes[skill.id] ?? [];
    if (current.length) { setSkillRoutes(routes => ({ ...routes, [skill.id]: [] })); return; }
    const first = candidatesForSkill(skill)[0]?.id;
    if (!first) { setMessage(`No enabled ${skill.name} models were found on this gateway.`); return; }
    setSkillRoutes(routes => ({ ...routes, [skill.id]: [first] }));
  }
  function toggleSkillModel(skillId: string, modelId: string) {
    setSkillRoutes(routes => {
      const current = routes[skillId] ?? [];
      return { ...routes, [skillId]: current.includes(modelId) ? current.filter(id => id !== modelId) : [...current, modelId] };
    });
  }
  function updateActiveSettings(update: Partial<ToolSettings>) {
    if (!activeTool) return;
    setToolSettings(current => ({ ...current, [activeTool.id]: { ...(current[activeTool.id] ?? activeSettings), ...update } }));
  }
  async function inspectImageModel(modelId: string) {
    setBusy(true); setMessage("");
    try {
      const result = await invoke<ModelInfoResult>("get_model_info", { baseUrl, token, modelId });
      setModelInfo(result);
    } catch (error) { setMessage(String(error)); }
    finally { setBusy(false); }
  }
  async function testImageModel(modelId: string) {
    if (!window.confirm(`Run a real image-generation request for ${modelId}? This can consume provider quota or incur a charge.`)) return;
    setBusy(true); setMessage("");
    try {
      const result = await invoke<ImageRouteTestResult>("test_image_route", { baseUrl, token, modelId });
      setImageTests(current => ({ ...current, [modelId]: result }));
    } catch (error) { setImageTests(current => ({ ...current, [modelId]: { modelId, status: "error", message: String(error) } })); }
    finally { setBusy(false); }
  }
  async function refreshBackups() { try { setBackups(await invoke<BackupEntry[]>("list_backups")); } catch { /* desktop only */ } }
  async function chooseOptimizerWorkspace() {
    const selectedPath = await open({ directory: true, multiple: false, title: "Choose project workspace for RTK rules" });
    if (typeof selectedPath === "string") setOptimizerWorkspace(selectedPath);
  }
  async function installOptimizer(tool: "rtk" | "ponytail") {
    if (!activeOptimizerTool) return;
    if (tool === "rtk" && rtkProjectScopedToolIds.includes(activeOptimizerTool.id) && !optimizerWorkspace) {
      setMessage("Choose the project workspace where RTK should write this tool's rules.");
      return;
    }
    setBusy(true); setMessage("");
    try {
      setMessage(await invoke<string>("install_optimizer", { tool, targetTool: activeOptimizerTool.id, workspacePath: optimizerWorkspace || null }));
    } catch (error) {
      setMessage(String(error));
    } finally {
      setBusy(false);
    }
  }
  async function apply() {
    if (!catalog || !allToolsConfigured) return;
    setBusy(true); setMessage("");
    try {
      const requiredModels = selectedModelIds;
      const selectedLimits = Object.fromEntries(requiredModels.map(modelId => [modelId, resolvedModelLimits(modelId)]));
      if (Object.entries(selectedLimits).some(([, limits]) => limits.maxInputTokens <= 0 || limits.maxOutputTokens <= 0)) {
        setMessage("Enter both max input and max output tokens for every selected model.");
        return;
      }
      const validation = await invoke<ValidationResult>("validate_api_key", { baseUrl, token, requiredModels });
      if (!validation.valid) throw new Error(validation.message);
      const fallback = catalog.chatModels[0]?.id ?? "";
      const routes = { defaultModel: fallback, opus: fallback, sonnet: fallback, haiku: fallback };
      const result = await invoke<string[]>("apply_configuration", { request: { toolIds: selected.map(tool => tool.id), routes, toolModels, toolModelPools, modelLimits: selectedLimits, claudeModels, token, baseUrl, compactWindow: null, codexContextWindow: null, toolSettings, cloakbrowserEnabled: localCapabilities.cloakbrowser, computerUseEnabled: localCapabilities.computerUse, indieAppShippingEnabled: localCapabilities.indieAppShipping, reverseSkillEnabled: localCapabilities.reverseSkill, superpowersEnabled: localCapabilities.superpowers, gitGuardianEnabled: localCapabilities.gitGuardian, capabilityRoutes: selectedSkills.map(skill => ({ skillId: skill.id, modelIds: skillRoutes[skill.id] })) } });
      setApplied(result); await refreshBackups(); setStep(6);
    } catch (error) { setMessage(`Nothing was written. ${String(error)}`); }
    finally { setBusy(false); }
  }
  async function restoreBackup(backup: BackupEntry) {
    if (!window.confirm(`Restore ${backup.toolName} from this snapshot?`)) return;
    setBusy(true); try { setMessage(await invoke<string>("restore_backup", { backupPath: backup.backupPath })); } catch (error) { setMessage(String(error)); } finally { setBusy(false); }
  }
  const canContinue = step === 1 ? selected.length > 0 : step === 2 ? Boolean(catalog && allToolsConfigured) : true;
  const titles = ["Find your tools", "Explore your gateway", "Choose capability routes", "Review your setup", "Apply safely", "Setup complete"];

  return <main><style>{`.workspace{height:100vh}.page{flex:1;min-height:0}.gateway-panel,.model-panel,.skill-card,.apply-target-panel,.option-panel,.backup-panel{border:1px solid #303931;border-radius:11px;background:#151b16;padding:16px}.gateway-panel{display:grid;grid-template-columns:1fr 1fr auto;gap:10px;align-items:end}.gateway-panel label,.model-panel label,.claude-mapping label{display:grid;gap:6px;color:#aab5aa;font-size:12px}.gateway-panel input,.gateway-panel select,.model-panel select,.claude-mapping select,.option-panel select{width:100%;min-width:0;color:#eaf2e8;background:#101510;border:1px solid #3a493a;border-radius:7px;padding:10px;font:12px 'DM Mono'}.claude-mapping{display:grid;grid-template-columns:repeat(3,minmax(0,1fr));gap:12px;margin-top:12px;padding:14px;border:1px solid #303931;border-radius:11px;background:#151b16}.catalog-summary{display:flex;flex-wrap:wrap;gap:8px;margin:15px 0}.catalog-summary span{border:1px solid #364237;border-radius:999px;padding:6px 9px;color:#b9c7b7;font:11px 'DM Mono'}.model-panel{margin-top:12px}.model-panel>div{display:grid;grid-template-columns:34px 1fr minmax(220px,1.25fr);align-items:center;gap:10px}.model-panel small,.skill-card small{display:block;color:#8e9b8e;font:11px 'DM Mono';margin-top:3px}.skill-grid{display:grid;grid-template-columns:1fr 1fr;gap:12px}.skill-card{position:relative;text-align:left;min-height:142px}.skill-card.disabled{opacity:.56}.skill-card>header{height:auto;padding:0;display:flex;gap:10px;align-items:center}.skill-card header strong{font-size:14px}.skill-card p{font-size:12px;color:#9eaa9e;line-height:1.45;margin:12px 0}.skill-card button{border:1px solid #3b493c;background:#101510;border-radius:7px;padding:7px 9px;color:#d7e3d5;font-size:12px}.skill-card button.active{background:#294221;border-color:#a9f26b;color:#caff9e}.route-models{display:flex;flex-wrap:wrap;gap:7px;margin-top:10px}.route-models button{font:10px 'DM Mono';max-width:100%;overflow:hidden;text-overflow:ellipsis}.route-models button.selected{border-color:#a9f26b;color:#caff9e}.image-route{border-top:1px solid #2b342c;margin-top:12px;padding-top:10px;display:grid;gap:7px}.image-route code{font:10px 'DM Mono';color:#c8ff9b;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.image-route>div{display:flex;flex-wrap:wrap;gap:6px;align-items:center}.route-status{font-size:10px;border-radius:999px;padding:4px 7px;border:1px solid #4b574b;color:#b9c6b7}.route-status.ready{border-color:#a9f26b;color:#baff91}.route-status.unavailable{border-color:#d6a866;color:#ffd48f}.route-status.unauthorized,.route-status.error{border-color:#dd6b61;color:#ffaaa3}.route-status.note{color:#99a599}.model-info{margin-top:12px;padding:10px;border-radius:8px;background:#101510;border:1px solid #303931;font:10px/1.45 'DM Mono';color:#b9c7b7;white-space:pre-wrap;max-height:150px;overflow:auto}.review-block{margin-top:13px}.review-block h3{font-size:13px;margin:0 0 8px;color:#cbd7c9}.review-list{border:1px solid #303931;border-radius:11px;overflow:hidden;background:#151b16}.review-list>div{display:flex;align-items:center;gap:10px;padding:12px 14px;border-bottom:1px solid #2b342c}.review-list>div:last-child{border-bottom:0}.review-list code{color:#c8ff9b;font:11px 'DM Mono';margin-left:auto;max-width:55%;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.notice{margin-top:13px;color:#aebaae;background:#192019;border:1px solid #2e3a2e;border-radius:9px;padding:11px 13px;font-size:12px;line-height:1.45}.apply-target-panel,.option-panel{margin-top:13px}.apply-target-panel label,.option-panel label{display:grid;grid-template-columns:1fr auto;align-items:center;gap:18px}.apply-target-panel strong,.option-panel strong{display:block;font-size:14px;color:#e6eee4}.apply-target-panel small,.option-panel small{display:block;margin-top:4px;font-size:12px;color:#93a093}.option-panel label+label{border-top:1px solid #2b342c;margin-top:14px;padding-top:14px}.option-panel input{width:18px;height:18px;accent-color:#a9f26b}.backup-panel{max-width:620px;margin:20px auto;text-align:left}.backup-list>div{display:flex;justify-content:space-between;align-items:center;border-top:1px solid #2b342c;padding:10px 0}.backup-list small{display:block;color:#8d998d;font:11px 'DM Mono'}`}</style>
    <aside><div className="brand"><span className="brand-mark"><span>9</span></span><span>Model Selector</span></div><nav>{[[1,"Detect tools",Code2],[2,"Explore gateway",Eye],[3,"Skills & routes",Layers3],[4,"Review",Copy],[5,"Apply safely",ShieldCheck],[6,"Health check",HeartPulse]].map(([number, label, Icon]: any) => <button className={`nav-step ${step === number ? "active" : ""} ${step > number ? "done" : ""}`} key={number} onClick={() => step > number && setStep(number)}><i>{step > number ? <Check size={14}/> : number}</i><span>{label}</span>{step > number && <Check className="trail" size={15}/>}</button>)}</nav><div className="privacy"><LockKeyhole size={16}/><div><strong>Your token stays local</strong><small>Used only for gateway discovery and selected tool configs</small></div></div><button className="support"><Github size={17}/>Help & documentation</button><span className="version">dynamic catalog · dev</span></aside>
    <section className="workspace"><header><div><span className="eyebrow">9ROUTER SETUP</span><h1>{titles[step - 1]}</h1></div><button className="close"><X size={19}/></button></header>
      {step === 1 && <div className="page detect-page"><div className="intro"><p>Only installed tools are shown. Pick the tools that should receive their own 9router model configuration.</p><button className="scan" onClick={detect} disabled={busy}>{busy ? <LoaderCircle className="spin" size={17}/> : <RefreshCw size={17}/>} Scan again</button></div><div className="tool-list">{foundTools.map(tool => <label className="tool" key={tool.id}><input type="checkbox" checked={tool.selected} onChange={() => setTools(current => current.map(item => item.id === tool.id ? { ...item, selected: !item.selected } : item))}/><span className="checkbox">{tool.selected && <Check size={14}/>}</span><span className="tool-logo">{toolLogo(tool)}</span><span className="tool-copy"><strong>{tool.name}</strong><small>{tool.detail}</small></span><span className="found"><CircleCheck size={15}/>Found</span><span className="path">{tool.path}</span></label>)}</div></div>}
      {step === 2 && <div className="page">
        <p className="lead">Enter a gateway once. The app asks 9router which models are currently enabled. Any listed model can be assigned to any tool.</p>
        <div className="gateway-panel">
          <label>Gateway base URL<input value={baseUrl} onChange={event => setBaseUrl(event.target.value)} placeholder="https://9router.link/v1"/></label>
          <label>API key<input value={token} onChange={event => setToken(event.target.value)} placeholder="9r_…" type="password"/></label>
          <button className="primary" disabled={busy || !token} onClick={explore}>{busy ? <LoaderCircle className="spin" size={16}/> : <Database size={16}/>} Explore</button>
        </div>
        {message && <div className="notice">{message}</div>}
        {catalog && <>
          <div className="catalog-summary"><span>{catalog.chatModels.length} chat</span><span>{catalog.imageModels.length} image</span><span>{catalog.webModels.length} web</span><span>{catalog.ttsModels.length} TTS</span><span>{catalog.sttModels.length} STT</span><span>{catalog.embeddingModels.length} embeddings</span></div>
          {selected.map(tool => {
            const models = candidatesForTool(tool);
            return <div className="model-panel" key={tool.id}>
              {tool.id === "claude" ? <>
                <div><span className="tool-logo">{toolLogo(tool)}</span><span><strong>Claude Code</strong><small>Choose its startup model, then map each Claude alias. Auto-compaction follows the smallest input window among all four routes.</small></span></div>
                <div className="gateway-panel" style={{ marginTop: 12 }}>
                  <label>Default model<select value={claudeModels.defaultModel} onChange={event => setClaudeModels(current => ({ ...current, defaultModel: event.target.value }))}>{models.map(model => <option key={model.id} value={model.id}>{model.id}</option>)}</select></label>
                  <label>Opus alias<select value={claudeModels.opus} onChange={event => setClaudeModels(current => ({ ...current, opus: event.target.value }))}>{models.map(model => <option key={model.id} value={model.id}>{model.id}</option>)}</select></label>
                  <label>Sonnet alias<select value={claudeModels.sonnet} onChange={event => setClaudeModels(current => ({ ...current, sonnet: event.target.value }))}>{models.map(model => <option key={model.id} value={model.id}>{model.id}</option>)}</select></label>
                  <label>Haiku alias<select value={claudeModels.haiku} onChange={event => setClaudeModels(current => ({ ...current, haiku: event.target.value }))}>{models.map(model => <option key={model.id} value={model.id}>{model.id}</option>)}</select></label>
                </div>
              </> : <div>
                <span className="tool-logo">{toolLogo(tool)}</span>
                <span><strong>{tool.name}</strong><small>{models.length ? `${models.length} enabled model(s) — one default model` : "No enabled model discovered"}</small></span>
                <label>Default model<select value={toolModels[tool.id] ?? ""} disabled={!models.length} onChange={event => setToolModels(current => ({ ...current, [tool.id]: event.target.value }))}>{models.map(model => <option key={model.id} value={model.id}>{model.id}</option>)}</select></label>
              </div>}
            </div>;
          })}
          {selectedModelIds.length > 0 && <section className="model-limit-panel">
            <div className="model-limit-heading">
              <div><strong>Model token limits & auto-compaction</strong><small>Max input drives a safe compaction target at roughly 80%. Max output is written where the target tool supports it. Known Codex and Claude values are prefilled but remain editable.</small></div>
            </div>
            <div className="model-limit-list">{selectedModelIds.map(modelId => {
              const model = catalog.chatModels.find(entry => entry.id === modelId);
              const limits = resolvedModelLimits(modelId);
              return <div className="model-limit-row" key={modelId}>
                <span><code>{modelId}</code><small>{model?.limitsSource ?? "Custom model · manual limits required"}{limits.maxInputTokens > 0 ? ` · compact around ${Math.floor(limits.maxInputTokens * 0.8).toLocaleString()}` : ""}</small></span>
                <label>Max input tokens<input type="number" min={1} step={1000} value={limits.maxInputTokens || ""} placeholder="e.g. 272000" onChange={event => updateModelLimit(modelId, "maxInputTokens", event.target.value)}/></label>
                <label>Max output tokens<input type="number" min={1} step={1000} value={limits.maxOutputTokens || ""} placeholder="e.g. 128000" onChange={event => updateModelLimit(modelId, "maxOutputTokens", event.target.value)}/></label>
              </div>;
            })}</div>
          </section>}
        </>}
      </div>}
      {step === 3 && <div className="page">
        <p className="lead">Gateway capabilities use the models enabled on 9router. Turn on only the routes you want, then pick one or more models. Every selected model is invoked; image generation creates one comparison image per model from the same prompt.</p>
        {!catalog ? <div className="notice">Explore a gateway first. The available models determine which capability routes can be selected.</div> : <div className="skill-grid">{catalog.skills.map(skill => { const models = candidatesForSkill(skill); const chosen = skillRoutes[skill.id] ?? []; return <article className={`skill-card ${!models.length ? "disabled" : ""}`} key={skill.id}><header><Sparkles size={18}/><div><strong>{skill.name}</strong><small>{models.length} enabled route(s)</small></div></header><p>{skill.description}</p><button disabled={!models.length} className={chosen.length ? "active" : ""} onClick={() => toggleSkill(skill)}>{chosen.length ? <><Check size={14}/> Selected for Apply</> : "Enable skill"}</button>{chosen.length > 0 && <><small className="capability-note">{chosen.length} provider{chosen.length === 1 ? "" : "s"} will run for each request{skill.id === "9router-image" ? " and produce separate comparison images" : ""}.</small><div className="route-models">{models.map(model => <button className={chosen.includes(model.id) ? "selected" : ""} key={model.id} onClick={() => toggleSkillModel(skill.id, model.id)}>{chosen.includes(model.id) ? "✓ " : "+ "}{model.id}</button>)}</div></>}{skill.id === "9router-image" && chosen.map(modelId => { const result = imageTests[modelId]; return <div className="image-route" key={modelId}><code>{modelId}</code><div><span className="route-status">Selected</span>{result ? <span className={`route-status ${result.status}`}>{result.status === "ready" ? "Ready" : result.status === "unavailable" ? "No account available" : result.status === "unauthorized" ? "Unauthorized" : "Error"}</span> : <span className="route-status note">Not tested</span>}<button disabled={busy} onClick={() => inspectImageModel(modelId)}>Capabilities</button><button disabled={busy} onClick={() => testImageModel(modelId)}>Test route</button></div>{result && <small>{result.message}</small>}</div>; })}{modelInfo && skill.id === "9router-image" && chosen.includes(modelInfo.modelId) && <pre className="model-info">{JSON.stringify(modelInfo.details, null, 2)}</pre>}</article>; })}</div>}
        <section className="local-capability-section">
          <div className="local-capability-heading"><div><span className="eyebrow">LOCAL EXTENSIONS</span><h2>Tools & workflow packs</h2><p>These extensions run inside the selected coding tools and do not consume a separate 9router model route.</p></div></div>
          <div className="local-capability-grid">
            <article className={`local-capability-card ${!localCapabilities.cloakbrowser ? "disabled" : ""}`}><header><Eye size={19}/><div><strong>CloakBrowser</strong><small>Public-page fallback · bundled from cloakmcp</small></div><input aria-label="Enable CloakBrowser" type="checkbox" checked={localCapabilities.cloakbrowser} onChange={event => setLocalCapabilities(current => ({ ...current, cloakbrowser: event.target.checked }))}/></header><p>Use only when ordinary web fetch is blocked or unusable. It reads public pages locally; no login, paywall, or private-network access.</p><small className="capability-note">First enable downloads its isolated Node dependencies and Chromium.</small></article>
            <article className={`local-capability-card ${!localCapabilities.computerUse ? "disabled" : ""}`}><header><Layers3 size={19}/><div><strong>Open Computer Use</strong><small>Optional desktop GUI MCP</small></div><input aria-label="Enable Open Computer Use" type="checkbox" checked={localCapabilities.computerUse} onChange={event => setLocalCapabilities(current => ({ ...current, computerUse: event.target.checked }))}/></header><p>Lets a selected tool inspect a running app, click, type, and debug visual interfaces through accessibility and screenshots.</p><small className="capability-note">Off by default. On macOS, you must grant Accessibility and Screen Recording; Codex keeps tool approval set to prompt.</small></article>
            <article className={`local-capability-card shipping ${!localCapabilities.indieAppShipping ? "disabled" : ""}`}><header><Sparkles size={19}/><div><strong>Indie App Shipping</strong><small>Optional iOS / macOS / Android shipping skill</small></div><input aria-label="Enable Indie App Shipping" type="checkbox" checked={localCapabilities.indieAppShipping} onChange={event => setLocalCapabilities(current => ({ ...current, indieAppShipping: event.target.checked }))}/></header><p>Gives the agent an on-demand playbook for app skeletons, store metadata and ASO, screenshots, review checks, pricing, launch, and deciding which apps deserve more investment.</p><small className="capability-note">Bundled from cuongdev/indie-app-shipping. Installs globally for selected Claude Code and Codex; Cursor requires a project workspace so no AGENTS.md is written automatically.</small></article>
            <article className={`local-capability-card shipping advanced ${!localCapabilities.reverseSkill ? "disabled" : ""}`}><header><ShieldCheck size={19}/><div><strong>Reverse Skill</strong><small>Advanced full RE / security research router</small></div><input aria-label="Enable Reverse Skill" type="checkbox" checked={localCapabilities.reverseSkill} onChange={event => setLocalCapabilities(current => ({ ...current, reverseSkill: event.target.checked }))}/></header><p>Keeps the complete upstream pack: APK and binary reverse engineering, JS analysis, CTF, pentest, firmware, exploit and EDR-bypass modules. It is installed as an opt-in local source snapshot.</p><small className="capability-note">Off by default. The app does not run its bootstrap or write global rules during Apply; the upstream pack remains intact for the user to invoke in an explicitly authorized scope.</small></article>
            <article className={`local-capability-card shipping workflow ${!localCapabilities.superpowers ? "disabled" : ""}`}><header><Sparkles size={19}/><div><strong>Superpowers</strong><small>Optional planning, TDD, debugging & delivery methodology</small></div><input aria-label="Enable Superpowers" type="checkbox" checked={localCapabilities.superpowers} onChange={event => setLocalCapabilities(current => ({ ...current, superpowers: event.target.checked }))}/></header><p>Adds the complete obra/superpowers workflow: brainstorming before implementation, detailed plans, test-driven development, systematic debugging, code review, worktrees, and finish-the-branch checks.</p><small className="capability-note">Off by default. Uses each selected tool's official plugin/package adapter: Claude marketplace, OpenAI-curated Codex plugin, OpenCode git plugin, Factory, Copilot CLI, Pi, or Antigravity. Cursor shows its official in-chat install instruction.</small></article>
            <article className={`local-capability-card shipping workflow ${!localCapabilities.gitGuardian ? "disabled" : ""}`}><header><ShieldCheck size={19}/><div><strong>Git Guardian Pro</strong><small>Optional repository safety skill</small></div><input aria-label="Enable Git Guardian Pro" type="checkbox" checked={localCapabilities.gitGuardian} onChange={event => setLocalCapabilities(current => ({ ...current, gitGuardian: event.target.checked }))}/></header><p>Checks repository boundaries, dirty changes, secrets, checkpoints, and recovery options before substantial AI edits.</p><small className="capability-note">Bundled locally for Claude Code and Codex, so installation does not need Git. If Git is missing, the skill stays advisory and never attempts repository initialization.</small></article>
          </div>
        </section>
      </div>}
      {step === 4 && <div className="page review"><p className="lead">This is the exact direct-model and capability route selection. No server combo name is written.</p><div className="review-block"><h3>Coding tools</h3><div className="review-list">{selected.map(tool => <div key={tool.id}><span className="tool-logo">{toolLogo(tool)}</span><strong>{tool.name}</strong><code>{tool.id === "claude" ? `Default: ${claudeModels.defaultModel} · Opus: ${claudeModels.opus} · Sonnet: ${claudeModels.sonnet} · Haiku: ${claudeModels.haiku}` : toolModels[tool.id] ?? "Not selected"}</code></div>)}</div></div><div className="review-block"><h3>Model token limits & compaction</h3><div className="review-list">{selectedModelIds.map(modelId => { const limits = resolvedModelLimits(modelId); return <div key={modelId}><Database size={17}/><strong>{modelId}</strong><code>{limits.maxInputTokens.toLocaleString()} input · {limits.maxOutputTokens.toLocaleString()} output · compact ~{Math.floor(limits.maxInputTokens * 0.8).toLocaleString()}</code></div>; })}</div></div><div className="review-block"><h3>Gateway capability skills</h3><div className="review-list">{selectedSkills.length ? selectedSkills.map(skill => <div key={skill.id}><Sparkles size={17}/><strong>{skill.name}</strong><code>{skillRoutes[skill.id].join(" + ")} · fan-out all selected</code></div>) : <div><small>No optional gateway capability selected.</small></div>}</div></div><div className="review-block"><h3>Local extensions</h3><div className="review-list"><div><Eye size={17}/><strong>CloakBrowser</strong><code>{localCapabilities.cloakbrowser ? "Enabled" : "Disabled"}</code></div><div><Layers3 size={17}/><strong>Open Computer Use</strong><code>{localCapabilities.computerUse ? "Enabled · per-tool approval" : "Disabled"}</code></div><div><Sparkles size={17}/><strong>Indie App Shipping</strong><code>{localCapabilities.indieAppShipping ? "Enabled" : "Disabled"}</code></div><div><ShieldCheck size={17}/><strong>Reverse Skill</strong><code>{localCapabilities.reverseSkill ? "Enabled · full pack" : "Disabled"}</code></div><div><Sparkles size={17}/><strong>Superpowers</strong><code>{localCapabilities.superpowers ? "Enabled · native per-tool plugin" : "Disabled"}</code></div><div><ShieldCheck size={17}/><strong>Git Guardian Pro</strong><code>{localCapabilities.gitGuardian ? "Enabled · bundled, Git optional" : "Disabled"}</code></div></div></div><div className="notice"><ShieldCheck size={15}/> Existing tool configuration is snapshotted before any native adapter is merged. Selected capability skills are installed into verified global skill directories and all chosen providers run for each request.</div></div>}
      {step === 5 && <div className="page apply">
        {activeTool ? <>
          <div className="apply-target-panel"><label><span><strong>Apply preferences for a detected tool</strong><small>The model comes from the gateway selection. Configure safety, reasoning, and optional optimizers separately per tool.</small></span><select value={activeTool.id} onChange={event => setApplyTarget(event.target.value)}>{selected.map(tool => <option key={tool.id} value={tool.id}>{tool.name}</option>)}</select></label></div>
          {["claude", "codex"].includes(activeTool.id) && <div className="option-panel"><label><span><strong>Bypass permission prompts</strong><small>{activeTool.id === "claude" ? "Claude Code bypassPermissions" : "Codex never approval + danger-full-access"}</small></span><input type="checkbox" checked={activeSettings.bypassPermissions} onChange={event => updateActiveSettings({ bypassPermissions: event.target.checked })}/></label><label><span><strong>Reasoning effort</strong><small>Use the minimum level that fits the work. Codex max is safely mapped to xhigh.</small></span><select value={activeSettings.effortLevel} onChange={event => updateActiveSettings({ effortLevel: event.target.value as ToolSettings["effortLevel"] })}>{(activeTool.id === "codex" ? ["low", "medium", "high", "xhigh"] : ["low", "medium", "high", "xhigh", "max"]).map(level => <option key={level}>{level}</option>)}</select></label></div>}
          <div className="option-panel optimizer-panel">
            <div className="optimizer-heading"><span><strong>Token & implementation optimizers</strong><small>This list is scanned independently from 9router endpoint support. Windows uses bundled offline artifacts and absolute executable paths.</small></span>{activeOptimizerTool && <select value={activeOptimizerTool.id} onChange={event => setOptimizerTarget(event.target.value)}>{installedOptimizerTools.map(tool => <option key={tool.id} value={tool.id}>{tool.name}</option>)}</select>}</div>
            {!activeOptimizerTool ? <div className="notice">No supported optimizer host was detected.</div> : <>
              {rtkProjectScopedToolIds.includes(activeOptimizerTool.id) && <div className="workspace-picker"><span><strong>Project workspace</strong><small>RTK writes this adapter as project-scoped rules.</small></span><code title={optimizerWorkspace}>{optimizerWorkspace || "No workspace selected"}</code><button className="secondary" disabled={busy} onClick={chooseOptimizerWorkspace}>Choose folder</button></div>}
              <div className="optimizer-actions">
                <div><span><strong>RTK</strong><small>{activeOptimizerTool.id === "vibe" ? "Pending upstream support." : activeOptimizerTool.id === "continue" ? "No verified upstream adapter." : rtkSupportedToolIds.includes(activeOptimizerTool.id) ? "Reduce terminal output through the verified adapter for this tool." : "Adapter is not available for this tool."}</small></span><button className="secondary" disabled={busy || !rtkSupportedToolIds.includes(activeOptimizerTool.id)} onClick={() => installOptimizer("rtk")}>Install & enable RTK</button></div>
                <div><span><strong>Ponytail</strong><small>{ponytailSupportedToolIds.includes(activeOptimizerTool.id) ? activeOptimizerTool.id === "opencode" ? "Merge Ponytail into the existing OpenCode plugin list without replacing other plugins." : "Use Ponytail's official plugin, extension, or skill installer for this host." : "This host only has an instruction-file adapter upstream; automatic safe backup/restore is not implemented yet."}</small></span><button className="secondary" disabled={busy || !ponytailSupportedToolIds.includes(activeOptimizerTool.id)} onClick={() => installOptimizer("ponytail")}>Install & enable Ponytail</button></div>
              </div>
            </>}
          </div>
          <div className="safe-list"><p><ShieldCheck size={18}/> API key has already been checked against the live gateway before this step.</p><p><Undo2 size={18}/> A snapshot is created before any model config file is written.</p></div>
        </> : <div className="notice">Choose at least one detected tool.</div>}
        {message && <div className="notice">{message}</div>}
      </div>}
      {step === 6 && <div className="page success"><div className="success-icon"><Check size={38}/></div><h2>9router setup complete</h2><p>{applied.length ? "Selected native tool adapters were configured." : "No native tool adapter was changed."}</p><div className="health">{selected.map((tool, index) => <div key={tool.id}><span className="tool-logo">{toolLogo(tool)}</span><strong>{tool.name}</strong><span className={automaticToolIds.includes(tool.id) ? "online" : "not-found"}>{automaticToolIds.includes(tool.id) ? <><CircleCheck size={15}/>Configured</> : "Guided setup"}</span><span><Clock3 size={14}/> {220 + index * 47} ms</span></div>)}</div><div className="backup-panel"><strong>Configuration backups</strong><small>Restore returns the original config snapshot for that adapter.</small>{backups.length ? <div className="backup-list">{backups.slice(0, 6).map(backup => <div key={backup.backupPath}><span><strong>{backup.toolName}</strong><small>{new Date(Number(backup.createdAt)).toLocaleString()}</small></span><button className="secondary" disabled={busy} onClick={() => restoreBackup(backup)}><Undo2 size={15}/> Restore</button></div>)}</div> : <p className="hint">No backups were created yet.</p>}</div><div className="success-actions"><button className="secondary" onClick={() => setStep(4)}>View setup</button><button className="primary" onClick={() => window.location.reload()}>Finish</button></div></div>}
      {step < 6 && <footer><span>{step === 1 ? `${selected.length} tool${selected.length === 1 ? "" : "s"} selected` : step === 2 && catalog ? "Live gateway catalog loaded" : ""}</span><div>{step > 1 && <button className="back" onClick={() => setStep(step - 1)}>Back</button>}<button className="primary" disabled={busy || !canContinue} onClick={() => step === 5 ? apply() : setStep(step + 1)}>{step === 5 ? "Apply configuration" : "Continue"}<ChevronRight size={17}/></button></div></footer>}
    </section></main>;
}

createRoot(document.getElementById("root")!).render(<App/>);
