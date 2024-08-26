# TODO

## Definition of program
### Grammar
```
prgm :
    definition*

definition :
    expr_def

expr_def :
    "def" ident (type_restr)? := expr

type_restr :
    ":" type

ident :
    [a-z-A-Z_(unicode)]+

type :
    ...
    "(" type ")"

expr :
    ...
    "(" expr ")"
```

### Ast
```
type ident α := {
    name : string,
    id : N,
    extra : α
}

type program α :=
    ident α -> option (definition α)

and definition α :=
    Expr_def : (expr_def α)

and expr_def α := {
    id : ident α,
    ty : ty α,
    body : expr α,
    extra : α
}
```

### Make localised ast
```
type pos := {
    start : N,
    end : N
}

type loc := {
    file_name : string,
    pos_start : pos,
    pos_end : pos
}

```

### Make typed ast
```
type ty := ...

type weak_typed := option ty

```

### pipeline
```
def main :=
| [ f_name ] =>
    parse f_name        (* make a parse tree *)
    |> to_weak_typed    (* make a weak_typed ast *)
    |> infer_type       (* make a localised (typed ast) *)
    |> erase_type       (* make a localised ast *)
    |> erase_localised  (* make a ast *)
    |> eval             (* eval expression *)
| args => error args
```

### typing

infer type and raise error if typing is unsatified

## Number (N and Z)
### Grammar
```
ty :
    ident

expr :
    const

const :
    number

number :
      number_N
    | number_Z

number_N :
      [0-9]+
    | number_N "E" number_Z
    | (0b | 0B) [0-1]+
    | (0o | 0O) [0-7]+
    | (0x | 0x) [0-9-a-f-A-F]+

number_Z :
    '-' number_N
```

### Typing
```
type ty α :=
| Ty_prim (prim_ty α)

type prim_ty α :=
| Number (number α)

type number α :=
| N { extra : α }
| Z { extra : α }
```

### Ast
```
type expression α :=
| Const (const α)

type constant α :=
| Number (number α)

type number α :=
| N { value : N_ty, extra : α }
| Z { value : Z_ty, extra : α }
```

### Interpretation
Interpret with localised ast

### Example

```
def a : N := 10
def b : Z := -10
```

## Type alias
### Grammar
```
definition :
    type_def

type_def :
    type" ident ":=" ty
    ...
```

### Ast
```
type definition α :=
| Type_def : (type_def α)
...

type type_def α := {
    name : ident α,
    ty : ty α,
    extra : α
}
```

### Typing
    add unfolding of type alias in type check

### Interpretation
    nothing todo

### Example
```
type t := N

def a : t := 10
```

## Make function
- define arrow type
- define abstraction
- define application

### Grammar

```
type :
    | type '->' type    (* assoc right *)
    ...

expr :
    | pattern "=>" expr (* abstraction *)
    | expr expr         (* application *) (* assoc left *)
    ...

pattern :
    | ident
    | "(" pattern type_restr ")"
```

### Ast
```
type expr α :=
| Abs {
    ty : ty α,
    binder : (list (pattern α)),
    body : expr α
}
| App {
    ty : ty α,
    fun : expr α,
    arg : expr α,
}
...

type pattern α :=
| Pat_var {
    ty : ty α
    id : ident α
}
```

## typing
```
type ty α :=
| Ty_arrow (ty α) * (ty α)
...
```
## Example

```
def add_1 : N -> N :=
    (x : N) => x + 1
```

## Make product
- product type
- product pattern
- product inference

### Grammar
```
type :
    | type '*' type    (* assoc left *)
    ...

expr :
    | (expr ",")+ expr
    ...

pattern :
    | (pattern ",")+ pattern
    ...
```

### Ast
```
type expr α :=
| Prod (expr α) (expr α)
...

type pattern α :=
| Pat_prod (list (pattern α))
```

### Typing
```
type ty α :=
| Ty_prod (list (ty α))
```

### Example
```
type prod_int := int * int

def mul : prod_int -> int :=
x, y => x * y
```

## Make union type
- add union type constructor
- add pattern

### Grammar
```
type :
    union_type
...

union_type :
      ident (type)*
    | ident ":" type
    | union_type? "|" union_type

pattern :
    ident pattern*
    "|" pattern
```

### ast
```
type pattern α :=
| Pat_union {
    id : ident α,
    pat : list (pattern α),
}
...

```

### Typing
```
type ty α :=
| Ty_union (list (union_ty α))
...

and union_ty α := {
    id : ident α,
    ty : list (ty α)
}

```

### Example
```
type t :=
| A : t
| B : N -> N -> t

type u :=
| A
| B N N

def f : t -> N :=
| A => 0
| B x y => x * y
```

## Polymorphism
- polymorphique type definition
- polymorphique type application

### Grammar
```
expr_def :
    "def" ident ("<" (ident)+ ">"? (":" type)? := expr

type_def :
    "type" ident pattern+ ":=" ty

type :
    | type type (* type application *) (* assoc left *)
    ...
```

### Ast
```
type type_def α := {
    name : ident α,
    binders : list (ident α),
    ty : ty α,
    extra : α
}
```

### Typing
```
type ty α :=
| Ty_app (ty α) (ty α)
...
```

### Example
```
type list α :=
| Nil : list α
| Cons : α -> list α -> list α

def hd <α> : list α -> option α :=
| Nil => None
| Cons a l => Some a
```

## Add constant
- unit
- bool
- char
- string
- Q

### Add recursive definition

### add mutual recursive definition

## Library extends

## Add Prop
- prop type
- operator `/\` `\/` `-->` `forall` `exists`
- encode in type theories

## Add env parameter

add reference
```
type ref a :=
def [r' : ref int] incr : ref int -> int :=
{ r' = r }
 r -> r := !r + 1;
```

## Add Pre cond
## Assert
