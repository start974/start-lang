# Syntax

## Commands

- No final dot: a command ends where the next command keyword starts.
  Newlines and indentation have no meaning; the formatter handles the
  layout.

  ```
  type Nat := ℕ
  def two : Nat := 2
  ```
- Commands (`eval`, `typeof`, `help`, `set`, `unset`) are only allowed in
  `.snbx` files and in the REPL; the other kinds only contain declarations.
- The operator shortcuts (`$` for `eval`, and those of `typeof` and `help`)
  are removed, freeing these symbols for `.synt` notations.

## Keywords

All keywords are lowercase with short names, no long aliases.

| Before               | After           |
|----------------------|-----------------|
| `Definition` / `Def` | `def`           |
| `Type` / `Ty`        | `type`          |
| `Import`             | `use`           |
| `Eval`               | `eval`          |
| `TypeOf`             | `typeof`        |
| `Help`               | `help`          |
| `Set` / `Unset`      | `set` / `unset` |

| Role         | Keywords                                                                                       | Kind       |
|--------------|------------------------------------------------------------------------------------------------|------------|
| Declarations | `def`, `rec`, `type`, `main`, `test`, `lemma`, `proof`, `notation`                             | reserved   |
| Imports      | `use`                                                                                          | reserved   |
| Visibility   | `pub`                                                                                          | reserved   |
| Expressions  | `let`, `in`, `if`, `then`, `else`, `match`, `with`, `where`, `assert`, `copy`, `free`, `panic` | reserved   |
| Commands     | `eval`, `typeof`, `help`, `set`, `unset`                                                       | reserved   |
| File kinds   | `synt`, `code`, `prop` (`proof` is already reserved)                                           | contextual |
| Path roots   | `lib`, `super`, `self`                                                                         | contextual |
| Renaming     | `as`                                                                                           | contextual |
| Types        | `const`                                                                                        | contextual |

A contextual keyword is only special in its position (`const` in a type) and can be used as an
identifier elsewhere.

```
def code := 1          (* ok: code is contextual *)
use code std.nat       (* code is a file kind here *)
def let := 3           (* error: let is reserved *)
```

## Delimiters

- `( … )` for tuples of paths (`use` groups, `pub(…)`), and to open a
  module locally in an expression: `std.nat.(add 1 (mul 2 3))`.
- `{ … }` for preconditions (see [proofs](./proofs.md)).
- No block form.

## Definitions

```
visibility def (rec)? name (: ty)? := e
```

- The type in the header is optional, but mandatory for anything `pub`
  (see [visibility](./visibility.md)).
- Definitions work by case analysis; pattern matching follows the usual
  rules: exhaustiveness check and **no overlap** between cases (no "first
  case wins"). A last `_` case means "all the other cases" and is not an
  overlap.
- No `inline` keyword and no `where` for code: a helper local to one
  expression is written with `let` (`where` only names hypotheses, see
  [proofs](./proofs.md)).

## Monadic `let*`

Code in a monad (`IO`, `State σ`, `Option`, …) is written with `let*`, the
monadic form of `let`:

```
let* p := e in body      ≡      bind e (p => body)
```

- `bind` comes from the `Monad` instance of the type of `e` (v0.4.0-beta.6), so
  `let*` works for every monad.
- `let*` is one token (no space between `let` and `*`); a step whose result
  is not used is bound to `_`.
- It is a notation defined in the stdlib `.synt` (v0.5), not a built-in.

```
def next : State i32 i32 :=
  let* n := get in
  let* _ := put (add n 1) in
  pure n
```

## Free declaration order and recursion

- A declaration can be used before it is defined (in the file, and across
  files since cyclic imports are allowed). Names are resolved in two
  passes; a dependency graph computes the checking order.
- Recursion is explicit: every definition of a cycle must be `def rec`
  (`rec` after `def`), otherwise the cycle is an error. `rec` is only
  allowed on functions.
- Recursive types are explicit too: `type rec`, only on sum types; a cycle
  of type aliases is always an error.

```
pub def rec even : ℕ -> 𝔹 := | 0 => true  | n + 1 => odd n
pub def rec odd  : ℕ -> 𝔹 := | 0 => false | n + 1 => even n

def a : ℕ := b              (* error: cycle without rec *)
def b : ℕ := a
def rec c : ℕ := c          (* error: only functions can be rec *)

type rec list α := | nil | cons α (list α)
type list2 α := | nil | cons α (list2 α)  (* error: recursive without rec *)
type rec t := u                           (* error: alias, not a sum type *)
```

## Equality

- `==` is the computed equality of the code (a boolean); `=` is the
  logical equality of propositions (to confirm).
