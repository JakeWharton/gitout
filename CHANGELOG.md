# Change Log

## [Unreleased]
[Unreleased]: https://github.com/JakeWharton/gitout/compare/0.3.0...HEAD

New:
- Linux Arm variant of Docker container.


## [0.3.0] - 2025-09-08
[0.3.0]: https://github.com/JakeWharton/gitout/releases/tag/0.3.0

New:
- Rewrite the app in Kotlin (from Rust). Sorry (not sorry).
- The binary distribution (which now requires a JVM) supports scheduled sync with the `--cron` option.
- Add `--timeout` option / `GITOUT_TIMEOUT` env var to control limit on git operations. Default is 10 minutes.

Changed:
- The `CRON`, `HEALTHCHECK_ID`, and `HEALTHCHECK_HOST` Docker container environment variables are now named `GITOUT_CRON`, `GITOUT_HC_ID`, and `GITOUT_HC_HOST`, respectively. These will also be honored by the standalone binary.


## [0.2.0] - 2020-05-23
[0.2.0]: https://github.com/JakeWharton/gitout/releases/tag/0.2.0

 * New: Output will now only print repositories which have updates. A counter is included to display overall progress.
 * Fix: Do not ping healthcheck if command fails in Docker container.


## [0.1.0]
[0.1.0]: https://github.com/JakeWharton/gitout/releases/tag/0.1.0

Initial release.
