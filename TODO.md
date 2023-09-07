# TODO

## Typed Ast
- To infer (not strict)
- Strict
Make a document for inference and type check.

## Basic libray
- bool
- int
- char
- string
- printing

## Evaluation
document of semantic rules

## Next feature

### Forall type
use char `'`
```
app (f : 'a -> 'b) (x: 'a) := f x
```

### Union Type
*constructor*
```
option := fn (a: type) =>
| none : option a
| some : a -> option a

option :=
| none a : option a
| some a : a -> option a

option (a: type) : type :=
| none : option a
| some : a -> option a
```

*destructor*
use case
```
app (f: 'a -> 'b) (x: 'a option) : 'b option :=
case x of
| none => none
| some x => some (f x)
```
use function pattern

```
app (f: 'a -> 'b) :=
| none => none
| some x => some (f x)

app := fn (f: 'a -> 'b)
| none => none
| some x => some (f x)
```

### As pattern
```
f ((x, y) as p) := ...
```

### Proof
Add *Pre* and *Pos* condition

```
{ P } f arg { Q } :=
    ...
Proof
    ...
End.
```

### Modules


## Compilation to C
