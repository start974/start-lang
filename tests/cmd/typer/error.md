# Error

## Type not found
```
$ startlang type_not_found.st
? 45
[301] Error: Variable not found.
   ╭─[ type_not_found.st:1:9 ]
   │
 1 │ Def a : Not_definied := 4.
   │         ──────┬─────  
   │               ╰─────── Variable "Not_definied" not found in the current scope.
───╯

```

## variable not found

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
