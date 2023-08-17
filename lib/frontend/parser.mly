%{
    open Ast
%}

%token <string> IDENT

%token EQUAL_DEF ":="
       FN "fn"
       ARROW_FN "=>"
       LPAR "("
       RPAR ")"
       DOT "."
       EOF

%start program

%type <Program.t> program

%%

let program :=
| defs = definition*; EOF;
    { defs }

let definition :=
| id = IDENT; ":="; e = expr; ".";
    { Definition.make ~loc:($loc) id e }

let application :=
| term
| e1 = application; e2 = term;
    { Expr.make_app ~loc:($loc) e1 e2 }

let abstraction ==
| "fn"; p = pattern; "=>"; e = expr;
    { Expr.make_abs ~loc:($loc) p e }

let expr :=
| application
| abstraction

let term :=
| x = IDENT;
    { Expr.make_var ~loc:($loc) x }
| "("; e = expr; ")";
    { e }

let pattern :=
| x = IDENT;
    { Pattern.make_var ~loc:($loc) x }

