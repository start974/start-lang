# Memory and base types

## Value types

Scalars, all copyable:

| Type | Values |
|------|--------|
| `i8`, `i16`, `i32`, `i64`, `isize` | signed machine integers |
| `u8`, `u16`, `u32`, `u64`, `usize` | unsigned machine integers |
| `f32`, `f64` | IEEE 754 floats (literals `1.5`, suffix `1.5f32`) |
| `𝔹` | `true`, `false` |
| `Char` | Unicode characters |
| `unit` | `()` |
| `never` | no value: type of an expression that does not terminate (`panic "message"`), compatible with every type; removed with polymorphism (`panic : string -> α`) |

Typed primitives operate on these types (arithmetic, comparisons, boolean
operators, conversions); the operators `+`, `==`, … are notations defined
on top of them (v0.5).

## Machine integers

Bignums (`ℕ`, `ℤ`, unbounded) are not in the code; they are used in
propositions, where machine integers are related to them (a `u32` seen as a
`ℕ` below `2^32`).

Integer literals take their type from the annotation or the context, with an
optional suffix; without context, an annotation is required.

```
def a : u8 := 42      (* u8 *)
def b := 42u8         (* u8 *)
def c := 42           (* error: ambiguous type *)
```

Allocated types arrive with the ADTs: unions (v0.4.0-beta.1), tuples (v0.4.0-beta.2),
records, `string` and arrays (v0.4.0-beta.3).

## Affine types and `const` (no GC)

No garbage collector. The language is pure (no mutation), so consuming a
value never means modifying it: it means **taking responsibility for it**
(who frees it, who may reuse it in place).

### Consumption

**A value is consumed when it is passed to a function, at a parameter of
type `T` (not `const T`).** Every other use is a read.

| Use | Consumed? |
|-----|-----------|
| `f x` with `f : string -> …` | yes |
| `f x` with `f : const string -> …` | no, read |
| constructors `cons x nil`, `(x, n)`, `{ name := x }` | yes (a constructor is a function) |
| returning `x` | yes |
| `free x` (`free : T -> unit`) | yes |
| `copy x` (`copy : const T -> T`) | no, read |
| `let y := x` | no, renaming: the rules then apply to `y` |
| `match x with …` | **never**: the bound parts are `const` |

A local value is consumed at most once (affine), and every use before the
consumption is a read. It is freed statically after its last use (no counter
at run time); an unused value is freed implicitly at the end of its scope,
with a warning (no warning if it is named `_`).

### `const`

On a function parameter, `const T` means the function only reads the
value, never stores nor returns it, and the caller keeps it (like `&T` in
Rust, but no lifetimes are needed). `const` is also the type of a global
that never changes (`def n : const u16 := 10`), scalars included: it marks
what can be computed at compile time (see below). A parameter that is only read gets a
`const` type by inference; a `pub` function has an explicit type, so its
`const` parameters are always written.

```
pub def length : const string -> usize := …
pub def push   : string -> Char -> string := …        (* consumes the string *)

def f : string -> list string * usize :=
  s => let n := length s in        (* read *)
       (cons s nil, n)             (* consumed by the constructor cons *)

def dup : string -> string * string :=
  x => (x, x)                      (* error: x consumed twice *)
def dup2 : string -> string * string :=
  x => (copy x, x)                 (* ok: two independent strings *)

def head : const list i32 -> i32 :=
| cons x _ => x                    (* match: reads, never consumes *)
| nil => 0
```

### Globals and scalars

- Global definitions are never consumed (a `const` global never changes). Passed to a
  `const` parameter, they are read; passed to a consuming parameter, they
  are implicitly copied.
- Scalars (integers, floats, `𝔹`, `Char`, `unit`) are copied freely; `const`
  is useless on them.

```
def greeting : string := "hello"
def f : unit -> usize := () => length greeting      (* read, no copy *)
def g : unit -> string := () => push greeting '!'   (* implicit copy *)
```

- `copy` and `free` are universal keywords (they may later rely on type
  classes).
- The code generation emits the copies and frees.

## Compile time

No keyword: a function whose arguments are all `const` and which changes
no global (no `[ref …]` marker, see below) has no side effect; applied to
`const` globals, it is computed at compile time and its result written in
the program.

```
pub def length : const string -> usize := …
def s : const string := "hello"
def n : const usize := length s      (* computed at compile time *)
```

## Mutable references (later, design subject to change)

For now, state is threaded through a state monad, updated in place thanks
to the affine typing (see [State](./state.md)). Mutable references come
later: `ref` parameters, and `ref` globals changed only by the functions
that name them in their signature:

```
def counter : ref u16 := 0
def incr [ref counter] : unit -> unit :=
  counter <- counter + 1
```

## Open questions

- [ ] `const` on a global: written, or the default when neither `const` nor
      `ref` is written?
- [ ] Compile time: limit (steps, time) for a computation that does not
      terminate; a `panic` during it is a compilation error.
- [ ] Mutable references: version, syntax (`ref`, `[ref x]`, `<-`), and
      how the proofs (v0.6, v0.7) reason about the `ref` globals.
- [ ] Integer overflow: runtime error, wrapping, or proof obligation?
- [ ] A match never consumes, so a consumed list cannot be reused in place
      (e.g. by `map`): accepted cost, or later optimization?
