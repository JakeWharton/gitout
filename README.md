Git Out
=======

A command-line tool to automatically backup Git repositories from GitHub or anywhere.


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
  "android/android-ktx",
]

[git.repos]
asm = "https://gitlab.ow2.org/asm/asm.git"
```
