Git Out
=======

A command-line tool to automatically backup Git repositories from GitHub or anywhere.


Installation
------------

### Rust

If you have Rust installed you can install the binary by running `cargo install gitout`.

[![Latest version](https://img.shields.io/crates/v/gitout.svg)](https://crates.io/crates/gitout)

### Docker

The binary is available inside the `jakewharton/gitout` Docker container which can run it as a cron job.

[![Docker Image Version](https://img.shields.io/docker/v/jakewharton/gitout?sort=semver)][hub]
[![Docker Image Size](https://img.shields.io/docker/image-size/jakewharton/gitout)][layers]
[![Docker Image Layers](https://img.shields.io/microbadger/layers/jakewharton/gitout)][layers]
[![Docker Pulls](https://img.shields.io/docker/pulls/jakewharton/gitout.svg)][hub]

 [hub]: https://hub.docker.com/r/jakewharton/gitout/
 [layers]: https://microbadger.com/images/jakewharton/gitout

Mount the `/data` and `/config` folders, specify a `CRON` environment variable, and run:

```
$ docker run -d \
    -v /path/to/data:/data \
    -v /path/to/config.toml:/config/config.toml \
    -e "CRON=*/1 * * * *" \
    jakewharton/gitout
```

For help creating a valid cron specifier, visit [cron.help](https://cron.help/#0_*_*_*_*).

To be notified when sync is failing visit https://healthchecks.io, create a check, and specify the ID to the container using the `HEALTHCHECK_ID` environment variable (for example, `-e "HEALTHCHECK_ID=..."`).

If you're using Docker compose, an example setup looks like;
```yaml
services:
  gitout:
    image: jakewharton/gitout:latest
    restart: unless-stopped
    volumes:
      - /path/to/data:/data
      - /path/to/config:/config
    environment:
      - "CRON=0 * * * *"
      #Optional:
      - "HEALTHCHECK_ID=..."
```

Note: You may want to specify an explicit version rather than latest. See https://hub.docker.com/r/jakewharton/gitout/tags.

### Binaries

TODO GitHub releases download?


Usage
-----

```
$ gitout --help
gitout 0.1.0

USAGE:
    gitout [FLAGS] <config> <destination>

FLAGS:
        --dry-run    Print actions instead of performing them
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Enable verbose logging

ARGS:
    <config>         Configuration file
    <destination>    Backup directory
```


Configuration specification
---------------------------

```toml
version = 0

[github]
user = "example"
token = "abcd1234efgh5678ij90"

[github.clone]
starred = true  # Optional, default false
watched = true  # Optional, default false
repos = [
  "JakeWharton/gitout",
]

[git.repos]
asm = "https://gitlab.ow2.org/asm/asm.git"
```


Development
-----------

If you have Rust installed, a debug binary can be build with `cargo build` and a release binary with `cargo build --release`.
The binary will be in `target/debug/gitout` or `target/release/gitout`, respectively.
Run all the tests with `cargo test`.
Format the code with `cargo fmt`.
Run the Clippy tool with `cargo clippy`.

If you have Docker but not Rust, run `docker build .` which will do everything. This is what runs on CI.


LICENSE
======

MIT. See `LICENSE.txt`.

    Copyright 2020 Jake Wharton
