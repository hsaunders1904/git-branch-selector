# Git Branch Selector

[![Build Status](https://github.com/hsaunders1904/git-branch-selector/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/hsaunders1904/git-branch-selector/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/hsaunders1904/git-branch-selector/branch/main/graph/badge.svg?token=BIE69ZU0D8)](https://codecov.io/github/hsaunders1904/git-branch-selector)

Interactively select git branches and print them to stdout.
Use within pipes to perform commands on multiple git branches.

## Usage

To interactively select branches and print your selection,
use the `bselect` (for 'branch-select') executable without any arguments.

Use the `Up`/`Down` arrow keys to navigate branch selection.
Use `Space` to select/deselect an entry,
`Enter` to confirm the selection,
or press `Q` or `Esc` to exit without action.

Use in conjunction with `xargs`
(or [command substitution](https://www.gnu.org/software/bash/manual/html_node/Command-Substitution.html))
to perform operations on selected branches.
This example demonstrates how to interactively delete branches.

![alt text](./docs/images/usage_example.gif)

## Configuration

You can make and select your own themes using the application's JSON config file.
Get the path to the config file on your system using:

```console
bselect --config
```

See the [JSON schema](./docs/config_schema.json) for all available settings.
As a quick example to get started configuring a new theme,
the below produces the theme used in the GIF above:

```json
{
    "theme": "emoji",
    "themes": [
        {
            "name": "emoji",
            "checked_item_prefix": {
                "value": "✓",
                "foreground": "green"
            },
            "unchecked_item_prefix": {
                "value": "✕",
                "foreground": "red"
            },
            "active_item_prefix": {
                "value": "👉 "
            },
            "inactive_item_prefix": {
                "value": "   "
            }
        }
    ]
}
```

## Build, Install, and Test

Do this using the usual `cargo` commands:

```console
cargo build
```

```console
cargo install --path .
```

```console
cargo test
```

Or grab the
[latest release](https://github.com/hsaunders1904/git-branch-selector/releases/latest) package or installer.
