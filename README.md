# Git Out

A command-line tool to automatically backup Git repositories from GitHub or anywhere.

The `gitout` tool will clone git repos from GitHub or any other git hosting service.
If the repository was already cloned, it will fetch any updates to keep your local copy in sync.

When you add your GitHub username and a token, `gitout` will discover all of your owned repositories and synchronize them automatically.
You can opt-in to having repositories that you've starred or watched synchronized as well.

The cloned repositories are [bare](https://www.saintsjd.com/2011/01/what-is-a-bare-git-repository/).
In other words, there is no working copy of the files for you to interact with.
If you need access to the files, you can `git clone /path/to/bare/repo`.


## Installation

### Docker

The binary is available inside the `jakewharton/gitout` Docker container,

[![Docker Image Version](https://img.shields.io/docker/v/jakewharton/gitout?sort=semver)][hub]
[![Docker Image Size](https://img.shields.io/docker/image-size/jakewharton/gitout)][layers]

 [hub]: https://hub.docker.com/r/jakewharton/gitout/
 [layers]: https://microbadger.com/images/jakewharton/gitout

Mount a `/data` volume which is where the repositories will be stored.
Mount the `/config` folder which contains a `config.toml` or mount a `/config/config.toml` file directly.

By default, the tool will run a single sync and then exit.
If you specify the `GITOUT_CRON` environment variable with a valid cron specifier, the tool will not exit and perform automatic syncs in accordance with the schedule.

```
$ docker run -d \
    -v /path/to/data:/data \
    -v /path/to/config.toml:/config/config.toml \
    -e "GITOUT_CRON=0 * * * *" \
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
      - "GITOUT_CRON=0 * * * *"
      #Optional:
      - "GITOUT_HC_ID=..."
```

Note: You may want to specify an explicit version rather than `latest`.
See https://hub.docker.com/r/jakewharton/gitout/tags or `CHANGELOG.md` for the available versions.

### Binaries

A `.zip` can be downloaded from the [latest GitHub release](https://github.com/JakeWharton/gitout/releases/latest).

The Java Virtual Machine must be installed on your system to run.
After unzipping, run either `bin/gitout` (macOS, Linux) or `bin/gitout.bat` (Windows).


## Usage

```
$ gitout --help
Usage: gitout [<options>] <config> <destination>

Options:
  --version            Show the version and exit
  -v, --verbose        Increase logging verbosity. -v = informational, -vv = debug, -vvv = trace
  -q, --quiet          Decrease logging verbosity. Takes precedence over verbosity
  --dry-run            Print actions instead of performing them
  --cron=<expression>  Run command forever and perform sync on this schedule
  --hc-id=<id>         ID of Healthchecks.io service to notify
  --hc-host=<url>      Host of Healthchecks.io service to notify. Requires --hc-id
  -h, --help           Show this message and exit

Arguments:
  <config>       Configuration TOML
  <destination>  Backup directory
```


## Configuration

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
# Repos temporary or otherwise that you do not want to be synchronized.
ignored = [
  "JakeWharton/TestParameterInjector",
]

# Repos not on GitHub to synchronize.
[git.repos]
asm = "https://gitlab.ow2.org/asm/asm.git"
```

### Creating a GitHub token

  1. Visit https://github.com/settings/tokens
  2. Click "Generate new token"
  3. Type "gitout" in the name field
  4. Select the "repo", "gist", and "read:user" scopes
     - `repo`: Needed to discover and clone private repositories (if you only have public repositories then just `public_repo` will also work)
     - `gist`: Needed to discover and clone private gists (if you only have public gists then this is not required)
     - `read:user`: Needed to traverse your owned, starred, and watched repo lists
  5. Select "Generate token"
  6. Copy the value into your `config.toml` as it will not be shown again


# LICENSE

MIT. See `LICENSE.txt`.

    Copyright 2020 Jake Wharton
