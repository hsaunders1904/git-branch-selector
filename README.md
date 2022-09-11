# Git Branch Selector

Interactively select git branches and print them to stdout.
Use within pipes to perform commands on multiple git branches.

## Usage

To interactively select branches and print your selection,
use the `bs` (for 'branch-select') executable without any arguments.

Use in conjunction with `xargs` to perform git operations on selected branches.
This example demonstrates how to interactively delete branches.

![alt text](./docs/images/usage_example.gif)

## Configuration

You can make and select your own themes using the application's JSON config file.
Get the path to the config file on your system using:

```console
bs --config
```

See the [JSON schema](./docs/config_schema.json) for the config,
for all available settings.
As a quick example to get started configuring a new theme,
the below produces the theme used in the GIF above:

```json
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
