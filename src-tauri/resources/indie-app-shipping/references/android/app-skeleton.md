# App Skeleton — Android

Every app reuses this structure. Only the **core feature** is new each time. Build this once as a reusable template project, then fork it.

Default stack assumption: **Kotlin + Jetpack Compose**, modern minSdk (recent enough to drop legacy cruft, low enough to reach your market — for VN, mid-range devices still matter, so don't set minSdk too high). Billing: **Google Play Billing Library**, or **RevenueCat** if you also ship iOS and want one subscription API + cross-platform analytics.

## Required components (the shared shell)

| Component | Why it exists | Notes |
|---|---|---|
| **Onboarding** | First-run "what is this, why care" in 1–3 screens | Skippable; don't gate core value behind a long flow |
| **Settings** | Restore purchases, manage subscription, links to legal, support, delete account | Single screen is fine for a small app |
| **Paywall** | The monetization surface | Never broken — every button works, prices load, close works |
| **Restore Purchases** | Re-sync entitlements across devices/reinstalls | Query Play Billing for active purchases on launch and on demand |
| **Privacy Policy** | Hosted URL, linked in-app and in the listing | Required; reachable without login; must match the Data safety form |
| **Terms of Use / EULA** | Expected if there's a subscription | Link in-app and on the paywall |
| **Account deletion** | **Required by Play** if the app creates an account | Needs BOTH an in-app deletion path AND a public web URL — see play-policies.md |
| **App icon** | Adaptive icon (foreground + background layers) | Keep a consistent visual family across your apps |
| **Analytics (basic)** | Know what users do | Installs, IAP conversion, key feature usage, retention. Every SDK that collects data must be reflected in the Data safety form |
| **Free / Pro logic** | Gating, entitlement state | One source of truth for "is this user Pro"; check on launch and after restore |
| **Localization scaffold** | vi + en at minimum | VN + US are the two markets that matter for this user; native vi strings, not machine-translated |

## Android-specific build notes

- **Adaptive icon** — provide foreground + background layers, not a single flat PNG, or it'll look wrong on modern launchers.
- **Edge-to-edge / predictive back** — recent Android target-SDK levels enforce edge-to-edge layouts and predictive back gestures. A layout that ignores insets looks broken on current devices and during review.
- **Target SDK level** — Play enforces a minimum targetSdkVersion for new apps and updates; raising the target periodically is non-optional to keep publishing.
- **SDK inventory** — keep a list of every third-party SDK and what data it touches. You'll need it for the Data safety form, and it's the easiest thing to get wrong.

## What "minimum" means for v1

Ship when all of these are true, and not before adding more:

- It runs and does the one thing without serious bugs.
- UI is clean enough to not embarrass — not polished.
- The paywall (if any) works end to end including restore.
- Legal links resolve and match the Data safety form.
- It passes policy review and (for new personal accounts) clears closed testing.

Everything beyond this waits for market signal.
