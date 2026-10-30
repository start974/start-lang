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
messages), then `expand` replaces each one by its definition: `kir` has no
trace of the dynamic grammar, and typing and proofs run on it (see
[compilation phases](./pipeline.md#representations)).

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
