# Input / output

The language is pure: a function called with the same arguments always
returns the same result. I/O must therefore be visible in the types,
otherwise the compiler could remove, reorder or merge I/O calls, and the
proofs would become unsound (`read_line () = read_line ()` would be
provable).

## World token hidden in an `IO` monad

- Internally, I/O primitives take and return an affine **world token**:
  `print : string -> world -> world`. The token orders the effects by data
  dependency, and the affine typing forbids reusing an old world.
- The token is never visible to the user: it is hidden in an `IO α` monad
  (`IO α ≈ world -> world * α`), with `pure` and `bind`.

```
main : array string -> IO i32 :=
  args =>
    bind (print "a") (_ =>
    bind (print "b") (_ =>
    pure 0))
```

With `let*` (v0.5, see [syntax](./syntax.md#monadic-let)):

```
main : array string -> IO i32 :=
  args =>
    let* _ := print "a" in
    let* _ := print "b" in
    pure 0
```

| Step | Version |
|------|---------|
| world token and I/O primitives (internal) | v0.4.0-beta.4 |
| `IO α`, `pure`, `bind` | v0.4.0-beta.5 (polymorphism) |
| `IO` instance of a `Monad` class | v0.4.0-beta.6 (type classes) |
| I/O functions in the stdlib, `main : array string -> IO i32` | v0.4.0-beta.7 |
| `let*` notation for `bind` | v0.5 |
| `IO` re-expressed as an algebraic effect, same user code | v0.9.0-beta.1 |

## Open questions

- [ ] Which primitives: console, files, arguments, time, …?
