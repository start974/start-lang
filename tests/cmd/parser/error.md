# Error

## Error on command
```
$ startlang command.st
? 101
[101] Error: Parsing error
   ╭─[ command.st:1:1 ]
   │
 1 │ 3.
   │ ┬  
   │ ╰── Expected "command", found "3".
───╯


```

```
$ startlang last_command.st
? 101
[101] Error: Parsing error
   ╭─[ last_command.st:5:1 ]
   │
 5 │ a.
   │ ┬  
   │ ╰── Expected "command", found "a".
───╯


```
## Error Expression Definition

```
$ startlang definition_name.st
? 101
[101] Error: Parsing error
   ╭─[ definition_name.st:1:12 ]
   │
 1 │ Definition 1 := 3.
   │            ┬  
   │            ╰── Expected "identifier", found "1".
───╯


```


```
$ startlang definition_not_end.st
? 101
[101] Error: Parsing error
   ╭─[ definition_not_end.st:1:14 ]
   │
 1 │ Definition a
   │              │ 
   │              ╰─ Expected "type_restriction", ":=".
───╯


```

```
$ startlang definition_expr_ty.st
? 101
[101] Error: Parsing error
   ╭─[ definition_expr_ty.st:1:16 ]
   │
 1 │ Definition a : 1 := 1.
   │                ┬  
   │                ╰── Expected "type", found "1".
───╯


```

## Error Type Definition

```
$ startlang definition_no_expression.st
? 101
[101] Error: Parsing error
   ╭─[ definition_no_expression.st:1:17 ]
   │
 1 │ Definition a :=
   │                 │ 
   │                 ╰─ Expected "expression".
───╯


```

```
$ startlang definition_ty.st
? 101
[101] Error: Parsing error
   ╭─[ definition_ty.st:1:6 ]
   │
 1 │ Type 1 := N.
   │      ┬  
   │      ╰── Expected "identifier", found "1".
───╯


```
