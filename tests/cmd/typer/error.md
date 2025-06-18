# Error

## Type not found
```
$ startlang def_expr_type_not_found.st
? 45
[301] Error: Variable not found.
   ╭─[ def_expr_type_not_found.st:1:9 ]
   │
 1 │ Def a : Not_definied := 4.
   │         ──────┬─────  
   │               ╰─────── Variable "Not_definied" not found in the current scope.
───╯

```

## Variable not found

```
$ startlang var_not_found.st
? 45
[301] Error: Variable not found.
   ╭─[ var_not_found.st:1:6 ]
   │
 1 │ Eval not_defined.
   │      ─────┬─────  
   │           ╰─────── Variable "not_defined" not found in the current scope.
───╯

```


```
$ startlang def_type_not_found.st
? 45
[301] Error: Variable not found.
   ╭─[ def_type_not_found.st:1:11 ]
   │
 1 │ Type T := Not_definied.
   │           ──────┬─────  
   │                 ╰─────── Variable "Not_definied" not found in the current scope.
───╯

```

## Type mismatch

```
$ startlang type_mismatch.st
? 101
Definition a : N :=
  1.
def a_0 : N :=
  1
Definition b : B :=
  true.
def b_0 : B :=
  true_0
Definition c : N :=
  b.
[302] Error: Type mismatch.
   ╭─[ type_mismatch.st:8:5 ]
   │
 8 │ Def c : N := b.
   │     ┬  
   │     ╰── Expect Type B.
   │ 
   │ Note: Expected type: B.
   │       Found type:    N
───╯
Definition c' : B :=
  a : B.
[302] Error: Type mismatch.
    ╭─[ type_mismatch.st:10:16 ]
    │
 10 │ Def c' : B := (a : B).
    │                ┬  
    │                ╰── Expect Type N.
    │ 
    │ Note: Expected type: N.
    │       Found type:    B
────╯
Type N1 := N.
Definition d : N1 :=
  c.
def d_0 : N1 :=
  c_0

thread 'main' panicked at src/vm/env.rs:34:29:
Variable c_0 not found
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```
