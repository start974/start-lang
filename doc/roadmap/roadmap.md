# startlang Roadmap

[← Home](../Home.md) · [Changelog](../changelog/README.md)

Goal: a functional language with a proof assistant, built step by step from
the current single-file language. Each version is small and ends in a
working state.

Current version: **v0.2.0-beta.1**

The decisions (syntax, modules, visibility, memory, proofs, …) are in
[doc/design](../design/README.md); this roadmap only says *when* each part
is delivered. **At every version, the tooling evolves with the language: CLI, LSP,
formatter, tests and documentation (the [wiki](../Home.md): guide, reference,
changelog; generated API docs).**

## Versioning

Versions follow [SemVer 2.0.0](https://semver.org/):

- Each step of a version is a **beta**: `v0.2.0-beta.1`, `v0.2.0-beta.2`, …
  A beta ends in a working state, but the version is not finished.
- When every beta is done, a **release candidate** `v0.2.0-rc.1`: no new
  feature, only the [release checklist](#release-checklist) and fixes
  (`rc.2`, … if something is found).
- The rc becomes the **release** `v0.2.0` unchanged, only the number changes.
- **Patch** releases `v0.2.1`, `v0.2.2`, … contain fixes only.

Order: `v0.2.0-beta.1 < … < v0.2.0-beta.5 < v0.2.0-rc.1 < v0.2.0 < v0.2.1`.
Pre-release identifiers are lowercase. Each pre-release and release is a git
tag (`v0.2.0-beta.1`) and the crate versions in `Cargo.toml` follow it.

### Release checklist

The work of an `rc`:

- [ ] `cargo test --workspace`, `cargo clippy`, `cargo fmt --check` pass.
- [ ] The wiki matches the version: guide, reference (grammar, typing,
      semantics), design pages touched by the version.
- [ ] The examples of the guide run as written.
- [ ] The changelog page of the version is complete: one section per
      pre-release, known issues.
- [ ] Formatter and LSP support everything the version added.
- [ ] Crate versions and `Current version` below updated; git tag.


## Versions

| Version | Title | Pre-releases |
|---------|-------|--------------|
| [v0.2](./v0.2-cleanup.md) | Cleanup and preparation | beta.1 compilation phases · beta.2 cleanup (lowercase keywords, no final dot, `.scod` / `.snbx`, `main`, machine integers) · beta.3 values, `let`, `if`, `panic`, primitives · beta.4 Cranelift backend, `vm` removed · beta.5 tooling foundations · rc |
| [v0.3](./v0.3-multi-file.md) | Multi-file | beta.1 free declaration order · beta.2 modules, `projet.toml`, `use`, prelude · beta.3 visibility · rc |
| [v0.4](./v0.4-adt-functions.md) | ADT, functions and polymorphism | beta.1 unions + affine typing · beta.2 tuples · beta.3 records, `string`, arrays · beta.4 functions · beta.5 polymorphism, `IO` · beta.6 type classes · beta.7 first real stdlib with I/O · rc |
| [v0.5](./v0.5-notations.md) | Dynamic grammar | beta.1 `.synt`, notations · rc |
| [v0.6](./v0.6-logic-proofs.md) | Logic and proofs | beta.1 logic (`.sprp`) · beta.2 proofs (`.sprf`) · rc |
| [v0.7](./v0.7-program-proofs.md) | Program proofs | beta.1 preconditions and proof obligations · rc |
| [v0.8](./v0.8-ecosystem.md) | Ecosystem | beta.1 FFI with Rust · beta.2 plugins · beta.3 packages, incremental compilation · beta.4 WebAssembly · rc |
| [v0.9](./v0.9-advanced-types.md) | Advanced types | beta.1 algebraic effects · beta.2 rationals `ℚ` · beta.3 other advanced types (to define) · rc |
| [v1.0](./v1.0-self-hosting.md) | Stabilization and self-hosting start | frozen formats, stable stdlib, complete documentation; the project can be rewritten in startlang |

## Compilation pipeline (target)

```
projet.toml
   │  1. Project loading: manifest, dependencies, prelude          (v0.3.0-beta.2)
   ▼
src/**  ──► 2. Module discovery, FileKind from the extension       (v0.2.0-beta.2, v0.3.0-beta.2)
   ▼
.synt   ──► 3. Notation shapes                                     (v0.5)
   ▼
all     ──► 4. Parsing with the known notations                    (v0.2.0-beta.2, v0.5)
   ▼
all     ──► 5. Name resolution: `use`, visibility, free order      (v0.3)
   ▼
all     ──► 6. Typing, affinity                                    (v0.2.0-beta.3, v0.4)
   ▼
.scod   ──► 7. Proof obligations                                   (v0.7)
   ▼
.sprp .sprf ► 8. Proof checking                                    (v0.6)
   ▼
.scod .snbx ► 9. Code generation: Cranelift JIT / AOT              (v0.2.0-beta.4)
```

## Long-term goals

- After v1.0: rewrite the compiler in startlang, piece by piece, through the
  FFI (v0.8.0-beta.1), until it is self-hosted.
- LLVM backend for optimized release builds.
