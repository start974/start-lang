# Grammar

- *IDENT* is define as [Unicode Standard Annex #31](https://www.unicode.org/reports/tr31/)
- *ANY* is any character
- *SPACE* is all whitespace

## Identifier
```
identifier := IDENT "'"*
```

## Number
```
number := number_dec | number_hex | number_oct | number_bit

number_dec := digit ( digit | _)* digit
digit := [0..9]

number_hex := "0" ("x" | "X") digit_hex (digit_hex | _)* digit_hex
digit_hex := digit | [A..F] | [a..f]

number_oct := "0" ("o" | "O") digit_oct (digit_oct | _)* digit_oct
digit_oct := [0..7]

number_bit := "0" ("b" | "B") digit_bit ( digit_bit | _)* digit_bit
digit_bit := [0..1]

```

## Character
```
character := "'" char "'"

character_literal := [U+0000 .. U+D7FF] | [U+E000 .. U+10FFFF] | escape_char

escape_char := "\"
    ("\\" | "\"" | "\'" | "n" | "r" | "t"
    | digit{3} | "x" digit_hex{2} | "o" digit_oct{3}
    | "u{" digit_hex+ "}")
```

After this section all element of grammar is padded

### Expression
```
type_restriction := ":" type

constant :=
| number
| character

expression :=
|  "(" expression ")"
|  identifier
|  constant
|  expression type_restiction

expr_definition := identifier type_rest? ":=" expression
```

### Type
```
type_definition := identifier ":=" type

type :=
| identifier
```

## Grammar
*WIP* Not implement yet (need to implement function and library on parser ast)

```
grammar :=
| "Add" SPACE grammar_syntax ":" grammar_rule ":=" expression
| "New" SPACE identifier
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

## Command
```
command_dot := command "."?

command :=
| ("Definition" | "Def") SPACE expr_definition
| ("Type" | "Ty") SPACE type_definition
| ("Eval" | "$") SPACE expr
| ("Grammar" | "Gram") SPACE grammar              // Not yet implemented
| ("TypeOf" | "?:") SPACE expr

```

## Program
```
prgm := command_dot*
```

## Other
Parser is generated using [tree-sitter](https://tree-sitter.github.io/tree-sitter/)
in directory
[tree-sitter-start](../tree-sitter-start)
```


