# App Skeleton

Every app reuses this structure. Only the **core feature** is new each time. Build this once as a reusable template/project, then fork it.

Default stack assumption: **SwiftUI + StoreKit 2**, iOS 17+ / macOS 14+. RevenueCat is a fine alternative for IAP if the user wants cross-app subscription analytics and easier server-side receipt handling — note the choice, don't assume it.

## Required components (the shared shell)

| Component | Why it exists | Notes |
|---|---|---|
| **Onboarding** | First-run "what is this, why care" in 1–3 screens | Keep it skippable; don't gate the core value behind a long flow |
| **Settings** | Restore purchases, manage subscription, links to legal, support | Single screen is fine for a small app |
| **Paywall** | The monetization surface | Must never be broken — every button tappable, every price loads, close button works |
| **Restore Purchases** | **Required by Apple** if the app has any IAP | A visible, working button. Missing this is a guaranteed rejection |
| **Privacy Policy** | Hosted URL, linked in-app and in metadata | Required; reachable without login |
| **Terms of Use / EULA** | **Required** if there's a subscription | Apple's standard EULA is acceptable if you don't have your own |
| **Account deletion** | **Required** if the app creates an account | In-app path to delete the account and data (guideline 5.1.1(v)) |
| **App icon** | All required sizes | Reuse an icon template; keep a consistent visual family across your apps |
| **Analytics (basic)** | Know what users do | App units, IAP conversion, key feature usage, retention. Disclose in privacy labels |
| **Free / Pro logic** | Gating, entitlement state | One source of truth for "is this user Pro"; check it on launch and after restore |
| **Localization scaffold** | vi + en at minimum | Even a stub now saves rework later; VN and US are the two markets that matter for this user |

## Monetization shapes (pick per app)

- **Free + ads** — fastest user growth; lowest revenue per user; adds AdMob/SDK review surface.
- **Subscription** — recurring; needs Terms/EULA + clear disclosure of price/period before purchase + restore.
- **Lifetime / one-time** — simplest to reason about; no renewal disclosure burden; still needs restore.
- **Free (top-of-funnel)** — no monetization, used to build audience or feed a paid app.

Match the shape to the app, not the other way around. A 30-second-utility app rarely supports a subscription; a daily-use tool might.

## What "minimum" means for v1

Ship when all of these are true, and not before adding more:

- It runs and does the one thing without serious bugs.
- UI is clean enough to not embarrass — not polished.
- The paywall (if any) works end to end including restore.
- Legal links resolve.
- It passes review.

Everything beyond this waits for market signal.
