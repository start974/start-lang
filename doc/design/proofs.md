# Logic, proofs and preconditions

## Principles

- **No dependent typing**: propositions talk about the code, but types never
  depend on terms. A lemma is always referenced by its name and never
  instantiated by hand (`lemma n` is not allowed); the checker instantiates
  it automatically.
- Lemmas are stated in `.sprp`, proved in `.sprf`; a proof is never
  referenced (see [file kinds](./file-kinds.md)).
- Free layout: no fixed `.sprf` ↔ `.sprp` mapping; a proof can use a lemma
  proved further down (circular proofs are rejected).
- `check` checks the proofs and warns on every `admit`.

## Proof language

To decide: tactics (possibly written directly with proof terms) or proof
trees. Core tactics: `intro`, `apply`, `exact`, `rewrite`, `induction`, …;
combinators: sequencing, `try`, `repeat`, …; tactics can be defined in
`.synt` files.

## Preconditions and proof obligations

A `.scod` exports proof obligations generated from the code (termination,
preconditions so the program does not crash, …). A precondition is named and
written after the parameters of a case, before `=>`; each case can have its
own.

```
visibility def name (: ty)? :=
  params ({H : formula})* => e
```

```
pub def div : ℕ -> ℕ -> ℕ :=
  x y {nz : y ≠ 0} => …

pub def get : list α -> ℕ -> α :=
  l i {ne : l ≠ [], lt : i < length l} => …
```

### At the call site

Positional, names optional, combinable (positional first, then named); each
item is a lemma (or a precondition of the caller) or a hole `_`; missing
preconditions are holes.

```
div x 2 {two_ne_zero}               (* lemma *)
div x y {h}                         (* caller's precondition *)
get l 2 {l_ne, two_lt}              (* positional *)
get l 2 {lt := two_lt}              (* named, ne is a hole *)
get l 2 {_}                         (* every precondition is a hole *)
```

- **Hole**: the compiler searches a compatible lemma in the local context
  (preconditions of the caller, `let` bindings), then in the visible lemmas.
  Found → warning naming it (the LSP replaces the hole); not found → error.
- **New obligations** have no syntax: from an unsolved hole, an LSP action
  generates a named lemma (statement in the `.sprp` of the module, stub in
  its `.sprf`) and replaces the hole.
- An obligation is proved once, in any `.sprf` of the project.

### Per-case preconditions

The compiler generates one global precondition `f.pre` by the same case
analysis. The caller gives one proof of it (`f a {p}`) or decomposes by case
with `|`; the cases are exactly those of the callee (no finer patterns, no
overlap), and a last `| _ => _` solves the remaining cases automatically.

```
pub def f : ℕ -> ℕ :=
| 0     {h1 : P} => …
| n + 1 {h2 : Q n, h3 : R n} => …

f a {
  | 0     => {p_lemma}
  | n + 1 => {h2 := q_lemma, h3 := _}
}
f a { | n + 1 => {h3 := _} | _ => _ }
```

### `where`

Hypotheses can be named at the call site and defined in a `where` at the
bottom of the definition (it only binds hypotheses, never code), so the proof
context is not polluted and lemmas can be deconstructed on the fly.

```
pub def g : ℕ -> ℕ :=
  a => f a {H}
where
  H  := { 0 => H1 | n + 1 => H2 }
  H1 := lemma1
  H2 := _
```

## Tooling

- LSP: goals and hypotheses, view of proofs done in the editor (proof state
  panel, replay), generation of lemmas left to prove, formatter ordering the
  proofs like their statements in `.sprp` / `.scod`.
- Proofs can also be done in the REPL (terminal UI).

## Open questions

- [ ] Tactics, proof terms, or proof trees?
- [ ] Classical or intuitionistic logic? HOL-style logic over the code types?
- [ ] Exact syntax of preconditions and call-site arguments (draft).
- [ ] Can a library export (`pub(*)`) code whose obligations are unproved?
- [ ] Can the code run while some obligations are unproved?
