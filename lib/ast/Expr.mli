type expr = Var of Ident.t | App of t * t | Abs of Pattern.t * t
and t = expr Location.node_loc

val make_var : ?loc:Location.loc -> Ident.t -> t
(** make a variable expression *)

val make_app : ?loc:Location.loc -> t -> t -> t
(** make application expression *)

val make_abs : ?loc:Location.loc -> Pattern.t -> t -> t
(** make abstraction expression *)

val equal : t -> t -> bool
(** equality of expression *)

val pp_print : Format.formatter -> t -> unit
(** pretty printing of expression *)
