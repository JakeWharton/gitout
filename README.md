Git Out
=======

A command-line tool to automatically backup Git repositories from GitHub or anywhere.

The `gitout` tool will clone git repos from GitHub or any other git hosting service.
If the repository was already cloned, it will fetch any updates to keep your local copy in sync.

When you add your GitHub username and a token, `gitout` will discover all of your owned repositories and synchronize them automatically.
You can opt-in to having repositories that you've starred or watched synchronized as well.

The cloned repositories are [bare](https://www.saintsjd.com/2011/01/what-is-a-bare-git-repository/).
In other words, there is no working copy of the files for you to interact with.
If you need access to the files, you can `git clone /path/to/bare/repo`.


Installation
------------

### Rust

If you have Rust installed you can install the binary by running `cargo install gitout`.

[![Latest version](https://img.shields.io/crates/v/gitout.svg)](https://crates.io/crates/gitout)

### Docker

The binary is available inside the `jakewharton/gitout` Docker container which can run it as a cron job.

[![Docker Image Version](https://img.shields.io/docker/v/jakewharton/gitout?sort=semver)][hub]
[![Docker Image Size](https://img.shields.io/docker/image-size/jakewharton/gitout)][layers]

 [hub]: https://hub.docker.com/r/jakewharton/gitout/
 [layers]: https://microbadger.com/images/jakewharton/gitout

Mount a `/data` volume which is where the repositories will be stored.
Mount the `/config` folder which contains a `config.toml` or mount a `/config/config.toml` file directly.
Specify a `CRON` environment variable with a cron specifier dictating the schedule for when the tool should run.

```
$ docker run -d \
    -v /path/to/data:/data \
    -v /path/to/config.toml:/config/config.toml \
    -e "CRON=0 * * * *" \
    jakewharton/gitout
```

For help creating a valid cron specifier, visit [cron.help](https://cron.help/#0_*_*_*_*).

To be notified when sync is failing visit https://healthchecks.io, create a check, and specify the ID to the container using the `HEALTHCHECK_ID` environment variable (for example, `-e "HEALTHCHECK_ID=..."`).

To write data as a particular user, the `PUID` and `PGID` environment variables can be set to your user ID and group ID, respectively.

If you're using Docker Compose, an example setup looks like;
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
      - "PUID=..."
      - "PGID=..."
```

Note: You may want to specify an explicit version rather than `latest`.
See https://hub.docker.com/r/jakewharton/gitout/tags or `CHANGELOG.md` for the available versions.

### Binaries

TODO GitHub releases download binaries https://github.com/JakeWharton/gitout/issues/8


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

Until version 1.0 of the tool, the TOML version is set to 0 and may change incompatibly between 0.x releases.
You can find migration information in the `CHANGELOG.md` file.

```toml
version = 0

[github]
user = "example"
token = "abcd1234efgh5678ij90"

[github.clone]
starred = true  # Optional, default false
watched = true  # Optional, default false
# Extra repos to synchronize that are not owned, starred, or watched by you.
repos = [
  "JakeWharton/gitout",
]

# Repos not on GitHub to synchronize.
[git.repos]
asm = "https://gitlab.ow2.org/asm/asm.git"
```

### Creating a GitHub token

  1. Visit https://github.com/settings/tokens
  2. Click "Generate new token"
  3. Type "gitout" in the name field
  4. Select the "repo" and "read:user" scopes
     - `repo`: Needed to clone private repositories (if you only have public repositories then just `public_repo` will also work)
     - `read:user`: Needed to traverse your owned, starred, and watched repo lists
  5. Select "Generate token"
  6. Copy the value into your `config.toml` as it will not be shown again


Development
-----------

If you have Rust installed, a debug binary can be built with `cargo build` and a release binary with `cargo build --release`.
The binary will be in `target/debug/gitout` or `target/release/gitout`, respectively.
Run all the tests with `cargo test`.
Format the code with `cargo fmt`.
Run the Clippy tool with `cargo clippy`.

If you have Docker but not Rust, run `docker build .` which will do everything. This is what runs on CI.


LICENSE
======

MIT. See `LICENSE.txt`.

    Copyright 2020 Jake Wharton
