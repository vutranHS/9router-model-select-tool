# macOS — Distribution & Packaging

macOS shares the strategy, skeleton, and most metadata with iOS, but **distribution and packaging are different** and trip up devs who treat a Mac app like an iPhone app. Read this when an app targets macOS. The reject-prone App Review items in `review-guidelines.md` still apply if you ship via the Mac App Store.

> macOS signing/notarization tooling and Gatekeeper rules change between Xcode/macOS releases. Verify the current notarization workflow in Apple's docs at build time.

## Two distribution paths — choose first

| | **Mac App Store (MAS)** | **Direct distribution (website/DMG)** |
|---|---|---|
| Discovery | Built-in store discovery | None — you drive all traffic |
| Review | Apple review (like iOS) | No review |
| Sandbox | **Mandatory** App Sandbox | Optional |
| Payments | StoreKit IAP (Apple's cut) | Your own (Paddle/Stripe/Gumroad/Paddle as merchant of record) — no Apple cut |
| Updates | Store handles it | You handle it — use **Sparkle** for auto-update |
| Trust/install friction | Lowest (store-trusted) | Must be signed + notarized or Gatekeeper blocks it |

**Pick MAS** for discovery, user trust, and simplicity. **Pick direct** when the app needs capabilities the sandbox forbids (deep system access, certain automation/utility behavior), when you want to avoid the 30% cut, or when you want to iterate without review. You can also ship the same app both ways.

## Direct distribution: signing + notarization (required)

Modern macOS Gatekeeper will block an unsigned/un-notarized app. The flow:

1. **Code-sign** with a **Developer ID** certificate (not the Mac App Store cert).
2. **Notarize** — submit the signed app/DMG to Apple's notary service (current tool: `notarytool`), which scans it and returns a ticket.
3. **Staple** the notarization ticket to the app/DMG so it validates offline.
4. Distribute the DMG (or pkg). First launch passes Gatekeeper cleanly.

Skipping notarization is the #1 reason a direct-download Mac app shows the scary "can't be opened" warning.

## Sandbox & entitlements (MAS path)

- MAS apps **must** enable App Sandbox.
- Capabilities you take for granted need explicit **entitlements**: outgoing/incoming network, user-selected file access, hardware, etc. Request only what you use.
- Some app types (system utilities, broad automation, things needing unrestricted system access) **don't fit the sandbox** — those go direct-distribution, not MAS.

## Screenshots & assets

macOS listings use their own required screenshot dimensions and a window-framed presentation distinct from iPhone. Keep a separate macOS screenshot template; the iOS phone template won't fit. Same principles as `screenshots.md` (one feature per shot, short captions, consistent style).

## Build approach

If porting an iOS app, decide between **Mac Catalyst** (fastest port of a UIKit/SwiftUI iOS app, but feels less native) and a **native AppKit/SwiftUI** Mac target (more work, more native). For a small test app, Catalyst can be enough to get a macOS signal cheaply; deepen to native only if the app earns it.
