# Command line

[← Home](../Home.md)

> Describes v0.1.0. The target CLI is in [design/tooling](../design/tooling.md).

```
startlang <COMMAND>
```

| Command | Description |
|---------|-------------|
| `run <path>` | Interprets a file. Exit code `0` on success, non-zero on error |
| `repl` | Starts the interactive session |
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

Inside a file or the REPL, `Set DebugLexer.`, `Set DebugParser.` and
`Set DebugTyper.` print the tokens, the parsed command and the typed command.
See the [language tour](./language-tour.md#debug-flags).
