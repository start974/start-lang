# Grammar

## Identifier
```
ident := (letter | _) {letter | 0..9 | _ | '}

letter := uppercase-letter | lowercase-letter | unicode-letter
```

## Number
```
number :=
| (0..9){ 0..9 | _}*
| (0x | 0X) (0..9 | A..F | a..f) { 0..9 | A-F | a..f | _}
| (0o | 0O) (0..7 ) { 0..7 | _}
| (0b | 0B) (0..1 ) { 0..1 | _}
```

## Command
```
command :=
| ("Definition" | "Def") definition
| ("Grammar" | "Gram") grammar
| ("TypeOf" | "?:") expr
| ("Eval" | "$") expr

```

## Definition
```
definition :=
  ident [type_restr] ":=" expr
  "Type" ident ":=" type
```

### Expression
```
expr :=
  "(" expr ")"
  expr type_rest
  ident
  constant

type_restr := ":" type
```

### Type
```
type_def := ident ":=" type
```

## Grammar
```
grammar :=
| "Add" grammar_syntax "as" ident "in" ident ["with" grammar_with] ":=" expr
| "New" ident
| ("Remove" | "Rm") ident

grammar_syntax :=
| ident
| "<" (ident ":" )? grammar_syntax ">"
| "..."
| "[" grammar_syntax "]"
| "{" grammar_syntax "}"
| "(" grammar_syntax ")"
| grammar_syntax "|" grammar_syntax
```

## Program
```
prgm := (command ".")*
```

## Other
Parser is generated using [tree-sitter](https://tree-sitter.github.io/tree-sitter/)
in directory
[tree-sitter-start](../tree-sitter-start)
```


