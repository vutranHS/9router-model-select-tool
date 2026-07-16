# App Review Notes

The review note tells the reviewer exactly what the app does, how to exercise it, and how to reach anything gated. A clear note prevents avoidable rejections and speeds review. Keep one per app.

## Template

```
What the app does:
[2–3 sentences in plain language. The reviewer should understand the purpose
without exploring.]

How to use the core feature:
[Step-by-step path to the main value, so the reviewer doesn't miss it.]

Account / login:
[ ] No account required, OR
[ ] Demo account: username + password (must work for the entire review window)

In-app purchases:
[What's free vs paid, how to reach the paywall, what restore does. If you can
provide a sandbox path to test purchase flow, say so.]

Third-party content / services:
[Any external APIs, user-generated content, streams, or data sources, and who
owns/moderates them. If the app plays user-supplied media/streams, say the user
provides the content and the app is a player.]

Permissions:
[Why each requested permission is needed.]

Anything non-obvious:
[Region-locked behavior, hardware requirements, why a feature may look empty on
first launch, etc.]
```

## Example — utility app, no login, with IAP

```
What the app does: A quick on-screen translator. The user captures any text on
screen and gets an instant translation in a floating panel.

How to use the core feature: Open the app, grant screen-capture permission when
prompted, tap "Translate", select a region of the screen. The translation
appears immediately.

Account / login: No account required.

In-app purchases: Free tier allows 5 translations/day. Pro (monthly subscription)
removes the limit. Tap Settings > Upgrade to see the paywall. Restore Purchases is
in Settings.

Third-party content: Uses [translation provider] API for translation only; no
user data is stored on our servers.

Permissions: Screen recording — required to read the text the user selects.

Anything non-obvious: The floating panel only appears after the first capture.
```

## Example — app that plays user-supplied streams

```
What the app does: A personal stream player. The user pastes their own stream URL
and the app plays it. The app ships with no preloaded content.

Account / login: No account required.

Third-party content: All content is supplied by the user (their own stream URLs).
The app is a player and hosts no content itself. For review, you can paste this
public test stream: [public test URL].

Anything non-obvious: On first launch the library is empty by design — add a URL
to see playback.
```

Keep notes honest and specific. Vague notes ("it's a productivity app") slow review; precise ones get you approved faster.
