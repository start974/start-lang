# Error

## Error on Parsing Definition

Error on identifier [file](/tests/parser/error.in/error_identifier.st)
```
$ startlang error_command.st
? 101
[101] Error: Parsing error
   ╭─[ error_command.st:1:1 ]
   │
 1 │ 3.
   │ ┬  
   │ ╰── Expected "command", found "3".
───╯


```
