# Notations (`.synt`)

User-defined syntax extensions and tactics. An imported `.synt` changes how
the importing files are parsed, and can provide tactics to `.sprf` files.

## Three-phase pipeline

`.synt` ↔ `.scod` cycles are allowed; only cycles between `.synt` files are
forbidden, since a notation may use another notation to be parsed.

1. Read the shape of every notation (pattern, precedence, associativity),
   without resolving their bodies.
2. Parse the other files with the known notations.
3. Resolve names everywhere (`+` → `add`), cycles allowed.

After parsing, the notations stay in `sir` (as written, for the LSP and
messages); the resolution resolves the notations like the names (the body
of a notation in the scope where it is defined: hygiene for free), then the
`elaboration` (`kir` →* `kir`) replaces each one by its definition, looped
until none is left: each node of `kir` keeps its origin (the notation it
comes from), so that the errors found by typing and proofs speak of the
notation the user wrote (see [compilation phases](./pipeline.md#representations)).

- **Errors on the fly**: a notation can raise its own errors during its
  elaboration, and rewrite the errors found on its expansion in its own
  words.
- **No sugar in the compiler**: after the dynamic grammar, every sugar is a
  notation; the compiler only knows the kernel.

```
(* std/nat.synt *)
use code std.nat.add
pub notation x "+" y := add x y   (level 50, left)

(* std/nat.scod *)
use synt std.nat
pub def add : ℕ -> ℕ -> ℕ := …
pub def double : ℕ -> ℕ := x => x + x
```

## Open questions

- [ ] Simple notations or full grammar rules?
- [ ] Notation scope: file, module, global?
- [ ] Parsing algorithm (Pratt, Earley, GLR…)?
- [ ] Can a notation be used inside the `.synt` that defines it?
- [ ] Syntax of the errors of a notation: how it raises an error and
      rewrites the errors on its expansion.
