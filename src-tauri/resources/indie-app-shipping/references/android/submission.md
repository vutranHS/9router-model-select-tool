# Submission & Testing — Android

Play doesn't have a single "App Review notes" box like Apple, but you still prepare equivalent materials: testing-track setup, reviewer/tester guidance, and submission hygiene. Keep one per app.

## Testing tracks (and which one matters)

Play offers a ladder; use it to ramp up and, for new personal accounts, to unlock production:

- **Internal testing** — up to a small list of your own trusted testers; instant builds; optional; does **not** count toward the new-account requirement.
- **Closed testing** — a controlled group you invite. **Mandatory** for new personal accounts: ≥12 testers opted in for 14 continuous days before you can apply for production access. Use it to fix issues and prove real-device testing.
- **Open testing** — anyone can join; surfaces a public test version and collects private feedback. Optional.
- **Production** — public. New personal accounts reach it only after passing closed testing and the readiness review.

> Default flow for a new personal account: Internal (smoke test) → Closed (run the 12/14 gate, ship a small fix mid-test) → apply for production access → Production. Organization accounts can skip straight to production.

## Tester / reviewer guidance template

```
What the app does:
[2–3 plain sentences. The purpose without exploration.]

How to use the core feature:
[Step-by-step path to the main value.]

Account / login:
[ ] No account required, OR
[ ] Test account: username + password (must work throughout the test/review window)

In-app purchases:
[What's free vs paid, how to reach the paywall, how restore works. If you can
provide a license-test / sandbox path, note it.]

Third-party content / services:
[External APIs, user-generated content, data sources, and who owns/moderates them.]

Permissions:
[Why each requested permission is needed — especially any sensitive permission.]

Anything non-obvious:
[Region-locked behavior, why a screen looks empty on first launch, hardware needs.]
```

## Finding 12 testers (the real bottleneck)

The hard part of the new-account gate is people, not engineering:

- Recruit from friends/colleagues with real Google accounts and real devices; over-recruit (15–20) so one drop-off doesn't break the 14-day streak.
- Tester-exchange communities exist (devs test each other's apps). Paid tester services exist too — weigh cost vs. the risk of low-quality/fake engagement, which can backfire under Play's engagement tracking.
- Once a tester opts in, they generally keep counting even if they uninstall — but genuine engagement is what Play's tightened algorithm rewards, so aim for testers who actually open the app.

## Submission hygiene

- [ ] Data safety form complete and consistent (see play-policies.md).
- [ ] Account deletion in-app path + public web URL live (if accounts exist).
- [ ] Privacy policy URL resolves and matches the form.
- [ ] Content rating completed.
- [ ] Test account credentials valid for the whole window if login is required.
- [ ] The uploaded build matches the version the listing describes.
- [ ] Target SDK meets the current minimum.

Keep notes honest and specific. Vague guidance slows things down; precise guidance gets you approved faster — same principle as the App Store, different surface.
