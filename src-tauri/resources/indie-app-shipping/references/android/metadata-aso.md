# Play Store Listing + ASO — Android

The biggest mental switch from iOS: **Google Play has no keyword field.** Play indexes your **title, short description, and full description**. So keyword optimization happens *inside readable copy*, not in a hidden 100-char box. This is the opposite discipline from App Store Connect — don't carry the iOS keyword-stuffing habit over.

## Fill-in template

```
App title:           [<=30 chars. Brandable name + your single most important keyword
                      if it fits naturally. No "free", no Google trademarks]
Short description:    [<=80 chars. The hook shown above the fold. High-value keywords,
                      still readable. This is prime ASO real estate — Play weights it]
Full description:     [<=4000 chars. This IS your keyword surface. Cover what it does,
                      who it's for, features — and naturally include the terms users
                      search. Front-load value; don't keyword-spam (Play penalizes
                      repetitive/irrelevant keyword stuffing)]
Privacy Policy URL:   [Required. Must match the Data safety form answers]
Account deletion URL: [Required if the app has accounts — public web link, see play-policies.md]
App category + tags:  [Pick the right category; choose relevant store tags]
Contact details:      [Email required; website/phone optional]
Content rating:       [Complete the IARC questionnaire honestly]
```

## The description-driven ASO model

- **Title (30):** brand + one anchor keyword if natural. e.g. "Sip - Water Reminder".
- **Short description (80):** the highest-leverage field after the title. Write it for a human, but make sure the 2–3 terms you want to rank for appear naturally.
- **Full description (4000):** structure it as readable paragraphs + a feature list. Include the natural-language phrases people actually type. Aim for relevance and coverage, not raw repetition — Play's spam policy treats keyword stuffing and irrelevant terms as a violation.

## Rejection / penalty avoidance for listings

- **No Google/Android trademarks** misused in title, icon, or graphics.
- **No keyword stuffing or irrelevant keywords** — explicit Play spam-policy violation; can get the listing rejected or demoted.
- **No misleading claims or fake urgency** ("#1", "best", "as seen on") without basis.
- **Metadata must match the app** — features described/shown must exist (parallels iOS 2.3; Play calls it misrepresentation).
- **No placeholder text** anywhere.
- **Data safety consistency** — the listing's data disclosures, your privacy policy, and the actual binary must all agree; mismatches trigger enforcement.

## Localization (vi + en minimum)

Play lets you localize the listing per language. Do at least:

- **en** — US + broad international.
- **vi** — write the title, short description, and full description natively in Vietnamese. Because ASO on Play is description-driven, native Vietnamese phrasing in the full description is what lets you rank for Vietnamese search terms — a real edge in the user's home market. Don't machine-translate the English copy.

Keep both copies in the per-app metadata file so updates stay in sync.
