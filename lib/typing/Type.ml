type t = T_Type | T_Var of Ident.t
type env = t Env.t

let t_type = T_Type
let t_var x = T_Var x
let ty _ = T_Type

let equal t1 t2 =
  match (t1, t2) with
  | T_Type, T_Type -> true
  | T_Var x1, T_Var x2 -> x1 = x2
  | _ -> false

(*
let rec pp_product fmt = function
  | T_Prod tl ->
      let pp_content fmt tl =
        let pp_sep fmt () = Format.fprintf fmt "*@ " in
        Format.pp_print_list ~pp_sep pp_arrow fmt tl
      in
      Format.fprintf fmt "@[%a@]" pp_content tl
  | t -> pp_arrow fmt t

and pp_arrow fmt = function
  | T_Arrow (t1, t2) ->
      Format.fprintf fmt "@[@[%a@] -> @[%a@]@]" pp_arrow t1 pp_value t2
  | t -> pp_value fmt t
*)
let pp_value fmt = function
  | T_Type -> Format.pp_print_string fmt "type"
  | T_Var x -> Format.pp_print_string fmt x

let pp_print = pp_value
