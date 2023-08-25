open InputUtils

(* identifier *)
type ident = string

(* cons expr and type *)
type expr_value =
  | E_Var of ident
  | E_Unit
  | E_Bool of bool
  | E_Int of Z.t
  | E_Char of char
  | E_String of string

(* expression *)
type expr =
  | E_Cons of expr_value
  | E_App of expr_loc * expr_loc
  | E_Abs of patt_loc * expr_loc
  | E_Prod of expr_loc list
  | E_Type of ty_loc

and expr_loc = expr Location.node_location

(* pattern *)
and patt = P_Var of ident * ty_loc option
and patt_loc = patt Location.node_location

(* type *)
and ty =
  | T_Type
  | T_Expr of expr_loc
  | T_Arrow of ty_loc * ty_loc
  | T_Prod of ty_loc list

(*| T_Union of ty_loc * ty_loc*)
and ty_loc = ty Location.node_location

(* definition *)
type definition = ident * expr_loc
type definition_pos = definition Location.node_location

(* program *)
type program = definition_pos list

(* builders *)
(* - expression *)
let make_expr_cons ?loc c = Location.make_node_loc ?loc (E_Cons c)
let make_expr_var ?loc x = make_expr_cons ?loc (E_Var x)
let make_expr_unit ?loc () = make_expr_cons ?loc E_Unit
let make_expr_bool ?loc b = make_expr_cons ?loc (E_Bool b)
let make_expr_int ?loc i = make_expr_cons ?loc (E_Int i)
let make_expr_char ?loc c = make_expr_cons ?loc (E_Char c)
let make_expr_string ?loc s = make_expr_cons ?loc (E_String s)
let make_expr_app ?loc e1 e2 = Location.make_node_loc ?loc (E_App (e1, e2))
let make_expr_abs ?loc p e = Location.make_node_loc ?loc (E_Abs (p, e))

let make_expr_product ?loc el =
  assert (List.length el >= 2);
  Location.make_node_loc ?loc (E_Prod el)

let make_expr_type ?loc t = Location.make_node_loc ?loc (E_Type t)

(* - pattern *)
let make_patt_var ?loc ?ty x = Location.make_node_loc ?loc (P_Var (x, ty))

(* - type *)
let make_type_type ?loc () = Location.make_node_loc ?loc T_Type
let make_type_expr ?loc e = Location.make_node_loc ?loc (T_Expr e)
let make_type_arrow ?loc t1 t2 = Location.make_node_loc ?loc (T_Arrow (t1, t2))

let make_type_product ?loc tl =
  assert (List.length tl >= 2);
  Location.make_node_loc ?loc (T_Prod tl)

(* - definition *)
let make_definition ?loc id e = Location.make_node_loc ?loc (id, e)

(* pretty printing *)
let pp_expr_value fmt = function
  | E_Var x -> Format.pp_print_string fmt x
  | E_Unit -> Format.pp_print_string fmt "()"
  | E_Bool true -> Format.pp_print_string fmt "⊤"
  | E_Bool false -> Format.pp_print_string fmt "⊥"
  | E_Int i -> Z.pp_print fmt i
  | E_Char c ->
      let s = if c = '\'' then "\\'" else Char.escaped c in
      Format.fprintf fmt "'%s'" s
  | E_String s ->
      let buffer = Buffer.create 17 in
      let update_buffer first s =
        let es = String.escaped s in
        if not first then Buffer.add_string buffer "\\\"";
        Buffer.add_string buffer es;
        false
      in
      ignore @@ List.fold_left update_buffer true (String.split_on_char '"' s);
      Format.fprintf fmt "\"%s\"" (Buffer.contents buffer)

let rec pp_expr fmt (e : expr_loc) =
  match e.node with
  | E_Abs (pat, e) ->
      Format.fprintf fmt "@[λ %a => %a@]" pp_pattern pat pp_expr e
  | E_Prod el ->
      let pp_content fmt el =
        let pp_sep fmt () = Format.fprintf fmt ",@ " in
        Format.pp_print_list ~pp_sep pp_application fmt el
      in
      Format.fprintf fmt "@[%a@]" pp_content el
  | _ -> pp_application fmt e

and pp_application fmt (e : expr_loc) =
  match e.node with
  | E_App (el, er) ->
      Format.fprintf fmt "@[%a %a@]" pp_application el pp_value er
  | _ -> pp_value fmt e

and pp_value fmt (e : expr_loc) =
  match e.node with
  | E_Cons e -> pp_expr_value fmt e
  | E_Type t -> pp_ty fmt t
  | _ -> Format.fprintf fmt "(@[%a@])" pp_expr e

and pp_pattern fmt (p : patt_loc) =
  match p.node with
  | P_Var (x, None) -> Format.pp_print_string fmt x
  | P_Var (x, Some t) -> Format.fprintf fmt "(@[%s : %a@])" x pp_ty t

and pp_ty fmt (t : ty_loc) =
  match t.node with
  | T_Type -> Format.pp_print_string fmt "type"
  | T_Expr e -> pp_expr fmt e
  | T_Arrow (t1, t2) ->
      Format.fprintf fmt "@[@[%a@] -> @[%a@]@]" pp_ty t1 pp_ty t2
  | T_Prod tl ->
      let pp_sep fmt () = Format.fprintf fmt " * " in
      Format.pp_print_list ~pp_sep pp_ty fmt tl

let pp_definition fmt (d : definition_pos) =
  let id, expr = d.node in
  Format.fprintf fmt "@[%s@ := @[%a@].@]" id pp_expr expr

let pp_program =
  Format.pp_print_list
    ~pp_sep:(fun fmt () -> Format.fprintf fmt "@.")
    pp_definition
