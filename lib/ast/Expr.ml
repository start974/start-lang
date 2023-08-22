open InputUtils

type expr = Var of Ident.t | App of t * t | Abs of Pattern.t * t
and t = expr Location.node_location

let make_var ?loc x = Location.make_node_loc ?loc (Var x)
let make_app ?loc e1 e2 = Location.make_node_loc ?loc (App (e1, e2))
let make_abs ?loc p e = Location.make_node_loc ?loc (Abs (p, e))

let rec equal (e1 : t) (e2 : t) : bool =
  match (e1.node, e2.node) with
  | Var x1, Var x2 -> Ident.equal x1 x2
  | App (el1, er1), App (el2, er2) -> equal el1 el2 && equal er1 er2
  | Abs (p1, e1'), Abs (p2, e2') -> Pattern.equal p1 p2 && equal e1' e2'
  | _ -> false

let rec pp_expr fmt (e : t) =
  match e.node with
  | App (el, er) -> Format.fprintf fmt "@[%a %a@]" pp_expr el pp_term er
  | Abs (pat, e) ->
      Format.fprintf fmt "@[λ %a => %a@]" Pattern.pp_print pat pp_expr e
  | _ -> pp_term fmt e

and pp_term fmt (e : t) =
  match e.node with
  | Var x -> Format.pp_print_string fmt x
  | _ -> Format.fprintf fmt "(@[%a@])" pp_expr e

let pp_print = pp_expr
