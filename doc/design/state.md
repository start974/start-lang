# State

[← Design](./README.md)

## No mutable references

The language has **no mutable references** (`ref`, `:=` on a cell, `!r`).
A mutable cell shared between two places breaks purity: a function could
return different results for the same arguments, the compiler could no longer
reorder or merge calls, and the proofs would have to reason about a heap.

Code that updates a state is written with a **state monad** instead: the
state is passed explicitly, but hidden behind `bind`, like the world token of
[`IO`](./io.md).

## `State σ α`

```
type State σ α := σ -> σ * α
```

| Function | Type | Effect |
|----------|------|--------|
| `pure` | `α -> State σ α` | returns a value, state unchanged |
| `bind` | `State σ α -> (α -> State σ β) -> State σ β` | sequences two steps |
| `get` | `State σ σ` | reads the state (copies it if `σ` is not a scalar) |
| `put` | `σ -> State σ unit` | replaces the state |
| `modify` | `(σ -> σ) -> State σ unit` | updates the state in place |
| `gets` | `(const σ -> α) -> State σ α` | reads a part of the state without copying it |
| `run` | `State σ α -> σ -> σ * α` | runs a computation from an initial state |

- `State σ` is an instance of the `Monad` class, so the generic monad
  functions (`map`, `sequence`, `for_each`, …) work on it.
- It is plain stdlib code: no compiler support is needed.

## Affinity and in-place update

The state is threaded through the computation and never duplicated: each
step consumes the previous state and produces the next one. With the
[affine typing](./memory.md), the compiler knows the old state is dead, so
`modify` and `put` can update it **in place**, with the cost of a mutable
variable and none of its problems. `get` on a non-scalar state copies it;
`gets` and `modify` avoid the copy.

## Writing code easily

Without sugar, `bind` nests:

```
def incr : State i32 unit := modify (n => add n 1)

def count_twice : State i32 i32 :=
  bind incr (_ =>
  bind incr (_ =>
  get))
```

With `let*` (v0.5, see [syntax](./syntax.md#monadic-let)):

```
def count_twice : State i32 i32 :=
  let* _ := incr in
  let* _ := incr in
  get
```

`let*` works for every monad (`IO`, `State σ`, `Option`, …).

## Combining state and I/O

- First: a `StateT σ m` transformer in the stdlib (`StateT σ IO` for a state
  next to I/O), with `lift : m α -> StateT σ m α`.
- With algebraic effects (v0.9.0-beta.1), state becomes an effect with a handler;
  the user code written with `get` / `put` / `modify` stays the same, and
  several effects combine without transformers.

| Step | Version |
|------|---------|
| `State σ α`, `pure`, `bind`, `get`, `put`, `modify`, `gets`, `run` | v0.4.0-beta.5 (polymorphism) |
| `State σ` instance of `Monad` | v0.4.0-beta.6 (type classes) |
| `State` and `StateT` in the stdlib, generic monad functions | v0.4.0-beta.7 |
| `let*` notation | v0.5 |
| state as an algebraic effect, same user code | v0.9.0-beta.1 |

## Open questions

- [ ] `StateT` in v0.4.0-beta.7, or wait for effects and only offer `State`?
- [ ] Guaranteed in-place update for `modify` (checked by the compiler), or
      best-effort optimization?
