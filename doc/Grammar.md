# Grammar
- *<ANY>* is any character
- *<IDENT>* is define as [Unicode Standard Annex #31](https://www.unicode.org/reports/tr31/)

## Lexer
All token in lexer can be separated by space
### Comment
```
COMMENT := "(*" <ANY>* "*)"
```

### Identifier

```
INDENTIFIER := IDENT "'"*
```

### Number
```
NUMBER_F(DIGIT) := DIGIT ("_"* DIGIT)*
```

```
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
```
CHARACTER := "'" CHARACTER_LIT "'"

CHARACTER_LIT := [U+0000 .. U+D7FF] | [U+E000 .. U+10FFFF] | ESCAPE_CHAR

ESCAPE_CHAR := "\"
    ("\\" | "\"" | "\'" | "n" | "r" | "t"
    | DIGIT{3} | "x" DIGITHEX{2} | "o" DIGITOCT{3}
    | "u{" DIGITHEX+ "}")
```

### Operator
```
EQ_DEF := ":="
COLON := ":"
DOT := "."
```

### keyword
```
DEFINITION := "Definition" | "Def"
EVAL := "Eval" | "$"
TYPE := "Type" | "Ty"
TYPE_OF := "TypeOf" | "?:"
```
## Parser

### Identifier
```
identifier := IDENTIFIER
```

### Expression
```
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
```
type_definition := IDENTIFIER ":=" type

type :=
| IDENTIFIER
```

## Command
```
command_kind :=
| DEFINITY expr_definition
| EVAL expr
| TYPE_OF expr
| TY type_definition
| SET IDENTIFIER
| UNSET IDENTIFIER

command := command_kind DOT
```

## Program
A program is a succession of `command`
