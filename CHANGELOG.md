# Change Log

## [Unreleased]
[Unreleased]: https://github.com/cashapp/redwood/compare/0.18.0...HEAD

New:
- Rewrite the app in Kotlin (from Rust). Sorry (not sorry).
- The binary distribution (which now requires a JVM) supports scheduled sync with the `--cron` option.

Changed:
- The `CRON`, `HEALTHCHECK_ID`, and `HEALTHCHECK_HOST` Docker container environment variables are now named `GITOUT_CRON`, `GITOUT_HC_ID`, and `GITOUT_HC_HOST`, respectively. These will also be honored by the standalone binary.


## [0.2.0] - 2020-05-23
[0.2.0]: https://github.com/JakeWharton/gitout/releases/tag/0.2.0

 * New: Output will now only print repositories which have updates. A counter is included to display overall progress.
 * Fix: Do not ping healthcheck if command fails in Docker container.


## [0.1.0]
[0.1.0]: https://github.com/JakeWharton/gitout/releases/tag/0.1.0

Initial release.
