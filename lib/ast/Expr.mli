open InputUtils

type expr = Var of Ident.t | App of t * t | Abs of Pattern.t * t
and t = expr Location.node_location

val make_var : ?loc:Location.t -> Ident.t -> t
(** make a variable expression *)

val make_app : ?loc:Location.t -> t -> t -> t
(** make application expression *)

val make_abs : ?loc:Location.t -> Pattern.t -> t -> t
(** make abstraction expression *)

val equal : t -> t -> bool
(** equality of expression *)

val pp_print : Format.formatter -> t -> unit
(** pretty printing of expression *)
