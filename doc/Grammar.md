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
    "(" type ")"

expr :
    "(" expr ")"

ident :
    [a-z-A-Z_][a-z-A-Z_(unicode)]*
```
