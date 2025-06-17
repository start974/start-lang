# Grammar

## Identifier
```
identifier := "_"* letter  (letter | digit | _)* "'"*

letter := Alphabetic
digit := [0..9]
```

*Alphabetic* is described in Chapter 4 (Character Properties) of the
[Unicode Standard](https://www.unicode.org/versions/latest/) and
specified in the
[Unicode Character Database](https://www.unicode.org/reports/tr44/)
[`DerivedCoreProperties.txt`](https://www.unicode.org/Public/UCD/latest/ucd/DerivedCoreProperties.txt).

## Number
```
number := number_dec | number_hex | number_oct | number_bit

number_dec := digit ( digit | _)* digit

number_hex := "0" ("x" | "X") digit_hex (digit_hex | _)* digit_hex
digit_hex := digit | [A..F] | [a..f]

number_oct := "0" ("o" | "O") digit_oct (digit_oct | _)* digit_oct
digit_oct := [0..7]

number_bit := "0" ("b" | "B") digit_bit ( digit_bit | _)* digit_bit
digit_bit := [0..1]

```

After this section all element of grammar is padded

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
*WIP*

```
grammar :=
| "Add" ("Expression" | "Expr") grammar_syntax ":" grammar_rule ":=" expr
| "Add" ("Type" | "Ty") grammar_syntax ":" grammar_rule ":=" ty
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

## Program
```
prgm := (command ".")*
```

## Other
Parser is generated using [tree-sitter](https://tree-sitter.github.io/tree-sitter/)
in directory
[tree-sitter-start](../tree-sitter-start)
```


