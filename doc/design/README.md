# Design

[← Home](../Home.md)

Decisions taken for startlang, by topic. The [roadmap](../roadmap/roadmap.md)
says *when* each part is delivered; these files say *what* it is.

| File | Topic |
|------|-------|
| [pipeline.md](./pipeline.md) | Compilation phases: lex, parse, check, type, run on the whole source; one pass after the other |
| [file-kinds.md](./file-kinds.md) | Extensions, `FileKind`, sandbox, import table between kinds |
| [syntax.md](./syntax.md) | Commands, keywords, delimiters, definitions, recursion |
| [memory.md](./memory.md) | Value types, affine types and `const` (no GC) |
| [backend.md](./backend.md) | Cranelift JIT / AOT, Rust FFI |
| [tooling.md](./tooling.md) | CLI, `main`, tests, documentation, REPL, LSP, plugins |
| [modules.md](./modules.md) | Module layout, `use`, `projet.toml`, prelude, std |
| [visibility.md](./visibility.md) | `pub(…)` markers, re-exports, explicit types |
| [notations.md](./notations.md) | `.synt`, three-phase parsing |
| [io.md](./io.md) | Input / output: world token hidden in an `IO` monad |
| [state.md](./state.md) | No mutable references: state monad, in-place update, `StateT` |
| [proofs.md](./proofs.md) | Logic, proofs, preconditions and obligations |
