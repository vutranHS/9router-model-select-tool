---
description: Use Open Computer Use only for an explicitly requested desktop-app workflow that cannot be completed through a dedicated integration, CLI, or browser tool.
---

Open Computer Use controls live desktop applications through MCP.

Workflow:

1. Call `get_app_state` before each turn that needs desktop interaction.
2. Prefer element-index actions from the accessibility tree over pixel coordinates.
3. After every action, read the app state again before deciding the next step.
4. Prefer a dedicated app integration, CLI, or browser tool when one can complete the task.

Safety:

- Ask before any destructive, externally visible, or financial action.
- Never type credentials, API keys, OTPs, or other secrets into an app unless the user specifically approved that exact value and destination.
- Do not bypass OS permission prompts, CAPTCHA, paywalls, login walls, or security controls.
- Do not use it to inspect unrelated apps or private user data.
