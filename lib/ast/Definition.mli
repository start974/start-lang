open InputUtils

type definition = Ident.t * Expr.t
type t = definition Location.node_location

val make : ?loc:Location.t -> Ident.t -> Expr.t -> t
(** make a defintion *)

val equal : t -> t -> bool
(** equality of definition *)

val pp_print : Format.formatter -> t -> unit
(** pretty printing of defintion *)
