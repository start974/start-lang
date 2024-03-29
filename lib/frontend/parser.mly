%{
%}

%token <string> IDENT

%token EQUAL_DEF ":="
%token FN "fn"
%token ARROW_FN "=>"
%token LPAR "("
%token RPAR ")"
%token COMMA ","
%token DOT "."
%token TYPE "type"
%token ARROW_TY "->"
%token SEMI ":"
%token UNDERSCORE "_"
%token UNIT "()"
(*%token PIPE "|"*)
(*%token STAR "*"*)
%token EOF

(* expression constant *)
%token<bool> E_BOOL
%token<Z.t> E_INT
%token<char> E_CHAR
%token<string> E_STRING


%start program

%type <ParseTree.program> program

%%
(* helpers *)
let rev_separated_list(sep, X) :=
| xl = rev_separated_list(sep, X); sep; x = X;
    { x :: xl }
| x1 = X; sep; x2 = X;
    { [ x2; x1 ] }

let separated_list(sep, X) ==
| l = rev_separated_list(sep, X);
    { List.rev l }

(* rules *)
let program :=
| defs = definition*; EOF;
    { defs }

(* definition *)
let definition :=
| name = IDENT; p = pattern?; ":="; body = expr; ".";
    { ParseTree.make_definition ~loc:$loc name p body }


(* expression *)
let expr :=
| abstraction
| product_expr

let abstraction :=
| "fn"; p = pattern; "=>"; e = expr;
    { ParseTree.make_expr_abs ~loc:$loc p e }

let product_expr :=
| arrow_type
| el = separated_list(",", arrow_type);
    { ParseTree.make_expr_product ~loc:$loc el }

let arrow_type :=
| application_expr
| t1 = arrow_type; "->"; t2 = application_expr;
    { ParseTree.make_expr_arrow_ty ~loc:$loc t1 t2 }

let application_expr :=
| value
| e1 = application_expr; e2 = value;
    { ParseTree.make_expr_app ~loc:$loc e1 e2 }

let var ==
| UNDERSCORE;
    { ParseTree.make_expr_var ~loc:$loc "_" }
| x = IDENT;
    { ParseTree.make_expr_var ~loc:$loc x }

let value :=
| var
| "type";
    { ParseTree.make_expr_type ~loc:$loc () }
| "()";
    { ParseTree.make_expr_unit ~loc:$loc () }
| b = E_BOOL;
    { ParseTree.make_expr_bool ~loc:$loc b }
| i = E_INT;
    { ParseTree.make_expr_int ~loc:$loc i }
| c = E_CHAR;
    { ParseTree.make_expr_char ~loc:$loc c }
| s = E_STRING;
    { ParseTree.make_expr_string ~loc:$loc s }
| "("; e = expr; ")";
    { e }

(* pattern *)
let pattern :=
| args = pattern_arg_typed+; ret = pattern_type?;
    { ParseTree.make_patt ~loc:$loc args ret }

let pattern_type :=
| ":"; ty = expr;
    { ty }

let pattern_arg_typed :=
| p = pattern_arg;
    { ParseTree.make_patt_arg_typed ~loc:$loc p None }
| "("; p = pattern_arg ; ty = pattern_type?; ")";
    { ParseTree.make_patt_arg_typed ~loc:$loc p ty }

let pattern_arg ==
| pattern_prod

let pattern_prod :=
| pattern_value
| "("; args = separated_list(",", pattern_arg); ")";
    { ParseTree.make_patt_arg_prod ~loc:$loc args }

let pattern_value :=
| v = IDENT;
    { ParseTree.make_patt_arg_var ~loc:$loc v }
| "()";
    { ParseTree.make_patt_arg_unit ~loc:$loc () }
| "_";
    { ParseTree.make_patt_arg_wildcard ~loc:$loc () }
