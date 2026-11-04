# Compilation phases

[← Design](./README.md)

How a source goes from text to execution. The long-term pipeline (projects,
notations, proofs, Cranelift) is in the [roadmap](../roadmap/roadmap.md#compilation-pipeline-target);
this page fixes the phases of v0.2, which everything after builds on.

## Before (v0.1)

`Interpreter::run` (`startlang/src/interpreter/interpret.rs`) loops **one
command at a time**:

```
rest of the file ─► lex until "." ─► parse one command ─► type ─► run ─► next
```

- The lexer stops at the final `.`: the dot is what separates commands, and
  the rest of the file is lexed again for every command.
- The loop stops at the first error (`continue_parsing`).
- `Set DebugLexer.` changes how the *next* commands are lexed, because
  phases are interleaved.
- The file runner, the REPL and the LSP each implement the `Interpreter`
  trait, with the stdlib loading copied three times.

This cannot survive v0.2.0-beta.3: without the final dot, the lexer cannot find the
end of a command alone.

## Since v0.2.0-beta.1

Each phase runs on the **whole source** and returns its result **and** all
its errors:

```
Source ─► 1. lex ─► tokens ─► 2. parse ─► pir ─► 3. lower ─► sir ─► 4. check ─► 5. type ─► tir ─► 6. codegen ─► lir ─► 7. run
```

### Representations

Every representation is named `<letter>ir`, the letter saying the stage, and
is a module of the single crate **`ir`**: `ir::token`, `ir::pir`, `ir::sir`,
`ir::tir`, `ir::lir` (later `ir::kir`); what several of them share is in the
private module `ir::common`, re-exported by each. `ir` also holds
`ir::location` (sources, spans). Pretty printing (`pp`) and diagnostics
(`errors`) are separate crates: the passes build their diagnostics themselves
(`pass::typing::error`, …), so `ir` does not depend on `errors`. A client imports the modules it needs
(`use ir::{pir, tir};`) and writes `pir::Expression`, `tir::Type`. The trees
`pir`, `sir` and `tir` have the **same module layout** (`command`,
`definition`, `expression`, `ty`, `pattern`, `constant`, …) and the **same type
names**: `pir::Expression`, `sir::Expression`, `tir::Expression` are the same
node at three stages, found at the same path in each module.

| Name | Stands for | Is | Built by | Contains | Used by |
|------|------------|----|----------|----------|---------|
| `token` | tokens | tokens | `lexer` | tokens, comments attached to them | parser |
| `pir` | *parse IR* | CST, concrete syntax tree (renamed from `cst`) | `parser` | everything written: keywords as spelled (`Def` / `Definition`), parentheses, comments, documentation, operators | formatter, LSP (positions, comments) |
| `sir` | *surface IR* | AST, untyped (new) | `lowering` (new) | the structure only: one node per construct, spans, documentation as text; aliases of a keyword merged, parentheses and comments dropped; every definition has a `DefId` and every use is resolved to it (`Identifier`, shown `a@0`) | checks, typing |
| `kir` | *kernel IR* | `sir` without the dynamic grammar (v0.5) | `expand` (v0.5) | every notation of a `.synt` (operators `+`, `==`, `let*`, user notations) replaced by its definition (`a + b` → `add a b`); spans kept to point to the source | typing, proofs (v0.6), obligations (v0.7) |
| `tir` | *typed IR* | typed AST (name kept) | `typing` | every expression has its type, every variable points to its unique definition (`a@0`) | VM, LSP hover, `lir` lowering |
| `lir` | *low IR* | backend IR | `codegen` | backend-independent code ([backend](./backend.md)): the definitions only, a global each (`#0`) with its type; an `Eval` is an expression of `lir` evaluated from the globals | VM, Cranelift (v0.2.0-beta.5) |

Naming rule: a representation is a module `ir::<letter>ir`; a pass going
from one to the next is a module of the crate **`pass`**, named after the
action (`pass::lexer`, `pass::parser`, `pass::lowering`, `pass::expand`,
`pass::typing`, `pass::codegen`, and `pass::vm` until Cranelift). Each pass is
a type built by its builder, with one entry point `run` that takes a whole
file and returns a `CheckedResult`: the next representation, `Checked` (no
error), or a `Partial`: the representation as far as it could be built, and
all its errors (`Lexer::builder().build().run(text)`); `run` takes the
`Checked` result of the pass before, so a pass only runs if the one before
had no error. The lowerer, the typer, the codegen and the vm keep their state
between files (stdlib, REPL inputs); every builder has a `debug` flag that
prints the representation the pass built (`--debug`).

`sir` keeps the notations as the user wrote them, for the LSP (hover, go to
the notation) and for messages; `kir` has no trace of the dynamic grammar, so
typing and proofs never depend on the notations in scope. Until v0.5 there
are no notations: typing runs directly on `sir` and `kir` does not exist yet.

### Phases

| # | Phase | Input → output | Errors | Module of `pass` |
|---|-------|----------------|--------|-------|
| 0 | Load | path / REPL input → `Source` (id, content; `FileKind` in v0.2.0-beta.3) | unreadable file, unknown extension | `ir::location` |
| 1 | Lex | `Source` → all tokens, up to `EndOfInput` | bad characters, unclosed comments / literals; lexing continues after an error | `lexer` |
| 2 | Parse | tokens → `pir::File` (every command) | syntax errors, one per broken command; resynchronizes on the next command start | `parser` |
| 3 | Lower | `pir::File` → `sir::File` | an unknown name, or a name of the wrong kind (expression / type); the command is left out | `lowering` |
| 4 | Check | `sir::File` → `sir::File` | rules of the file kind (v0.2.0-beta.3): commands only in `.snbx`, one `main` in `.scod` | `lowering` |
| 5 | Type | `sir::File` + env → `tir::File` + env | all typing errors; a definition that fails is marked broken, and every definition or command that uses it is skipped silently: only the original error is shown | `typing` |
| 6 | Codegen | `tir::File` + globals → `lir::File` (the globals) + globals; each `Eval` → an expression of `lir` | internal errors only | `codegen` |
| 7 | Run | `lir::File` + env → env (globals defined); each `Eval` evaluated from them → output, exit code | runtime errors | `vm` (Cranelift in v0.2.0-beta.5) |

Later versions insert their phases on `sir`, between *lower* and *type*:
modules (v0.3); from v0.5, notations are read before
parsing and expanded after (`sir` → `kir`), and typing runs on `kir`.

### Rules

- **A phase never stops at its first error**: it reports every error it can
  find, each with its recovery (how the phase went on) and a help (how to
  fix it).
- **Phases are independent**: a phase runs only if the phase before has no
  error (its result is `Checked`); a file with a lexing error reports only
  the lexing errors.
- **Nothing runs if any error was found** before phase 7: no `eval` output
  from a file that does not type-check (already the case today for the VM).
- **Errors are sorted by position** before printing, whatever the phase.
- **No driver**: `pass::stdlib::Stdlib::run` calls the passes one after the
  other (and goes back to the state before the text on error); the file
  runner, the REPL and the LSP share that chain and only differ by the last
  step (`startlang/src/compile.rs` gives the outputs in command order):

| Client | Phases | Last step |
|--------|--------|-----------|
| `startlang run` | 0–7 | prints, exits with the error code (`main` in v0.2.0-beta.3) |
| REPL | 1–7 per input | env (typer + VM) kept between inputs |
| LSP | 0–7 | errors → diagnostics, `tir` → hover; *info* diagnostics: `TypeOf` / `Help` from typing, `Eval` by running, only in a sandbox (every `.st` in beta.1, `.snbx` from beta.3) and only if there is no error |
| `startlang format` | 0–2 | prints the `pir::File` (the formatter never sees `sir`) |

- The stdlib is loaded **once**, through the same phases, into the initial env
  (`pass::stdlib::Stdlib::load`).

### Debug output

Phases are no longer interleaved, so a command inside a file cannot change
how that same file is lexed. Debug output becomes an option of the tools:

- CLI: `startlang run --debug lexer,parser,lowering,typer,codegen file`: each
  phase named prints the representation it built, once it has no error
  (tokens, `pir`, `sir`, `tir`, `lir`), before the outputs of the file.
- `Set` / `Unset` are removed from the language; the REPL has no debug
  output for now.

### Consequences

- Error output changes: a pass with several errors now reports all of them,
  so some `trycmd` snapshots change. These diffs are reviewed, as for v0.2.0-beta.3.
- The lexer drops the "until `.`" rule; the dot stays an ordinary token until
  v0.2.0-beta.3 removes it, so v0.2.0-beta.1 does not change the syntax.
- The REPL needs another way to know an input is complete once the dot is
  gone (v0.2.0-beta.3): the terminal UI of v0.2.0-beta.2 runs the input on
  Enter, and a separate key inserts a new line.

## Open questions

- [ ] Typing errors found on `kir`: shown in the expanded form (`add a b`)
      or mapped back to the notation the user wrote (`a + b`)?
- [x] REPL: end of input without the final dot: the user decides. In the
      terminal UI (v0.2.0-beta.2), Enter runs the whole input and a separate
      key inserts a new line.
