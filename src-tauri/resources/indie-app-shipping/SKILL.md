---
name: indie-app-shipping
description: A solo/indie playbook for shipping small mobile apps fast on iOS/macOS (App Store) and Android (Google Play), testing real demand, and deepening only the apps that show signal. Use whenever the user is shipping a mobile app — scaffolding app structure, wiring onboarding/settings/paywall/restore/legal screens, writing or reviewing store metadata (title, subtitle, keywords, short/long description, promo text), planning screenshots/feature graphics, running a pre-submission Apple App Review or Google Play policy compliance check, writing review notes, setting up Play closed testing, handling a rejection, or deciding which shipped apps deserve more investment. Trigger even when the user doesn't say "checklist" — any mention of a new app idea, App Store or Play Store submission, App Store Connect, Play Console, IAP/subscription/paywall, screenshots, app rejection, ASO, Data safety form, closed testing, or the "ship many small apps" workflow should pull this in.
---

# Indie App Shipping

A standardized "mini assembly line" for one person shipping many small mobile apps across iOS/macOS (App Store) and Android (Google Play). The core bet: don't guess what the market wants — ship a clean, minimal app fast, measure real users, then invest deeply only in the apps that show signal. Most of the work that slows a solo dev down is *non-core* work (onboarding, paywall, legal pages, metadata, screenshots, review prep). Standardize all of that once so each new app only needs its core feature swapped in.

The user works in Vietnamese — respond in Vietnamese unless they switch. Store Console fields, keywords, and platform terminology stay in English where that's the actual on-platform value.

## The strategic frame

There are two ways to ship apps:

1. **One or two deep apps** — heavy UI/UX, marketing, branding, fighting competitors head-on. Right when you have a team, budget, clear market insight, and certainty the niche is big enough.
2. **Many small apps to test fast** — each app solves one concrete need, ships quickly, and measures real reaction. Deepen only what shows signal.

This skill is built for path 2. The point of an early app is **information**, not perfection: which niches have demand, which countries download, which monetization works, what the store flags, which apps earn the right to more work.

When the user is clearly resourced for path 1 on a specific app (real budget, real signal already), say so and adjust — don't force the assembly-line frame onto an app that has earned depth.

## Platform selection — decide first

Before scaffolding, decide which platform(s) this app targets, then read the matching reference track. Most of the *strategy* and the *workflow shape* below is shared; the *execution details* (store policies, ASO model, billing, assets, submission gates) differ enough that each platform has its own reference files.

- **iOS / macOS** → `references/ios/`
- **Android (Google Play)** → `references/android/`
- **Both** → read both tracks; build the shared skeleton once with a billing layer that abstracts the store (RevenueCat is the pragmatic cross-platform choice here, since it wraps StoreKit 2 and Play Billing behind one API and one set of subscription analytics).

### Key differences that change how you ship (not just code)

| Dimension | iOS / App Store | Android / Google Play |
|---|---|---|
| **ASO model** | Dedicated **keyword field** (100 chars), separate from description | **No keyword field** — Play indexes title (30) + short desc (80) + full desc (4000); weave keywords naturally into description |
| **Billing** | StoreKit 2 | Google Play Billing Library |
| **Policy doc** | App Review Guidelines (numbered) | Play Developer Program Policies + User Data policy |
| **Privacy disclosure** | Privacy nutrition labels | **Data safety form** (must match privacy policy AND actual binary) |
| **Account deletion** | In-app deletion path (5.1.1(v)) | In-app path **AND** a public web deletion URL, declared in Data safety form |
| **Third-party login** | "Sign in with Apple" required if you offer social login | No equivalent mandate |
| **Review process** | Human review; slower; frequent rejections | More automated; faster; but policy strikes can suspend the whole account |
| **Pre-prod gate** | TestFlight (optional) | New personal accounts: **mandatory closed test, 12 testers, 14 continuous days** before production access |
| **Store assets** | App icon + screenshots | App icon (adaptive) + screenshots + **feature graphic 1024×500** |

> Store policies change. The Android reference files were grounded against current Play policy, but verify time-sensitive gates (tester counts, Data safety questions, new-account verification) in Play Console / Apple's guidelines at submission time.

### Which platform first?

If you can only ship one first, let the target market decide. Android dominates device share in Vietnam and most emerging markets, so a VN-first app reaches more users sooner on Play — but ad/IAP revenue per user there tends to be low. iOS concentrates paying users and revenue in the US and other high-income markets, at the cost of a slower human review. Rule of thumb: **lead with the platform where your target users and your monetization model actually live** — Play for VN-market reach and volume, App Store for US-market revenue — then port the winners to the other store (see the decision framework). Match this against the monetization shape in `references/shared/pricing-monetization.md` (a VN-heavy ads app earns little; a US-heavy subscription app earns more).

## Workflow for a new app

Run these in order. Pull the matching platform reference when you reach each step.

1. **Validate the need in one sentence.** Before any code: "This app helps [who] do [one specific thing]." If you can't say it in one line, the app isn't ready. Personal pain points and needs of people around the user are the strongest idea source — when you genuinely have the problem, the app description writes itself.
2. **Pick the platform(s)** (see above) and read the matching track.
3. **Scaffold the standard skeleton.** Reuse the shared structure; only the core feature is new. See `<platform>/app-skeleton.md`.
4. **Build the minimum that earns a verdict.** Ships, no serious bugs, clean-enough UI, solves the one need, basic monetization if it fits, passes review. Do **not** optimize early — let the market answer first.
5. **Choose monetization and price it per market.** Match the shape to the app and set regional prices (VN vs US differ a lot). See `references/shared/pricing-monetization.md`.
6. **Prepare metadata.** Fill the template; sloppy metadata is a top rejection cause and, on Play, your only ASO surface. See `ios/metadata.md` or `android/metadata-aso.md`. Legal pages (privacy policy, EULA, deletion page) have fill-in templates in `assets/`.
7. **Produce store assets.** Same template across apps, one feature per screenshot, minimal text. See `<platform>/screenshots.md`. macOS packaging/distribution differs — see `ios/macos.md`.
8. **Run the pre-submit compliance pass.** Walk the reject-prone checklist. See `ios/review-guidelines.md` or `android/play-policies.md`.
9. **Write submission materials.** App Review note (iOS) / submission + testing setup (Android). See `ios/review-notes.md` or `android/submission.md`.
10. **Submit, then instrument for signal.** Wire the standard analytics events and measure against your portfolio. What counts as signal vs. noise — and the deepen/park rules — is in `references/shared/signal-metrics.md`.

## Decision framework: deepen or park

After an app is live and has had time to gather data, sort it. The concrete signal definitions, metric priorities, and thresholds for this call live in `references/shared/signal-metrics.md` — the summary:

**Signal → deepen.** Improve UI/UX, add Pro features, optimize ASO, add localization, test ads, write a build-in-public post, build a landing page, grow a small community — and consider porting a single-platform winner to the other store.

**No signal → park.** Light maintenance only. Don't burn more time. A parked app is not a failure — it's a completed experiment that returned data.

The trap this avoids: spending six months on one app only to discover the market didn't want it. The solo dev's real advantage isn't money or team — it's *speed of turning*. See it's wrong, fix it. See an app is weak, stop. See signal, dig in. See a rejection, learn the policy and update the checklist.

"Short-term to fund long-term": the small apps generate cash flow and learning that fund the eventual deep bet.

## Reference files

Read the relevant file when you hit its step — don't load all of them up front.

**Shared** (`references/shared/`) — platform-agnostic, used on every app
- `signal-metrics.md` — what to measure, signal vs noise, and the deepen/park decision rules.
- `pricing-monetization.md` — monetization shape selection + VN/US regional pricing + trial/ads strategy.

**iOS / macOS** (`references/ios/`)
- `app-skeleton.md` — shared app structure (SwiftUI + StoreKit 2 default; RevenueCat noted).
- `metadata.md` — App Store Connect metadata template + rejection-avoidance + vi/en localization.
- `screenshots.md` — reusable screenshot spec and sizes.
- `review-guidelines.md` — pre-submission compliance checklist for the items that actually cause rejections.
- `review-notes.md` — App Review note template with examples.
- `macos.md` — macOS distribution (MAS vs direct), notarization, sandbox, screenshots.

**Android / Google Play** (`references/android/`)
- `app-skeleton.md` — shared app structure (Kotlin + Jetpack Compose default; Play Billing / RevenueCat).
- `metadata-aso.md` — Play Store listing + the description-driven ASO model + vi/en localization.
- `screenshots.md` — screenshot + feature graphic + adaptive icon spec.
- `play-policies.md` — pre-submission compliance: Data safety, account deletion, closed testing, enforcement.
- `submission.md` — testing-track setup, review/test notes, submission hygiene.

**Assets** (`assets/`) — fill-in templates to copy per app (placeholders in brackets; not legal advice)
- `privacy-policy-template.md`
- `eula-template.md`
- `account-deletion-page-template.html`

## When the user hits a rejection

Don't treat it as a one-off. Identify the exact guideline/policy the store cited, fix this app, **then** propose a line to add to the relevant `review-guidelines.md` / `play-policies.md` so the same rejection never recurs. Every rejection is a permanent upgrade to the checklist.
