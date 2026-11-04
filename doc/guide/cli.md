# Command line

[← Home](../Home.md)

> Describes v0.2.0-beta.2. The target CLI is in [design/tooling](../design/tooling.md).

```
startlang <COMMAND>
```

| Command | Description |
|---------|-------------|
| `run [--debug <phases>] <path>` | Interprets a file. Exit code `0` on success, non-zero on error |
| `tui` | Starts the interactive session in a terminal UI (see [TUI](./tui.md)); needs a terminal (error 103 otherwise) |
| `format <path>` | Formats a file (see below) |
| `lsp` | Starts the language server on stdio |
| `--help`, `--version` | |

## `format`

| Option | Effect |
|--------|--------|
| `--overwrite` | Rewrites the file in place (default) |
| `--print` | Prints the formatted file |
| `--diff` | Prints a diff with the formatted file; non-zero exit code if it differs |

## Errors

Errors print a code, the source excerpt and labels:

```
[301] Error: Variable not found.
   ╭─[ error.st:1:3 ]
   │
 1 │ ? Not_exists.
   │   ─────┬────
   │        ╰────── Variable Not_exists not found.
```

## Debugging

`--debug` takes a comma-separated list of phases; each one prints the
representation it built, before the outputs of the file:

| Phase | Prints |
|-------|--------|
| `lexer` | the tokens, one per line |
| `parser` | `pir`: the file as written |
| `lowering` | `sir`: one node per construct, every name shown `name@id` |
| `typer` | `tir`: every definition with its type |
| `codegen` | `lir`: a global per definition (`global #2 a@7 : ℕ := 1`), an `eval` per `Eval` |

The phases print in pipeline order, whatever the order given. A phase with
errors prints nothing, and the phases after it do not run:

```
$ startlang run --debug lowering,typer file.st
Definition a@7 := 1.
Eval a@7.
Definition a@7 : ℕ := 1.
Eval a@7.
1
```

The `Set` / `Unset` debug commands of v0.1 are removed.
