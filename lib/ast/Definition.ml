type def = Ident.t * Expr.t
type t = def Location.node_loc

let make ?loc id e = Location.make_node_loc ?loc (id, e)

let equal (p1 : t) (p2 : t) : bool =
  match (p1.node, p2.node) with
  | (id1, e1), (id2, e2) -> Ident.equal id1 id2 && Expr.equal e1 e2

let pp_print fmt ({ node = id, expr; _ } : t) =
    Format.fprintf fmt "@[%a@ :=@ @[%a@].@]" Ident.pp_print id Expr.pp_print expr
