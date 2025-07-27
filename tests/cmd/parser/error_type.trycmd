# Error Type

```
$ startlang run no_eq_def.st
? 201
[201] Error: Parsing error
   ╭─[ no_eq_def.st:1:8 ]
   │
 1 │ Type N Nat.
   │        ┬  
   │        ╰── Expected ":=", found "N".
───╯


```

```
$ startlang run no_space.st
? 201
[201] Error: Parsing error
   ╭─[ no_space.st:1:5 ]
   │
 1 │ TypeN := Nat.
   │     ┬  
   │     ╰── Expected "whitespace", "''O''", found "N".
───╯


```

```
$ startlang run not_end.st
? 201
[201] Error: Parsing error
   ╭─[ not_end.st:1:8 ]
   │
 1 │ Type N
   │        │ 
   │        ╰─ Expected ":=".
───╯


```
```
$ startlang run wrong_name.st
? 201
[201] Error: Parsing error
   ╭─[ wrong_name.st:1:6 ]
   │
 1 │ Type 1 := ℕ.
   │      ┬  
   │      ╰── Expected "whitespace", "identifier", found "1".
───╯


```
