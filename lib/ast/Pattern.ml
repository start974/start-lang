open InputUtils

type pattern = Var of Ident.t
type t = pattern Location.node_location

let make_var ?loc x = Location.make_node_loc ?loc (Var x)

let equal (p1 : t) (p2 : t) : bool =
  match (p1.node, p2.node) with Var x1, Var x2 -> Ident.equal x1 x2

let pp_print fmt ({ node; _ } : t) =
  match node with Var x -> Format.fprintf fmt "%s" x
