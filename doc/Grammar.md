# Grammar

- _<ANY>_ is any character
- _<IDENT>_ is define as [Unicode Standard Annex #31](https://www.unicode.org/reports/tr31/)

## Lexer

All token in lexer can be separated by space

### Comment

```ebnf
COMMENT := "(*" <ANY>* "*)"
```

### Identifier

```ebnf
INDENTIFIER := IDENT "'"*
```

### Number

```ebnf
NUMBER_F(DIGIT) := DIGIT ("_"* DIGIT)*
```

```ebnf
NUMBER_DEC := NUMBER_F DIGIT
DIGIT := [0..9]

NUMBER_HEX := "0" ("x" | "X") NUMBER_F(DIGIT_HEX)
DIGIT_HEX := DIGIT | [A..F] | [a..f]

NUMBER_OCT := "0" ("o" | "O") NUMBER_F(DIGIT_OCT)
DigitOct := [0..7]

NUMBER_BIT := "0" ("b" | "B") NUMBER_F(DIGIT_BIT)
DIGIT_BIT := [0..1]

NUMBER := NUMBER_DEC | NUMBER_HEX | NUMBER_OCT | NUMBER_BIT
```

### Character

```ebnf
CHARACTER := "'" CHARACTER_LIT "'"

CHARACTER_LIT := [U+0000 .. U+D7FF] | [U+E000 .. U+10FFFF] | ESCAPE_CHAR

ESCAPE_CHAR := "\"
    ("\\" | "\"" | "\'" | "n" | "r" | "t"
    | DIGIT{3} | "x" DIGITHEX{2} | "o" DIGITOCT{3}
    | "u{" DIGITHEX+ "}")
```

### Operator

```ebnf
EVAL_OP := "$"
HELP_OP := "?"
TYPE_OF_OP := "?:"

EQ_DEF := ":="
COLON := ":"
DOT := "."
L_PAREN := "("
R_PAREN := ")"

```

## Parser

All element in quotes is keyword in "IDENTIFIER"

### Operator

```
eq_def := EQ_DEF            display as operator
dot := DOT                  display as operator
colon := COLON              display as operator
l_paren := L_PAREN          display as operator
r_paren := L_PAREN          display as operator
```

### Constant

```
constant :=
| NUMBER                    display as number
| CHARACTER                 display as character
```

### Pattern

```ebnf
pattern :=
| IDENTIFIER                display as def_var
```

### Type

```ebnf
type_var := IDENTIFIER      display as ty_var

type :=
| type_var
```

### Type Definition

```ebnf
type_definition := type_var EQ_DEF type
```

### Expression

```ebnf
expr_var := IDENTIFIER      display as expr_var

expression@0 :=
|  l_paren expression r_paren
|  expr_var
|  constant

expression@1 :=
| expression@0 colon type
| expression@0

expression = expression@1
```

### Expression Definition

```ebnf
expr_definition := pattern (colon type)? eq_def expression
```

## Command

```ebnf
keyword_definition :=
| "Definition"          display as keyword
| "Def"                 display as keyword
```

```ebnf
keyword_type :=
| "Type"                 display as keyword
| "Ty"                   display as keyword
```

```ebnf
keyword_eval :=
| "Eval"                display as keyword
| EVAL_OP               display as keyword
```

```ebnf
keyword_typeof :=
| "TypeOf"               display as keyword
|  TYPE_OF_OP            display as keyword
```

```ebnf
keyword_help :=
| "Help"                 display as keyword
|  HELP_OP               display as keyword
```

```ebnf
keyword_set :=
| "Set"                  display as keyword
```

```ebnf
keyword_unset :=
| "Unset"                display as keyword
```

```ebnf
dot := DOT               display as operator
```

```ebnf
command_kind :=
| keyword_definition expr_definition
| keyword_type type_definition
| keyword_help IDENTIFIER
| keyword_eval expr
| keyword_typeof expr
| keyword_set variable
| keyword_unset variable

command := command_kind DOT
```

## Program

A program is a succession of `command`
