# Error on definition

```
$ startlang def_not_space.st
? 201
[201] Error: Parsing error
   ╭─[ def_not_space.st:1:4 ]
   │
 1 │ Defa := 1.
   │    ┬  
   │    ╰── Expected "''i''", "whitespace", found "a".
───╯


```

```
$ startlang no_eq_def.st
? 201
[201] Error: Parsing error
   ╭─[ no_eq_def.st:1:14 ]
   │
 1 │ Definition a 1.
   │              ┬  
   │              ╰── Expected "type_restriction", ":=", found "1".
───╯


```

```
$ startlang no_expression.st
? 201
[201] Error: Parsing error
   ╭─[ no_expression.st:1:17 ]
   │
 1 │ Definition a :=
   │                 │ 
   │                 ╰─ Expected "expression".
───╯


```

```
$ startlang stop_ident.st
? 201
[201] Error: Parsing error
   ╭─[ stop_ident.st:1:14 ]
   │
 1 │ Definition a
   │              │ 
   │              ╰─ Expected "type_restriction", ":=".
───╯


```

```
$ startlang wrong_name.st
? 201
[201] Error: Parsing error
   ╭─[ wrong_name.st:1:12 ]
   │
 1 │ Definition 1 := 3.
   │            ┬  
   │            ╰── Expected "whitespace", "identifier", found "1".
───╯


```

```
$ startlang wrong_ty.st
? 201
[201] Error: Parsing error
   ╭─[ wrong_ty.st:1:16 ]
   │
 1 │ Definition a : 1 := 1.
   │                ┬  
   │                ╰── Expected "type", found "1".
───╯


```
