# Error eval

```
$ startlang no_space.st
? 201
[201] Error: Parsing error
   ╭─[ no_space.st:1:5 ]
   │
 1 │ Eval1.
   │     ┬  
   │     ╰── Expected "whitespace", found "1".
───╯


```

## Unclosed char

```
$ startlang char_unclosed1.st
? 201
[201] Error: Parsing error
   ╭─[ char_unclosed1.st:1:8 ]
   │
 1 │ Eval '1.
   │        ┬  
   │        ╰── Expected "'", found ".".
───╯


```

```
$ startlang char_unclosed2.st
? 201
[201] Error: Parsing error
   ╭─[ char_unclosed2.st:1:7 ]
   │
 1 │ Eval 1'.
   │       ┬  
   │       ╰── Expected "something else".
───╯


```

## Unclosed parenthesis

```
$ startlang parent_unclosed1.st
? 201
[201] Error: Parsing error
   ╭─[ parent_unclosed1.st:1:8 ]
   │
 1 │ Eval (1.
   │        ┬  
   │        ╰── Expected "something else".
───╯


```


```
$ startlang parent_unclosed2.st
? 201
[201] Error: Parsing error
   ╭─[ parent_unclosed2.st:1:8 ]
   │
 1 │ Eval (1.
   │        ┬  
   │        ╰── Expected "something else".
───╯


```
