open InputUtils
module PTree = Frontend.ParseTree

type expr = E_Const of Constant.t | E_Type of Type.t
and t = { loc : Location.t; expr : expr; ty : Type.t }

let make_ty = function E_Const c -> Constant.ty c | E_Type t -> Type.ty t

let make_expr ~loc expr =
  let ty = make_ty expr in
  { loc; expr; ty }

let e_const ?(loc = Location.unknown) c = make_expr ~loc (E_Const c)
let e_type ?(loc = Location.unknown) t = make_expr ~loc (E_Type t)

let from_parse_tree_expr_val ~loc =
  let open Frontend.ParseTree in
  function
  | E_Type -> e_type ~loc Type.t_type
  | E_Var _v -> failwith "WIP"
  | E_Unit -> e_const ~loc (Constant.c_unit ~loc ())
  | E_Bool b -> e_const ~loc (Constant.c_bool ~loc b)
  | E_Int i -> e_const ~loc (Constant.c_int ~loc i)
  | E_Char c -> e_const ~loc (Constant.c_char ~loc c)
  | E_String s -> e_const ~loc (Constant.c_string ~loc s)

let from_parse_tree
    ({ location = loc; node = expr } : Frontend.ParseTree.expr_loc) =
  let open Frontend.ParseTree in
  match expr with
  | E_Value v -> from_parse_tree_expr_val ~loc v
  | _ -> failwith "WIP"

let ty { ty; _ } = ty
let expr { expr; _ } = expr
let location { loc; _ } = loc

let pp_print fmt ({ expr; _ } : t) =
  match expr with
  | E_Const c -> Constant.pp_print fmt c
  | E_Type t -> Type.pp_print fmt t

(*let loc_unknown = Location.unknown*)

(*let make_value ~loc v =*)
(*make_expr ~loc ~ty_env:Env.empty (E_Value v)*)

(*let e_var ?(loc=loc_unknown) ~ty_env x =*)
(*make_expr ~loc ~ty_env (E_Var x)*)

(*let e_app ?(loc=loc_unknown) ~ty_env e1 e2 =*)
(*make_expr ~loc ~ty_env (E_App (e1, e2))*)

(*let e_abs ?(loc=loc_unknown) ~ty_env p e =*)
(*make_expr ~loc ~ty_env (E_Abs (p, e))*)

(*let e_prod ?(loc=loc_unknown) ~ty_env el =*)
(*make_expr ~loc ~ty_env (E_Prod el)*)

(*let e_type ?(loc=loc_unknown) ~ty_env t*)
(*make_expr ~loc ~ty_env (E_Type el)*)

(*let from_parse_tree_expr_val ~loc = PTree.(function*)
(*| E_Type -> e_var ~loc "type"*)
(*| E_Var v -> e_var ~loc v*)
(*| E_Unit -> e_unit ~loc ()*)
(*| E_Bool b -> e_bool ~loc b*)
(*| E_Int i -> e_int ~loc i*)
(*| E_Char c -> e_char ~loc c*)
(*| E_String s -> e_string ~loc s*)
(**)
(*let rec from_parse_tree ({location=loc; node}: Ptree.expr_loc) =*)
(* PTree.( *)
(*match node with*)
(*| E_Value v -> from_parse_tree_expr_val ~loc v*)
(*| E_App (e1, e2) ->*)
(*let e1' = from_parse_tree e1*)
(*and e2' = from_parse_tree e2*)
(*e_app ~loc e1' e2'*)
(*| E_Abs (p, e) ->*)
(*let p' = Pattern.from_parse_tree p in*)
(*and e' = from_parse_tree e in*)
(*e_abs p' e'*)
(*| E_Prod el ->*)
(*let el' = List.map from_parse_tree el in*)
(*e_prod*)

(**)

(*let pp_print ?(typed=false) fmt e =failwith "WIP"*)
