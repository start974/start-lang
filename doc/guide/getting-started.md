# Getting started

[← Home](../Home.md)

> Describes v0.1.0. The syntax changes in v0.2.0-beta.2, see the
> [changelog](../changelog/v0.2.0.md#v020-beta2--cleanup).

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

## REPL

```sh
startlang repl
```

Type commands ending with `.`; a command can span several lines. `Ctrl-D`
quits. History is kept in `.start-history.txt`.

## Editor

`startlang lsp` starts the language server on stdio. For Neovim, see
[startlang.nvim](https://github.com/start974/startlang.nvim).

## Next

- [Language tour](./language-tour.md)
- [Command line](./cli.md)
