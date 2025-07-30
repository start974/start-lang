# Grammar
- *<ANY>* is any character
- *<IDENT>* is define as [Unicode Standard Annex #31](https://www.unicode.org/reports/tr31/)

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
TYPE_OF_OP := "?:"
EQ_DEF := ":="
COLON := ":"
DOT := "."

```


## Parser
All element in quotes is keyword in "IDENTIFIER"

### Identifier
```ebnf
identifier := IDENTIFIER
```

### Expression

```ebnf
constant :=
| NUMBER
| CHARACTER

expression@0 :=
|  LPAR expression RPAR
|  identifier
|  constant

expression@1 :=
| expression@0 COLON type
| expression@0

expr_definition := identifier (COLON type)? EQ_DEF expression
```

### Type
```ebnf
type_definition := IDENTIFIER EQ_DEF type

type :=
| IDENTIFIER
```

## Command

```ebnf
command_kind :=
| ("Definition" | "Def") expr_definition
| ("Eval" | EVAL_OP) expr
| ("TypeOf" | TYPE_OF_OP) expr
| ("Type" | "Ty") type_definition
| ("Set") | "Unset") IDENTIFIER

command := command_kind DOT
```

## Program
A program is a succession of `command`
