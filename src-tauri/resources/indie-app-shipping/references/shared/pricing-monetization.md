# Pricing & Monetization

Pick the monetization shape per app, then price it for the market — not the other way around. For a portfolio targeting both Vietnam and the US, the single biggest pricing mistake is using one global price for two markets with very different purchasing power. Read this at the "prepare monetization" step.

## Match the shape to the app

Use the usage pattern to choose, before you think about price:

| Usage pattern | Best shape | Why |
|---|---|---|
| Used repeatedly / daily, ongoing value | **Subscription** | Recurring value justifies recurring pay; needs Terms/EULA + clear period disclosure + restore |
| Solves a one-off or occasional need | **Lifetime / one-time** | Users resist subscribing to something they open monthly; simpler to reason about |
| Broad, casual, high-volume, low intent | **Ads** (± remove-ads IAP) | Monetizes users who'd never pay; but revenue depends heavily on geo (see below) |
| Top-of-funnel / audience builder | **Free** | No monetization; feeds a paid app or builds reach |

Forcing a subscription onto a 30-second utility is the most common indie mis-step. If you can't articulate the *ongoing* value, don't subscribe-gate it.

## Regional pricing: VN vs US

Both App Store and Google Play support per-storefront pricing, and both can auto-localize a base price into local equivalents. Use that as a *starting point*, then tune — don't ship one USD price worldwide.

- **US / high-income storefronts** tolerate higher prices; this is usually where subscription and lifetime revenue concentrate.
- **Vietnam** is highly price-sensitive — a price that converts in the US will kill conversion in VN. Set a deliberately lower local price; a lower price at much higher volume often nets more, and it builds the install/retention base that *is* the signal you're hunting for.
- **Lifetime** especially needs regional tuning — a US lifetime price applied directly to VN reads as absurd locally.

Practical approach: set a sensible base price, let the store localize, then manually lower the VN (and similar emerging-market) tiers. Revisit once you see per-country conversion.

## Trial & paywall design

- **Hard paywall** (must subscribe to use) maximizes revenue per user but suppresses installs/retention — bad when your goal is *signal*. **Soft paywall** (free core + Pro upsell) keeps the funnel open so you can actually measure activation and retention. For test-phase apps, prefer soft.
- **Trial length**: short (3–7 days) for habit/utility apps where value is obvious fast; longer only if the value takes time to appear.
- **One clear Pro tier** beats a confusing menu for a small app. Free vs Pro, one price, done.
- Always show **price + billing period before purchase** and a working **Restore** — both stores require it and both reject for getting it wrong.

## Ads: the geo trap (important for VN-heavy traffic)

Ad revenue (eCPM) varies enormously by country — US/Western traffic earns multiples of what VN traffic earns per impression. So an ad-monetized app whose installs are mostly Vietnamese can show great install/retention numbers and still earn very little. This interacts directly with the geographic signal in `signal-metrics.md`: a VN-concentrated app is often a better candidate for a cheap one-time/lifetime IAP than for ads. Decide monetization with the expected *audience geography* in mind, not just the app type.

## "Testing" price

You can't cleanly A/B prices on the stores, but you can:
- Change price over time and compare cohorts before/after.
- Use a billing layer like RevenueCat to run paywall/price experiments across your portfolio.
- Lean on portfolio comparison: if one app's price/shape converts far better, copy it to similar apps.

Keep monetization decisions cheap and reversible at the test stage — the app earning the right to depth is when serious pricing work pays off.
