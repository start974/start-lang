# Grammar

Parser is generated using [tree-sitter](https://tree-sitter.github.io/tree-sitter/)
in directory
[tree-sitter-start](../tree-sitter-start)
```
prgm :
  definition*

definition :
  expr_def

expr_def :
  "def" ident (type_restr)? := expr

type_restr :
  ":" type

type :
  ident

expr_final :
    expr "."?
expr :
  "(" expr ")"
  constant

constant:
  number_n

number_n :
| [0-9][0-9_]*
| 0[xX][0-9A-Fa-f][0-9A-Fa-f_]*
| 0[oO][0-7][0-7_]*
| 0[oB][0-1][0-1_]*


ident :
  [a-z-A-Z(unicode)]+
```
