# Grammar

## Identifier
```
identifier := (letter | _) (letter | 0..9 | _ | ')*

letter := uppercase-letter | lowercase-letter | unicode-letter
```

## Number
```
number :=
| [0..9] ( [0..9] | _)*
| ("0x" | "0X") ([0..9] | [A..F] | [a..f]) { [0..9] | [A-F] | [a..f] | _}
| ("0o" | "0O") ([0..7] ) { [0..7] | _}
| ("0b" | "0B") ([0..1] ) { [0..1] | _}
```

## Command
```
command :=
| ("Definition" | "Def") expr_definition
| ("Type" | "Ty") type_definition
| ("Eval" | "$") expr
| ("Grammar" | "Gram") grammar
| ("TypeOf" | "?:") expr

```

### Expression
```
expr :=
  "(" expr ")"
  expr type_rest
  identifier
  constant

type_restr := ":" type
```

### Type
```
type_definition := identifier ":=" type

type :=
| identifier
```

## Grammar
```
grammar :=
| "Add" grammar_syntax ":" grammar_rule ":=" template
| "New" identifier
| ("Remove" | "Rm") identifier

grammar_rule := identifier "@" number

grammar_syntax :=
| identifier
| "<" identifier ":" grammar_syntax ">"
| "[" group_char ".." group_char "]"
| grammar_syntax "?"
| grammar_syntax "*"
| grammar_syntax "+"
| "~" grammar_syntax
| "(" grammar_syntax ")"
| grammar_syntax "|" grammar_syntax


group_char := [0..9] | [a..z] | [A..Z]
```

## Template
```
template := {template_part}

template_part :=
| template_text
| template_var

template_text := (~template_escape | template_escape)+
template_escape := '\' ('\' | '{' | '}')

template_var :=
| '{' identifier '}'

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


