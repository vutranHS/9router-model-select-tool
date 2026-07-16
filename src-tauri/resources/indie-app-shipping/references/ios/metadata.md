# App Store Metadata Template

Fill every field before submission. Sloppy or non-compliant metadata is the single most common avoidable rejection. Keep a filled copy per app.

## Fill-in template

```
App name:              [<=30 chars. The brandable name. No "free", no Apple trademarks]
Subtitle:              [<=30 chars. The one-line value prop, keyword-rich but readable]
Promotional text:      [<=170 chars. Updatable without review — use for news/offers]
Description:           [What it does, who it's for, key features. First 1–2 lines matter
                        most (that's what shows before "more"). No keyword stuffing,
                        no competitor names, no fake testimonials]
Keywords:              [<=100 chars total, comma-separated, NO spaces after commas.
                        Don't repeat words already in name/subtitle — Apple indexes those.
                        Singular vs plural: pick one, Apple handles the variation]
Support URL:           [Required. A real reachable page — even a simple one]
Marketing URL:         [Optional landing page]
Privacy Policy URL:    [Required. Reachable without login]
Terms of Use URL:      [Required if subscription. Apple standard EULA acceptable]
Primary category:      [...]
Secondary category:    [optional]
Age rating:            [answer the questionnaire honestly]
Subscription display name + description: [if IAP — clear, matches what the paywall shows]
App Review notes:      [see references/ios/review-notes.md]
```

## Keyword field discipline

- 100 characters is a hard budget — every wasted char is a lost keyword.
- No spaces after commas: `budget,money,expense` not `budget, money, expense`.
- Words in the **app name and subtitle are already indexed** — never duplicate them in keywords.
- Don't use competitor or brand names you don't own (trademark + 2.3.x rejection risk).
- Think in the user's search language, not your internal feature names.

## Rejection-avoidance rules for metadata

- **No Apple trademarks** in name or metadata (no "for iPhone" branding tricks, no Apple logos).
- **No overclaiming** — "best", "#1", medical/financial guarantees invite scrutiny and 2.3 rejections.
- **Accurate metadata (2.3)** — screenshots and description must reflect what the app actually does. No features shown that aren't in the build.
- **No placeholder text** anywhere ("lorem ipsum", "TODO", "test").
- **No third-party IP** you don't have rights to (characters, logos, copyrighted content).
- **Price/period disclosure** — if subscription, the duration and price must be clear before purchase, both on the paywall and in the subscription description.

## Localization (vi + en minimum)

Localize at least name, subtitle, keywords, and description for both markets:

- **en** — for US and the broad international audience.
- **vi** — Vietnamese keywords are often far less contested than English ones; this is a real ASO edge for this user's home market. Localize keywords natively, don't machine-translate the English set.

Keep both copies in the per-app metadata file so updates stay in sync.
