%{
%}

%token <string> IDENT

%token EQUAL_DEF ":="
       FN "fn"
       ARROW_FN "=>"
       LPAR "("
       RPAR ")"
       COMMA ","
       DOT "."
       TYPE "type"
       ARROW_TY "->"
       SEMI ":"
       (*PIPE "|"*)
       STAR "*"
       EOF

(* expression constant *)
%token E_UNIT
%token<bool> E_BOOL
%token<Z.t> E_INT
%token<char> E_CHAR
%token<string> E_STRING


%start program

%type <Ast.program> program

%%
(* helpers *)
let separated_list(sep, X) ==
| x = X; xl = preceded(sep, X)+;
    { x :: xl }

(* rules *)
let program :=
| defs = definition*; EOF;
    { defs }

let definition ==
| id = IDENT; ":="; e = expr; ".";
    { Ast.make_definition ~loc:$loc id e }

(* expression *)
let expr :=
| abstraction
| product_expr

let abstraction ==
| "fn"; p = pattern; "=>"; e = expr;
    { Ast.make_expr_abs ~loc:$loc p e }

let product_expr :=
| el = separated_nonempty_list(",", application);
    { match el with
      | [] -> assert false
      | [e] -> e
      | el -> Ast.make_expr_product ~loc:$loc el }

let application :=
| value
| e1 = application; e2 = value;
    { Ast.make_expr_app ~loc:$loc e1 e2 }


let value ==
| E_UNIT;
    { Ast.make_expr_unit ~loc:$loc () }
| x = IDENT;
    { Ast.make_expr_var ~loc:$loc x }
| b = E_BOOL;
    { Ast.make_expr_bool ~loc:$loc b }
| i = E_INT;
    { Ast.make_expr_int ~loc:$loc i }
| c = E_CHAR;
    { Ast.make_expr_char ~loc:$loc c }
| s = E_STRING;
    { Ast.make_expr_string ~loc:$loc s }
| t = ty_exp;
    { Ast.make_expr_type ~loc:$loc t }
| "("; e = expr; ")";
    { e }

let pattern ==
| x = IDENT;
    { Ast.make_patt_var ~loc:$loc x }
| "("; x = IDENT; ":"; ty = ty_exp; ")";
    { Ast.make_patt_var ~loc:$loc ~ty x }

(* Type *)
let ty_exp ==
| product_type
(*| union_type*)


let product_type ==
| el = separated_nonempty_list("*", arrow_type);
    { match el with
      | [] -> assert false
      | [t] -> t
      | tl -> Ast.make_type_product ~loc:$loc tl }

let arrow_type :=
| ty
| t1 = arrow_type; "->"; t2 = ty;
    { Ast.make_type_arrow ~loc:$loc t1 t2 }


(*let union_type :=*)
(*| label_type*)
(*| t1=type_exp; "|"; t2=type_exp*)
    (*{ Ast.make_type_union ~loc:$loc t1 t2}*)

(*let label_type :=*)
(*| l = IDENT; ":"; t = ty_exp;*)
    (*{ Ast.make_type_label ~loc:$loc l t }*)

let ty :=
| TYPE;
    { Ast.make_type_type ~loc:$loc () }
(*| "("; t = ty_exp; ")";*)
    (*{ t }*)
(*| e = term;*)
    (*{ Ast.make_type_expr ~loc:$loc e}*)

