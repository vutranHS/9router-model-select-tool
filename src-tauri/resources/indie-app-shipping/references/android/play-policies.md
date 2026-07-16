# Google Play — Pre-Submission Policy Checklist

Understanding Play policy matters as much as the code. Play review is more automated and faster than Apple's, but enforcement is harsher: a serious policy strike can suspend the whole developer account, not just the app. Walk this list before every release. These are the items that actually block or endanger small apps.

> Policy numbers/details and time-sensitive gates change. Verify current requirements in Play Console at submission time. When a rejection or warning teaches you something new, add a line here.

## New personal account — the production-access gate

This is the single biggest surprise for new solo devs, and it has nothing to do with your code:

- [ ] If your Play Console **personal** account was created after **Nov 13, 2023**, you must run a **closed test with at least 12 testers, opted in for 14 continuous days**, before you can apply for production access.
- [ ] **Organization accounts (registered legal entity) are exempt** and can publish straight to production.
- [ ] Testers must be real Google accounts on real devices — emulators/fake accounts risk suspension.
- [ ] Shipping a small update during the test (a fix, a tweak) signals genuine testing and helps approval.
- [ ] After 14 days with ≥12 opted-in testers, apply for production access from the Dashboard and answer the readiness questions.

Plan timelines around this: a brand-new account cannot ship to production on day one. See `submission.md` for testing-track mechanics.

## Data safety form

- [ ] **Completed Data safety form** — Play Console blocks submission until it's done. Declares, per data category, what you collect, what you share, why, how it's secured, and whether users can request deletion.
- [ ] **Form matches reality** — declarations must agree with your privacy policy AND what the binary (including every third-party SDK) actually does. Mismatches trigger enforcement.
- [ ] **Encrypted in transit** answered correctly (any reputable SDK uses HTTPS/TLS → Yes).
- [ ] SDK inventory reconciled — analytics/ads/crash SDKs often collect identifiers you must disclose.

## Account deletion (if the app creates accounts)

Play requires **two** things, not one:

- [ ] **In-app path** to delete the account and associated data.
- [ ] **Public web URL** where a user can request account + data deletion (must be reachable without installing the app, and the same URL referenced in your privacy policy).
- [ ] **Data deletion questions answered** in the Data safety form, consistent with both of the above.

## Billing & monetization

- [ ] Digital goods/subscriptions use **Google Play Billing** where required.
- [ ] Subscription paywall shows **price and billing period clearly before purchase**; Terms/EULA linked.
- [ ] Restore/entitlement re-sync works (query Play Billing on launch).
- [ ] Free trial / intro offer terms accurate and disclosed.

## Functionality, content & permissions

- [ ] **Minimum functionality** — not a thin webview wrapper or near-empty app (Play spam/minimum-functionality policy).
- [ ] **No broken functionality** — no crashes on launch/core flow, no dead buttons, no placeholder content.
- [ ] **Permissions minimal and justified** — request only what you use; sensitive permissions (location, SMS, etc.) face extra scrutiny and may need a declaration form.
- [ ] **No third-party IP** without rights; **no Google/Android trademark** misuse.
- [ ] **Content rating (IARC)** questionnaire completed honestly.
- [ ] **Target SDK level** meets Play's current minimum for new apps/updates.

## Multi-platform / large screen

- [ ] Tested on the device classes you declare; large-screen/tablet layout doesn't break if you list tablet support.

## Reality check

Play is fast and mostly automated, but two things bite solo devs hardest: the **12-tester/14-day closed-test gate** on new personal accounts, and **Data safety + account-deletion** consistency. Get those right by default and most releases go through quickly. Treat policy strikes seriously — account-level suspension is the real downside, not a single-app rejection.
