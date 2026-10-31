# v0.2.0-beta.1 — Compilation phases

Split the command-by-command loop into phases on the whole source, so that
beta.2 can remove the final dot. **No syntax change.**
Plan: [`doc/roadmap/v0.2-cleanup.md`](doc/roadmap/v0.2-cleanup.md#v020-beta1--compilation-phases) · Design: [`doc/design/pipeline.md`](doc/design/pipeline.md)

```
Source ─► lex ─► tokens ─► parse ─► pir ─► lower ─► sir ─► check ─► type ─► tir ─► run
```

## Done when

- [ ] `cargo test --workspace` passes; the `trycmd` outputs that change (several
      errors reported instead of one) are reviewed in the diff, not only
      regenerated with `TRYCMD=overwrite`.
- [ ] A file with a lexing error, a parsing error and a typing error reports the
      three errors, in source order, and runs nothing.
- [ ] The lexer is called once per file.
- [ ] `typing` and `ir::tir` no longer use `ir::pir`.

---

Each step below ends with the tests passing.

## 1. Names of the middle-end (no behaviour change)

- [x] Rename the crate `cst` → `pir` (*parse IR*): `Cargo.toml` of the workspace,
      `lexer`, `parser`, `startlang`; `use cst::` → `use pir::`.
- [x] One crate `ir` for every representation: modules `ir::pir`, `ir::sir`,
      `ir::tir` (the crates `pir` and `tir` merged into it); clients write
      `use ir::{pir, tir};`.
- [x] New module `ir::sir` (*surface IR*, untyped AST), same module layout as `pir`
      and `tir` (`command`, `definition`, `expression`, `ty`, `pattern`,
      `constant`, …), same type names, a `Span` on every node:
  - [x] one `Expression` (a span + one `ExpressionKind` enum, no `Expression0` /
        `Expression1`), no parentheses;
  - [x] one node per command (`Definition`, `TypeDefinition`, `Eval`, `TypeOf`,
        `Help`, `Set`, `Unset`), no keyword variants (`Def` / `Definition`,
        `Eval` / `$`, …);
  - [x] documentation as text on the definition, no comments, no `Meta`;
  - [x] `sir::File`: the list of commands.
- [ ] New crate `lowering`: `pir::File` → `sir::File`, no errors (`set` options
      stay names, checked later).
- [x] `ir::tir`: stop re-exporting `pir::Documentation` (`ir/src/tir/mod.rs`),
      use the documentation of `sir`; `ir::tir` no longer uses `ir::pir`.
- [ ] `typing`: take `sir` instead of `cst` (`expression`, `ty`, `pattern`,
      `definition`, `type_definition`, `help` in `typing/src/lib.rs`); `typing`
      no longer uses `ir::pir`.
- [ ] Interim: the current loop calls `lowering` between parse and type.

## 2. Lexer on the whole source

- [ ] `lexer/src/lexing.rs`: remove the "until `.`" rule (`token_dot` +
      `token_end`); the `.` becomes an ordinary operator token, the stream
      ends with `EndOfInput`.
- [ ] Recovery: on a bad character or an unclosed comment / literal, record
      the error and continue; `lex` returns the tokens **and** the errors.
- [ ] Drop the offset juggling (`offset`, `get_offset_source`,
      `last_offset`) that exists only to re-lex the rest of the file.
- [ ] Tests: several lexing errors in one file are all reported.

## 3. Parser on the whole token stream

- [ ] `parser`: parse a whole `pir::File` (`file := command* EndOfInput`)
      instead of `CommandOrEnd` one command at a time.
- [ ] Recovery: on a broken command, record one error and skip to the next
      command keyword (`Def`, `Definition`, `Type`, `Ty`, `Eval`, `$`, …) or the
      next `.`; return the file **and** the errors.
- [ ] Remove `CommandOrEnd`.
- [ ] Tests: two broken commands in one file give two errors, the valid
      commands around them are in the `pir::File`.

## 4. Typing on the whole file

- [ ] `typing`: `file(&sir::File, env) -> (tir::File, Errors)`; every command
      is typed, all errors are returned.
- [ ] A definition that fails is recorded as broken; every definition or
      command that uses it is skipped **silently**: only the original error is
      shown (no `Variable not found`, no type error in the users).
- [ ] `tir::File`: the typed commands, in order.

## 5. One driver

- [ ] `startlang/src/pipeline`: load → lex → parse → lower → check → type →
      run, with each phase's errors collected.
- [ ] Nothing runs if any phase before *run* has an error.
- [ ] `errors`: sort errors by position (`Errors::sorted`) before printing.
- [ ] Stdlib loaded **once**, through the driver, into the initial env; remove
      the three copies (`file_interpreter/interpreter.rs`,
      `repl/interpreter.rs`, `lsp/interpreter.rs`).
- [ ] Clients of the driver:
  - [ ] `startlang run`: phases 0–6, exit code;
  - [ ] REPL: phases 1–6 per input, env (typer + VM) kept between inputs;
        input still ends with a line ending in `.` (changes in beta.2);
  - [ ] LSP: diagnostics from all phases; *info* of `TypeOf` / `Help` from
        typing; `Eval` run only in a sandbox (every `.st` in beta.1, `.snbx`
        from beta.2) and only if there is no error;
  - [ ] `startlang format`: phases 0–2 on `pir`.
- [ ] Remove the `Interpreter` trait (`startlang/src/interpreter/interpret.rs`)
      and its three implementations.

## 6. Debug output

- [ ] `startlang run --debug lexer,parser,typer <file>` (also `lowering` to
      print `sir`).
- [ ] REPL: `Set DebugParser.` applies to the **next** inputs.
- [ ] In a file, a debug `Set` is ignored with a warning pointing to `--debug`
      (an error from beta.2).
- [ ] Port `startlang/tests/cmd/{lexer,parser,typer}/debug*.in` to `--debug`.

## 7. Tests

- [ ] Unit tests: `lowering` (one test per `pir` construct), lexer and parser
      recovery, typing without cascade, error sorting.
- [ ] CLI test: one file with a lexing, a parsing and a typing error.
- [ ] Review every changed `.trycmd` in `git diff startlang/tests/cmd`.

## 8. Documentation and release

- [ ] `doc/design/pipeline.md`: update if the implementation differs.
- [ ] `doc/guide/cli.md`: `--debug`.
- [ ] `doc/contributing/development.md`: workspace layout (`ir` with `pir`,
      `sir`, `tir`; `lowering`).
- [ ] Section beta.1 of [`doc/changelog/v0.2.0.md`](doc/changelog/v0.2.0.md#v020-beta1--compilation-phases)
      completed, date set.
- [ ] Roadmap: tick beta.1, *Current version* → `v0.2.0-beta.1`; crate
      versions `0.2.0-beta.1`; git tag `v0.2.0-beta.1`.
