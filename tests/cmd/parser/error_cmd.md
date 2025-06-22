# Error Command

## File not found

```
$ startlang file_not_exist.rs
? 101
[101] Error: Cannot read file "file_not_exist.rs".

```

```
$ startlang file1 file2
? 1
Usage: startlang [file.st]

```

## Option not found

```
$ startlang option.st
? 103
[103] Error: Option "UknownOption" is unknown.

```

## Error on command unknown
```
$ startlang command.st
? 201
[201] Error: Parsing error
   ╭─[ command.st:1:1 ]
   │
 1 │ 3.
   │ ┬  
   │ ╰── Expected "command", found "3".
───╯


```

```
$ startlang last_command.st
? 201
[201] Error: Parsing error
   ╭─[ last_command.st:5:1 ]
   │
 5 │ a.
   │ ┬  
   │ ╰── Expected "command", found "a".
───╯


```
