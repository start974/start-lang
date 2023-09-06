open InputUtils

(* identifier *)
type ident = string

(* cons expr and type *)
type expr_value =
  | E_Type
  | E_Var of ident
  | E_Unit
  | E_Bool of bool
  | E_Int of Z.t
  | E_Char of char
  | E_String of string

(* expression *)
type expr =
  | E_Value of expr_value
  | E_App of expr_loc * expr_loc
  | E_Abs of patt_loc * expr_loc
  | E_Prod of expr_loc list
  | E_Ty_Arrow of expr_loc * expr_loc

and expr_loc = expr Location.node_location

(* pattern *)
and patt_arg =
  | P_Unit
  | P_Wildcard
  | P_Var of ident
  | P_Prod of patt_arg_loc list

and patt_arg_loc = patt_arg Location.node_location
and patt_arg_typed = { patt_arg : patt_arg_loc; patt_ty : expr_loc option }
and patt_arg_typed_loc = patt_arg_typed Location.node_location
and patt = { patt_args : patt_arg_typed_loc list; patt_ret : expr_loc option }
and patt_loc = patt Location.node_location

(* definition *)
type def = { name : ident; patt : patt_loc option; body : expr_loc }
type def_loc = def Location.node_location

(* program *)
type program = def_loc list

(* builders *)
(* - expression *)
let make_expr_value ?loc c = Location.make_node_loc ?loc (E_Value c)
let make_expr_type ?loc () = make_expr_value ?loc E_Type
let make_expr_var ?loc x = make_expr_value ?loc (E_Var x)
let make_expr_unit ?loc () = make_expr_value ?loc E_Unit
let make_expr_bool ?loc b = make_expr_value ?loc (E_Bool b)
let make_expr_int ?loc i = make_expr_value ?loc (E_Int i)
let make_expr_char ?loc c = make_expr_value ?loc (E_Char c)
let make_expr_string ?loc s = make_expr_value ?loc (E_String s)
let make_expr_app ?loc e1 e2 = Location.make_node_loc ?loc (E_App (e1, e2))
let make_expr_abs ?loc p e = Location.make_node_loc ?loc (E_Abs (p, e))

let make_expr_product ?loc el =
  assert (List.length el >= 2);
  Location.make_node_loc ?loc (E_Prod el)

let make_expr_arrow_ty ?loc e1 e2 =
  Location.make_node_loc ?loc (E_Ty_Arrow (e1, e2))

(* - pattern *)
let make_patt_arg ?loc arg = Location.make_node_loc ?loc arg
let make_patt_arg_var ?loc x = make_patt_arg ?loc (P_Var x)
let make_patt_arg_wildcard ?loc () = make_patt_arg ?loc P_Wildcard
let make_patt_arg_unit ?loc () = make_patt_arg ?loc P_Unit
let make_patt_arg_prod ?loc pl = make_patt_arg ?loc (P_Prod pl)

let make_patt_arg_typed ?loc patt_arg patt_ty =
  Location.make_node_loc ?loc { patt_arg; patt_ty }

let make_patt ?loc patt_args patt_ret =
  Location.make_node_loc ?loc { patt_args; patt_ret }

(* - definition *)
let make_definition ?loc name patt body =
  Location.make_node_loc ?loc { name; patt; body }

(* pretty printing *)
let pp_expr_value fmt = function
  | E_Type -> Format.pp_print_string fmt "type"
  | E_Var x -> Format.pp_print_string fmt x
  | E_Unit -> Format.pp_print_string fmt "()"
  | E_Bool b -> Utils.Pretty.pp_const_bool fmt b
  | E_Int i -> Z.pp_print fmt i
  | E_Char c -> Utils.Pretty.pp_const_char fmt c
  | E_String s -> Utils.Pretty.pp_const_string fmt s

let rec pp_expr fmt (e : expr_loc) =
  match e.node with
  | E_Abs (pat, e) -> Format.fprintf fmt "@[λ %a => %a@]" pp_patt pat pp_expr e
  | E_Prod el ->
      let pp_content fmt el =
        let pp_sep fmt () = Format.fprintf fmt ",@ " in
        Format.pp_print_list ~pp_sep pp_application fmt el
      in
      Format.fprintf fmt "@[%a@]" pp_content el
  | _ -> pp_arrow_ty fmt e

and pp_arrow_ty fmt (e : expr_loc) =
  match e.node with
  | E_Ty_Arrow (e1, e2) ->
      Format.fprintf fmt "@[@[%a@] -> @[%a@]@]" pp_arrow_ty e1 pp_application e2
  | _ -> pp_application fmt e

and pp_application fmt (e : expr_loc) =
  match e.node with
  | E_App (el, er) ->
      Format.fprintf fmt "@[%a %a@]" pp_application el pp_value er
  | _ -> pp_value fmt e

and pp_value fmt (e : expr_loc) =
  match e.node with
  | E_Value e -> pp_expr_value fmt e
  | _ -> Format.fprintf fmt "(@[%a@])" pp_expr e

and pp_patt_arg fmt (p : patt_arg_loc) =
  match p.node with
  | P_Prod xl ->
      let pp_content fmt xl =
        let pp_sep fmt () = Format.fprintf fmt ",@ " in
        Format.pp_print_list ~pp_sep pp_patt_value fmt xl
      in
      Format.fprintf fmt "(@[%a@])" pp_content xl
  | _ -> pp_patt_value fmt p

and pp_patt_value fmt (p : patt_arg_loc) =
  match p.node with
  | P_Var x -> Format.pp_print_string fmt x
  | P_Unit -> Format.pp_print_string fmt "()"
  | P_Wildcard -> Format.pp_print_string fmt "_"
  | _ -> pp_patt_arg fmt p

and pp_patt_arg_typed fmt (p : patt_arg_typed_loc) =
  let { patt_arg; patt_ty } = p.node in
  match patt_ty with
  | None -> pp_patt_arg fmt patt_arg
  | Some t ->
      Format.fprintf fmt "(@[%a@ :@ %a@])" pp_patt_arg patt_arg pp_expr t

and pp_patt fmt (p : patt_loc) =
  let { patt_args; patt_ret } = p.node in
  let pp_sep fmt () = Format.fprintf fmt "@ " in
  Format.pp_print_list ~pp_sep pp_patt_arg_typed fmt patt_args;
  Option.iter (fun ty -> Format.fprintf fmt " : %a" pp_expr ty) patt_ret

let pp_definition fmt (d : def_loc) =
  let { name; patt; body } = d.node in
  match patt with
  | Some patt ->
      Format.fprintf fmt "@[%s@ @[%a@] := @[%a@].@]" name pp_patt patt pp_expr
        body
  | None -> Format.fprintf fmt "@[%s@ := @[%a@].@]" name pp_expr body

let pp_program =
  Format.pp_print_list
    ~pp_sep:(fun fmt () -> Format.fprintf fmt "@.")
    pp_definition
