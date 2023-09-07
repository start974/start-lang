type t = T_Type | T_Var of Ident.t
type env = t Env.t

val t_type : t
(* [t_type] type of type type *)

val t_var : string -> t
(* [t_var x] type of type [x] *)

val equal : t -> t -> bool
(* type equal *)

val ty : t -> t
(* just type *)

val pp_print : Format.formatter -> t -> unit
(** pretty print with type expressions typed by default is false *)
