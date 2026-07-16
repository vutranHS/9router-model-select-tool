# Store Graphics — Android

Reusable, consistent graphics across all your apps save design time and still pass review. v1 graphics need to be clean, clear, and enough to grasp the app. The same screenshot principles as iOS apply; Android adds a couple of required assets that don't exist on the App Store.

## Shared principles

- **One feature per screenshot.** Each shot communicates a single idea.
- **Short caption text.** A few words at most.
- **Instant comprehension.** A glance answers "what does this do for me?"
- **Consistent style across apps.** Same grid, font, caption placement so each new app reuses one template.
- **Lead with the strongest shot.** The first 1–2 screenshots dominate what users see.

## Required Play assets

| Asset | Notes |
|---|---|
| **Phone screenshots** | At least 2 required (more is better). Follow Play's current aspect-ratio/min-dimension rules |
| **Feature graphic** | **1024 × 500**, required. Shown at the top of the listing and used in promotional placements. No iOS equivalent — don't forget it |
| **App icon** | 512 × 512 hi-res icon for the listing, plus the **adaptive icon** (foreground + background layers) shipped in the app |
| **Tablet screenshots** | Provide if you support tablets/large screens (Play surfaces large-screen quality) |
| **Promo/TV/Wear assets** | Only if you target those form factors |

> Play periodically adjusts exact dimensions and which assets are mandatory. Confirm current requirements in Play Console at submission time rather than trusting cached numbers.

## Localization

If you localize the listing (vi + en), captions should match the listing language for each locale. Keep caption text in editable template layers (not flattened into the image) so swapping languages is a text change, not a re-export.

## Workflow

1. Build one master template (background, device frame, caption style) plus a feature-graphic template.
2. Drop the app's real screens into the frames.
3. Write 3–5 captions, one feature each; design the 1024×500 feature graphic.
4. Export per required size + the feature graphic + the 512 icon.
5. Reuse the same template files for the next app.
