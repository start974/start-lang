# File kinds

## Extensions

Each file kind is named after what it adds to a module.

| Extension | `FileKind` | Kind in `use` | Content                       | Introduced |
|-----------|------------|---------------|-------------------------------|------------|
| `.scod`   | `Code`     | `code`        | Executable code               | v0.2.0-beta.3     |
| `.snbx`   | `Sandbox`  | —             | Sandbox, never importable     | v0.2.0-beta.3     |
| `.synt`   | `Syntax`   | `synt`        | Syntax, notations and tactics | v0.5       |
| `.sprp`   | `Prop`     | `prop`        | Propositions and logic        | v0.6.0-beta.1     |
| `.sprf`   | `Proof`    | `proof`       | Proofs                        | v0.6.0-beta.2     |

- Every default extension starts with `s`.
- The defaults can be overridden per project in `projet.toml`
  (see [modules](./modules.md)); custom extensions are free (no `s` rule),
  but each kind keeps a distinct extension.
- The legacy `.st` format is removed (no alias, no compatibility). An unknown
  extension is an error listing the expected ones, with a dedicated message
  for `.st`.

## Source

`SourceId` keeps describing where the code comes from; a `Source` adds its
kind:

```rust
pub struct Source {
    pub id: SourceId,
    pub kind: FileKind,
}
```

- REPL input is always of kind `Sandbox`.
- A file outside a project (no `projet.toml`) is always a sandbox and needs
  no manifest. The other kinds require a project. Until projects exist
  (v0.3.0-beta.2), a standalone `.scod` is allowed.

## Sandbox `.snbx`

- Can import everything except proofs; never importable.
- The only place (with the REPL) where commands (`eval`, `typeof`, `help`)
  are allowed.
- Holds unit tests (see [tooling](./tooling.md)).
- Relaxed rules: `admit` and holes allowed (warnings in `check`).

## Import table between kinds

Row imports column.

| File ↓ imports → | `synt` | `code` | `prop` | `proof` | `.snbx` |
|------------------|--------|--------|--------|---------|---------|
| `.synt`          | ✅     | ✅     | ✅     | ❌      | ❌      |
| `.scod`          | ✅     | ✅     | ❌ (1) | ❌      | ❌      |
| `.sprp`          | ✅     | ✅     | ✅     | ❌      | ❌      |
| `.sprf`          | ✅     | ✅     | ✅     | ❌      | ❌      |
| `.snbx`          | ✅     | ✅     | ✅     | ❌      | ❌      |

(1) Becomes ✅ with preconditions (v0.7), only to write preconditions and
proof obligations; propositions are never executed.

- **A proof is never referenced.** Every lemma is stated in a `.sprp`; a
  `.sprf` only contains proofs. Any file refers to a lemma through its
  statement (a tactic in a `.synt`, a `.snbx` printing the proof of a
  lemma, …). `.sprf` files are leaves of the dependency graph and can be
  checked last, or in parallel.
- The files of one module share their local names without `use`, following
  the same table (e.g. `A.sprp` sees the local names of `A.scod`, so a lemma
  can talk about a local helper). A module is a single namespace: two files
  of the same module cannot define the same name.

## Matrix

|                              | `.synt`          | `.scod`         | `.sprp`                     | `.sprf`          | `.snbx`          |
|------------------------------|------------------|-----------------|-----------------------------|------------------|------------------|
| Can import                   | synt, code, prop | synt, code (1)  | synt, code, prop            | synt, code, prop | synt, code, prop |
| Can be imported by           | all              | all             | synt, prop, proof, snbx (1) | none             | none             |
| Executed / checked           | ?                | executed        | checked                     | checked          | executed         |
| Commands (`eval`, `typeof`…) | no               | no              | no                          | no               | yes              |

## Open questions

- [ ] Allowed constructs per kind.
