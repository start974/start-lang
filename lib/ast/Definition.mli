type def = Ident.t * Expr.t
type t = def Location.node_loc

val make : ?loc:Location.loc -> Ident.t -> Expr.t -> t
(** make a defintion *)

val equal : t -> t -> bool
(** equality of definition *)

val pp_print : Format.formatter -> t -> unit
(** pretty printing of defintion *)
