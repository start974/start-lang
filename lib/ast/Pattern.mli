type pattern = Var of Ident.t
type t = pattern Location.node_loc

val make_var : ?loc:Location.loc -> Ident.t -> t
(** make variable pattern *)

val equal : t -> t -> bool
(** equality of pattern *)

val pp_print : Format.formatter -> t -> unit
(** pretty printing of pattern *)
