# Language tour

[← Home](../Home.md)

> Describes v0.1.0. The formal definition is in the
> [grammar](../reference/grammar.md), [typing](../reference/typing.md) and
> [semantics](../reference/semantics.md) pages.

A file is a sequence of **commands**, each ending with a `.`.

## Definitions

```
Definition a := 3.
Def b : Nat := a.            (* with a type annotation *)
```

## Type aliases

```
Type N := Nat.
Ty N1 := N.
Def c : N1 := 4.
```

Aliases are unfolded when types are compared.

## Builtin types

| Type | Alias | Literals |
|------|-------|----------|
| `ℕ` | `Nat` | `42`, `1_000`, `0x2A`, `0o52`, `0b101010` |
| `𝔹` | `Bool` | `true`, `false` |
| `Char` | | `'a'`, `'τ'`, `'\n'`, `'\x7E'`, `'\126'`, `'\o176'`, `'\u{3C4}'` |

## Expressions

- a variable: `a`
- a literal: `42`, `'a'`
- parentheses: `(a)`
- a type restriction: `(a : Nat)`

## Commands that print

| Command | Short | Prints |
|---------|-------|--------|
| `Eval e.` | `$ e.` | the value of `e` |
| `TypeOf e.` | `?: e.` | the type of `e` |
| `Help x.` | `? x.` | the type or definition of `x`, and its documentation |

## Debug flags

```
Set DebugParser.
Eval a.
Unset DebugParser.
```

Flags: `DebugLexer`, `DebugParser`, `DebugTyper`.

## Comments and documentation

```
(* a comment *)

(** a documentation comment,
    shown by Help *)
Def d := 0.
```
