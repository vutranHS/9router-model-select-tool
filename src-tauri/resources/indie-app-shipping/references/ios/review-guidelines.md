# Apple App Review — Pre-Submission Checklist

Understanding the guidelines matters as much as the code. Apple isn't hard to pass if you standardize compliance from the start. Walk this list before every Submit. These are the items that actually cause rejections for small apps.

> Guideline numbers below reference Apple's App Review Guidelines. The reject-prone *categories* are stable, but exact numbering and details change — verify the current text at developer.apple.com/app-store/review/guidelines when in doubt. When a rejection teaches you something new, add a line here.

## In-app purchases & subscriptions

- [ ] **Restore Purchases button** present and working (required for any IAP). — 3.1.1
- [ ] **Terms of Use / EULA** linked if there's a subscription (Apple standard EULA acceptable). — 3.1.2
- [ ] **Price and billing period shown clearly before purchase**, on the paywall and in the subscription description. — 3.1.2
- [ ] Paywall has **no dead ends**: every button tappable, prices load, a working close/dismiss control, no infinite spinner.
- [ ] No mention of **alternative/external payment** methods to dodge Apple's IAP for digital goods. — 3.1.1
- [ ] Free trial / intro offer terms are accurate and disclosed.

## Accounts & privacy

- [ ] **In-app account deletion** path if the app creates an account (delete account + data, not just sign-out). — 5.1.1(v)
- [ ] **Sign in with Apple** offered if you offer any third-party social login (Google/Facebook/etc.). — 4.8 / 5.1
- [ ] **Privacy Policy URL** reachable without login.
- [ ] **Privacy nutrition labels** in App Store Connect match what the app and its SDKs actually collect.
- [ ] **App Tracking Transparency** prompt if you track users across apps/sites (e.g., ad SDKs).
- [ ] Only request permissions you use; each usage-description string explains *why*.

## Functionality & content

- [ ] **Minimum functionality** — not a thin wrapper around a website or a near-empty app. — 4.2
- [ ] **No placeholder/beta content**, no broken links, no crashes on launch or core flow.
- [ ] **Accurate metadata (2.3)** — screenshots/description reflect the real build; no features shown that don't exist.
- [ ] **No third-party IP** without rights (characters, logos, copyrighted media, brand names).
- [ ] **No Apple trademarks** misused in name, icon, or metadata.
- [ ] No **overclaiming** (medical/financial guarantees, "#1", "best") without basis.

## Multi-platform

- [ ] Tested on the actual device classes the app declares (iPhone / iPad / macOS). A layout that breaks on iPad while iPad is enabled is a rejection.
- [ ] macOS build: window resizing, menu bar, and quit behave correctly if shipping Mac.

## Submission hygiene

- [ ] **App Review note** written (see review-notes.md): what the app does, demo account if login required, how to reach paid features, any third-party content.
- [ ] Demo account credentials provided and working if any feature requires sign-in.
- [ ] Build actually contains the version the metadata describes.

## Reality check

Apple is predictable once you standardize. The same handful of items — restore button, subscription terms, account deletion, accurate metadata, minimum functionality — account for most small-app rejections. Get them right by default and most submissions sail through.
