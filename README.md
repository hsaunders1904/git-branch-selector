# Git Branch Selector

Interactively select git branches and print them to stdout.
Use within pipes to perform commands on multiple git branches.

## Usage

To interactively select branches and print your selection,
use the `bs` executable without any arguments.

Use in conjunction with `xargs` to perform git operations on selected branches.
This example demonstrates how to interactively delete branches.

![alt text](./docs/images/usage_example.gif)

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
