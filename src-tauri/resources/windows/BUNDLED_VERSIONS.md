# Windows bundled optimizer versions

- RTK: `v0.43.0`
  - Asset: `rtk-x86_64-pc-windows-msvc.zip`
  - Download archive SHA-256: `7c5e4a2ef816a4d4ed947ddd74ca3df851fc39ea87d49a3ca2bf3abc515a016b`
  - Source: https://github.com/rtk-ai/rtk/releases/tag/v0.43.0
  - The matching `openclaw/` adapter is bundled from tag commit
    `5a7880d404db8364d602f2ecdc41dd790f64013f`.
- Ponytail: commit `16f29800fd2681bdf24f3eb4ccffe38be3baec6b`
  - Source: https://github.com/DietrichGebert/ponytail

These files are included only by `cfg(windows)`, so macOS builds do not embed
the Windows RTK binary or Ponytail archive.
