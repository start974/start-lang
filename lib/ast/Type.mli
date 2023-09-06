type t = T_Type | T_Var of Ident.t

val t_type : t
(* [t_type] type of type type *)

val t_var : string -> t
(* [t_var x] type of type [x] *)

val t_unit : t
(* [t_unit] type of type char *)

val t_bool : t
(* [t_bool] type of type bool *)

val t_int : t
(* [t_bool] type of type int *)

val t_char : t
(* [t_int] type of type char *)

val t_string : t
(* [t_string] type of type string *)

val equal : t -> t -> bool
(* type equal *)

val ty: t -> t
(* just type *)

val pp_print : Format.formatter -> t -> unit
(** pretty print with type expressions typed by default is false *)
