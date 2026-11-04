# Tooling

The compiler is a library with a clear API; the CLI and the LSP (crate
`startlang`) are only clients of it, and may move to another repository one
day.

## CLI

Cargo-style: commands work on the project found by walking up to
`projet.toml`; an optional path restricts them to a file or a module.

```
startlang new mylib        # projet.toml, src/main.scod, tests/, .gitignore
startlang check            # type checking + proofs (warning on admit)
startlang build            # native executables (AOT) in target/
startlang run              # runs main
startlang run demo.snbx    # runs a sandbox
startlang test             # unit, integration and doc tests
startlang format [path]    # formats the project or a path
startlang doc              # generates the API documentation
startlang tui              # interactive session (terminal UI)
startlang lsp
```

## `main`

The entry point is declared with the `main` keyword in a `.scod`.

- First version (v0.2.0-beta.3): `main` takes no argument and returns an integer
  used as the exit code (`main : u8 := 0`): `u8`, so that every value is
  an exit code on every system (0 to 255).
- v0.4.0-beta.4 (functions, with `string` and arrays from v0.4.0-beta.3): `main` receives
  the arguments as an array (its size gives the count),
  `main : array string -> u8`.
- v0.4.0-beta.7 (I/O): `main : array string -> IO u8` (see
  [I/O](./io.md)).
- `projet.toml` declares the executables with `[[bin]]` (several allowed),
  each one naming the module that holds its `main`.

## Tests

Written with a `test` keyword; a test is a sequence of
`assert (cond) "message"` and fails at the first false `assert`.

- **Unit tests**: in the `.snbx` of a module (`A.snbx` next to `A.scod`),
  which sees the local names of the module.
- **Integration tests**: `.snbx` files in `tests/`, which only see what the
  project exposes.
- **Doc tests**: code blocks in documentation comments `(** … *)`.

```
(* src/arith.snbx *)
test arith :=
  let x := add 2 2 in
  assert (x == 4) "add";
  assert (div x 2 == 2) "div"
```

## Documentation

Documentation is part of the tooling and evolves at every version:

- language documentation: `doc/reference/grammar.md`,
  `doc/reference/typing.md`, `doc/reference/semantics.md`;
- API documentation generated from the `(** … *)` comments
  (`startlang doc`), with the doc tests.

## REPL

`startlang tui` (v0.2.0-beta.2) replaces the REPL: a full-screen terminal
interface on `crossterm` (`ratatui` + `ratatui-textarea`).

- The outputs and errors of each input scroll above the input area.
- The keys are those of a classic shell (readline): Ctrl+A / Ctrl+E, Ctrl+U /
  Ctrl+K, Ctrl+W, Ctrl+Y, Alt+← / Alt+→ (or Alt+B / Alt+F), Ctrl+P / Ctrl+N.
- Enter runs the whole input; Shift+Enter inserts a new line where the
  terminal reports it (keyboard enhancement, the kitty protocol), Alt+Enter in
  every terminal.
- ↑ on the first line of the input (or Ctrl+P) recalls the previous input, ↓
  on the last line (or Ctrl+N) the next one. An input comes back whole, to
  edit and run again; the history is kept between sessions.
- Pasted text keeps its line breaks and does not run.
- Ctrl+C clears the input (quits on an empty one), Ctrl+D quits on an empty
  input, Ctrl+L clears the outputs, PageUp / PageDown and the mouse wheel
  scroll them (the mouse is captured: Shift+drag selects text); F1 or Alt+H
  shows the keys.
- Without a terminal, `startlang tui` stops with an error.

Later:

- Completion on Tab: the keywords, and the names visible at the cursor (the
  definitions of the stdlib and of the previous inputs, from the scope of
  the lowering), with their kind and type; a list to choose from when
  several match.
- Search in the history on Ctrl+R, as in readline: an incremental search
  through the previous inputs, the match shown whole in the input area, Enter
  to keep it, Ctrl+R again for an older one (Ctrl+R is "redo" in the text area
  until then).
- Panels (proof state) and proofs in it.

## LSP

- Walks up to `projet.toml` and loads the whole project; diagnostics on every
  file; on each change, rechecks the modified file and its dependents.
- Multi-file hover and go-to-definition.
- A standalone `.snbx` is checked with the stdlib and the prelude only.
- Later: view proofs in the editor (proof state panel, replay of tactics),
  generate the lemmas left to prove, reorder proofs (formatter).

## Plugins

Plugins add commands and LSP features. They are written in startlang, which
needs the Rust FFI (see [backend](./backend.md)).

## Open questions

- [ ] Plugin API: what can a plugin hook into (commands, LSP requests,
      pipeline steps)?
