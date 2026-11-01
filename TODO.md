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
  - [x] documentation as text, no comments, no `Meta`;
  - [x] a `DefId` on every expression and type definition, given by
        `lowering` in source order (a redefinition gets a new one); uses keep
        their name until name resolution;
  - [x] `sir::File`: the list of commands and a `DefTable` (`DefId` → name,
        kind, place, documentation; name → its `DefId`s in source order); the
        place is the `SourceId` of the table, the span of the name and the
        span of the whole definition (`location(id)`, `name_location(id)`).
- [x] New crate `lowering`: `pir::File` → `sir::File`, no errors (`set` options
      stay names, checked later); a `Lowering` keeps the `sir::File` being
      built, so the command-by-command loop can lower one command at a time.
- [x] `ir::tir`: stop re-exporting `pir::Documentation` (`ir/src/tir/mod.rs`),
      use the documentation of `sir`; `ir::tir` no longer uses `ir::pir`.
- [x] `typing`: take `sir` instead of `cst` (`expression`, `ty`, `pattern`,
      `definition`, `type_definition`, `help` in `pass/src/typing.rs`); `typing`
      no longer uses `ir::pir`.
- [x] Interim: the current loop calls `lowering` between parse and type.

## 2. Lexer on the whole source

- [x] `pass/src/lexer/lexing.rs`: remove the "until `.`" rule (`token_dot` +
      `token_end`); the `.` becomes an ordinary operator token, the stream
      ends with `EndOfInput`.
- [x] Recovery: on a bad character, record the error and continue; every
      lexing error is reported (`Lexer::run -> Partial<tokens>`).
- [x] Drop the offset juggling (`offset`, `get_offset_source`,
      `last_offset`) that exists only to re-lex the rest of the file;
      `Lexer::builder().offset(n)` only places a REPL input in its source.
- [x] Tests: several lexing errors in one file are all reported.

## 3. Parser on the whole token stream

- [x] `parser`: parse a whole `pir::File` (`file := command* EndOfInput`)
      instead of `CommandOrEnd` one command at a time.
- [x] Recovery: on a broken command, record one error and skip to the next
      `.`; every parsing error is reported (`Parser::run -> Partial<pir::File>`).
- [x] Remove `CommandOrEnd`.
- [x] Tests: two broken commands in one file give two errors.

## 4. Typing on the whole file

- [x] `typing`: `Typer::run(&sir::File) -> Partial<tir::File>`; every
      command is typed, all errors are returned; on error the environment is
      left as before the file.
- [x] A definition that fails is recorded as broken; every definition or
      command that uses it is skipped **silently**: only the original error is
      shown (no `Variable not found` in the users).
- [x] `tir::File`: the typed commands, in order (`Eval`, `TypeOf`, `Help`,
      definitions).

## 5. One pass after the other

- [x] Crate `pass` (first named `driver`) with every pass as a module (`lexer`, `parser`,
      `lowering`, `typing`, `codegen`, `vm`; the crates merged into it) and
      the tokens in `ir::token`, `location` in `ir::location`; `pp` and
      `errors` stay crates (`ir` → `pp`, `errors` → `ir`): `tir` errors are
      data, their diagnostics are made in `pass::typing`.
- [x] No driver object and no events: each pass is a type built by its
      builder (`Lexer::builder()…build()`) with one entry point, `run`, taking
      a whole file and returning a `pass::Partial`: what it could build and
      **all** its errors; each error says how the pass corrected it to go on
      (`Error::with_recovery`: character skipped, command skipped up to the
      next `.`, command left out); the tools call the passes one after the other
      (`startlang/src/compile.rs`) and go to the next pass only if there is no
      error (`Partial::into_result()?`).
- [x] Nothing runs if any phase before *run* has an error (the first pass with
      errors stops the chain).
- [ ] Errors of **several** phases in one report (`pipeline.md`: parsing runs
      on the tokens around a lexing error, typing on the commands that
      parsed); each pass already returns its partial result, but the tools
      stop at the first failing pass.
- [ ] `ir::errors`: sort errors by position (`Errors::sorted`) before printing.
- [x] Stdlib loaded **once** (`pass::stdlib::Stdlib::load`): the state of
      the typer, codegen and vm the tools start from.
- [x] Tools:
  - [x] `startlang run`: every pass, exit code (`startlang/src/run.rs`);
  - [x] REPL: every pass per input, typer + codegen + vm kept between
        inputs; input still ends with a line ending in `.` (changes in beta.2);
  - [x] LSP: errors → diagnostics; *info* of `Eval`, `TypeOf`, `Help`, only
        if there is no error (`startlang/src/lsp/diagnostics.rs`);
  - [x] `startlang format`: lexer and parser on `pir`.
- [x] Remove the `Interpreter` trait and its three implementations;
      `startlang/src/printer.rs` prints the outputs on a terminal (`run`,
      REPL).

- [ ] Errors also advise how to fix them (a `help` next to `recovery` in
      `errors::Error`, ariadne `with_help`).

## 6. Debug output

- [ ] `startlang run --debug lexer,parser,typer <file>` (also `lowering` to
      print `sir`).
- [ ] REPL: `Set DebugParser.` applies to the **next** inputs.
- [ ] In a file, a debug `Set` is ignored with a warning pointing to `--debug`
      (an error from beta.2). For now, `Set DebugLexer/Parser/Typer` are
      accepted and have no effect.
- [ ] Port `startlang/tests/cmd/{lexer,parser,typer}/debug*.in` to `--debug`.

## 7. Tests

- [ ] Unit tests: `lowering` (one test per `pir` construct), lexer and parser
      recovery, typing without cascade, error sorting.
- [ ] CLI test: one file with a lexing, a parsing and a typing error.
- [ ] Review every changed `.trycmd` in `git diff startlang/tests/cmd`.

## 8. Documentation and release

- [ ] `doc/design/pipeline.md`: update if the implementation differs.
- [ ] `doc/guide/cli.md`: `--debug`.
- [ ] `doc/contributing/development.md`: workspace layout (`ir` with `token`,
      `pir`, `sir`, `tir`, `lir`; `pass` with every pass).
- [ ] Section beta.1 of [`doc/changelog/v0.2.0.md`](doc/changelog/v0.2.0.md#v020-beta1--compilation-phases)
      completed, date set.
- [ ] Roadmap: tick beta.1, *Current version* → `v0.2.0-beta.1`; crate
      versions `0.2.0-beta.1`; git tag `v0.2.0-beta.1`.
