# Git Branch Selector

Interactively select Git branches and print them in a way Git can understand.
Use in conjunction with `xargs` or string expansion to perform commands on
multiple git branches.

## Examples

To print your selection of branches

```console
gbs
```

To delete your selection of branches

```console
gbs | xargs git branch -d
```

or

```console
git branch -d $(gbs)
```

## Build, Install, and Test

Do this using the usual `cargo` commands.
