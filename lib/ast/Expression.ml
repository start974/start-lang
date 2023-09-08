open InputUtils
open Typing
module Error = Error.Ast.Expression

type expr = E_Const of Constant.t | E_Type of Type.t | E_Var of Ident.t
and t = { loc : Location.t; expr : expr; ty : Type.t }

let e_const ?(loc = Location.unknown) c =
  let ty = Constant.ty c in
  let expr = E_Const c in
  { loc; expr; ty }

let e_type ?(loc = Location.unknown) t =
  let ty = Type.ty t in
  let expr = E_Type t in
  { loc; expr; ty }

type res_var = t Error.ErrorVar.res
type res = res_var

let e_var ?(loc = Location.unknown) ~ty_env x =
  match Env.find_opt x ty_env with
  | None -> Error.error_unknown_var loc x
  | Some ty ->
      let expr = if Type.t_type = ty then E_Type (Type.t_var x) else E_Var x in
      Result.ok { loc; expr; ty }

let from_parse_tree_expr_val ~ty_env ~loc =
  let open Frontend.ParseTree in
  function
  | E_Type ->
      let e = e_type ~loc Type.t_type in
      Result.ok e
  | E_Var v -> e_var ~loc ~ty_env v
  | (E_Unit | E_Bool _ | E_Int _ | E_Char _ | E_String _) as c ->
      let c' = Constant.from_parse_tree c in
      let e = e_const ~loc c' in
      Result.ok e

let from_parse_tree ~ty_env
    ({ location = loc; node = expr } : Frontend.ParseTree.expr_loc) =
  let open Frontend.ParseTree in
  match expr with
  | E_Value v -> from_parse_tree_expr_val ~loc ~ty_env v
  | _ -> failwith "WIP"

let ty { ty; _ } = ty
let expr { expr; _ } = expr
let location { loc; _ } = loc

let pp_print fmt ({ expr; _ } : t) =
  match expr with
  | E_Const c -> Constant.pp_print fmt c
  | E_Type t -> Type.pp_print fmt t
  | E_Var x -> Ident.pp_print fmt x

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
