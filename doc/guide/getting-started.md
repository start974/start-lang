# Getting started

[← Home](../Home.md)

> Describes v0.1.0. The syntax changes in v0.2.0-beta.3, see the
> [changelog](../changelog/v0.2.0.md#v020-beta3--cleanup).

## Install

From the repository root:

```sh
cargo install --path startlang
```

This installs the `startlang` binary.

## First file

Create `hello.st`:

```
(** the answer *)
Definition answer : Nat := 42.

Eval answer.
TypeOf answer.
Help answer.
```

Run it:

```sh
startlang run hello.st
```

```
42
Nat
answer : Nat
  the answer
```

## Interactive session

```sh
startlang tui
```

A full-screen session in the terminal: type commands (ending with `.`, over
several lines if you want: Shift+Enter or Alt+Enter), run them with Enter, and
see under each command what it gave. ↑ brings back the previous input, F1 or
Alt+H shows the keys, Ctrl+D quits. See [TUI](./tui.md).

## Editor

`startlang lsp` starts the language server on stdio. For Neovim, see
[startlang.nvim](https://github.com/start974/startlang.nvim).

## Next

- [Language tour](./language-tour.md)
- [Interactive session (TUI)](./tui.md)
- [Command line](./cli.md)
