# Signed macOS release setup

The release workflow signs and notarizes the Apple Silicon DMG when these GitHub Actions secrets are configured:

- `APPLE_CERTIFICATE` — base64-encoded `.p12` export of the **Developer ID Application** identity.
- `APPLE_CERTIFICATE_PASSWORD` — password used when exporting the `.p12`.
- `APPLE_SIGNING_IDENTITY` — full certificate name, for example `Developer ID Application: Vu Tran Huu (2M58JBRPSX)`.
- `APPLE_ID` — Apple Account email.
- `APPLE_PASSWORD` — an Apple app-specific password, not the Apple Account password.
- `APPLE_TEAM_ID` — Apple Developer Team ID.

Export the certificate locally without committing it:

```bash
security export -k ~/Library/Keychains/login.keychain-db \
  -t identities -f pkcs12 -P '<p12-password>' -o ~/Desktop/9router-model-selector.p12
base64 -i ~/Desktop/9router-model-selector.p12 | pbcopy
```

Create the app-specific password at [appleid.apple.com](https://appleid.apple.com/), then add all values under **GitHub repository → Settings → Secrets and variables → Actions**.

Once the secrets exist, create and push a new `v*` tag. Tauri will sign with the Developer ID certificate and notarize/staple the DMG using `APPLE_ID`, `APPLE_PASSWORD`, and `APPLE_TEAM_ID`.

For a local signed and notarized build, export the same six variables in the terminal and run:

```bash
npm run build:mac-arm
```
