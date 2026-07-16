# Screenshots

A reusable, consistent screenshot template across all your apps saves enormous design time and still passes review. v1 screenshots need to be clean, clear, and enough for a user to grasp the app — not a marketing masterpiece.

## Principles

- **One feature per screenshot.** Each shot communicates a single idea.
- **Short caption text.** A few words at most. Don't cram the screen with copy.
- **Instant comprehension.** A glance should answer "what does this app do for me?"
- **Consistent style across apps.** Same layout grid, font, caption placement, background treatment — so every new app reuses the same template and you only swap the device frame content.
- **Lead with the strongest shot.** The first 1–2 screenshots are what most users actually see in search results.

## Required sizes (App Store Connect)

You don't need every device size — App Store Connect scales down from the largest required size per device class. Provide at minimum:

- **iPhone** — the current required 6.9"/6.7" size (largest iPhone). This covers smaller iPhones via scaling.
- **iPad** — required only if the app supports iPad; provide the 13"/12.9" size.
- **macOS** — required if you ship a Mac app; macOS screenshots have their own fixed dimensions and window-style framing.

> Apple periodically changes the exact pixel dimensions and which sizes are mandatory. Confirm the current required sizes in App Store Connect at submission time rather than trusting cached numbers.

## Localization

If you localize the listing (vi + en), captions in screenshots should match the listing language for that storefront. Keep caption text in editable template layers (not flattened into the image) so swapping languages is a text change, not a re-export from scratch.

## Workflow

1. Build one master screenshot template (background, frame, caption style).
2. Drop the app's real screens into the frames.
3. Write 3–5 captions, one feature each.
4. Export per required size.
5. Reuse the same template file for the next app.
