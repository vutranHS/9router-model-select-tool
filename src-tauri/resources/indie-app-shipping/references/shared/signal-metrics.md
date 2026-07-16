# Signal & Metrics — When to Deepen or Park

This is the heart of the "ship many small apps" method. The entire strategy is *ship → measure → decide*, so "measure what, and what counts as signal" must be concrete — otherwise the deepen/park call is just a gut feeling. The point of a v1 is **information, not revenue**. Read this when an app has been live long enough to have data, or when setting up analytics before launch.

## Signal vs. noise at low volume

Small apps start with tiny numbers, so don't wait for statistical significance — you'll never get it on a parked-or-deepen timescale. Early on, weight **direction and quality** over magnitude:

- A handful of **organic** installs (zero ad spend) beats hundreds of paid installs — it means the store + the need found each other on their own.
- **Qualitative** signals (an unsolicited 5-star review, a support email asking for a feature, someone sharing it) carry more weight at low N than any conversion percentage.
- One **country over-indexing** unexpectedly is a real signal — lean in / localize there.

## The metrics that matter (in priority order)

1. **Organic discovery** — installs/units from search and browse with no ad spend. The strongest early signal that the niche is real. Track it separately from any paid/referral installs.
2. **Activation** — % of new users who reach the core action (the "aha"). If people install but never use the one thing the app does, the idea or the onboarding is wrong — fix onboarding before judging the idea.
3. **Retention (D1 / D7)** — do they come back? Retention is the truest proxy for genuine need. Use D1/D7 for fast iteration; D30 is too slow for this loop.
4. **Monetization** — IAP conversion %, and for subs the trial→paid rate. Only meaningful once activation + retention are non-trivial; a high conversion on no traffic tells you nothing.
5. **Geographic & device mix** — which storefronts and which devices (iPhone vs iPad vs Mac) behave differently. Feeds both localization and pricing.

## Compare against your own portfolio, not absolutes

Because you ship many apps, your most reliable yardstick is your **own median**. Maintain a simple portfolio table (app, organic installs/wk, D7, IAP conv%, country mix). Rank each new app against your existing apps. "Top third of my portfolio on organic + retention" is a more actionable signal than any universal threshold, and it adjusts automatically as your baseline improves.

## Decision rules

Set a **fixed observation window** per app (e.g., 4–8 weeks post-launch) so apps don't drift into indefinite limbo. At the end of the window:

- **Deepen** if any of: organic installs trending up without spend; D7 retention notably above your portfolio median; healthy IAP/trial conversion; a concentrated market or qualitative pull. → Then invest: UI/UX, Pro features, ASO, localization, controlled ad spend, landing page, port to the other platform.
- **Park** if: near-zero organic installs AND near-zero retention after the window. Light maintenance only. This is a *completed experiment*, not a failure — log what it taught you (niche, country, monetization shape) and move on.
- **Iterate once before parking** if activation is low but installs exist — that usually means onboarding/positioning, not the idea. One focused fix, then re-judge.

## Instrumentation minimum (put in every app's skeleton)

Log these events in every app so the data is comparable across the portfolio:

- `install` / first open
- `onboarding_complete`
- `core_action` (the app's one key action — name it per app)
- `paywall_view`
- `purchase` (+ product id) / `trial_start` / `trial_convert`
- `day2_open` (cheap retention proxy)

Keep it light and disclose every collecting SDK in the privacy labels / Data safety form. The goal is a comparable scoreboard across apps, not deep product analytics.
