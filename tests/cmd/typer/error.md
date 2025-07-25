# Error

## Type not found
```
$ startlang run def_expr_type_not_found.st
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
$ startlang run var_not_found.st
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
$ startlang run def_type_not_found.st
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
$ startlang run type_mismatch.st
? 1
Definition a : ℕ :=
  1.
Definition a_0 : ℕ :=
  1
Definition b : 𝔹 :=
  true.
Definition b_0 : 𝔹 :=
  true_0
Definition c : ℕ :=
  b.
[302] Error: Type mismatch.
   ╭─[ type_mismatch.st:8:5 ]
   │
 8 │ Def c : ℕ := b.
   │     ┬  
   │     ╰── Expect Type 𝔹.
   │ 
   │ Note: Expected type: 𝔹.
   │       Found type:    ℕ
───╯
Definition c' : 𝔹 :=
  a : Bool.
[302] Error: Type mismatch.
    ╭─[ type_mismatch.st:10:16 ]
    │
 10 │ Def c' : 𝔹 := (a : Bool).
    │                ┬  
    │                ╰── Expect Type ℕ.
    │ 
    │ Note: Expected type: ℕ.
    │       Found type:    Bool
────╯
Type N1 := Nat.
Definition d : N1 :=
  c.
Definition d_0 : N1 :=
  c_0

```
