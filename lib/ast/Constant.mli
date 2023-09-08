open Typing
open St_Stdlib

type t =
  | C_Unit
  | C_Bool of St_Bool.t
  | C_Int of St_Int.t
  | C_Char of St_Char.t
  | C_String of St_String.t

val c_unit : t
(** [c_bool b] make a unit const of [b] *)

val c_bool : St_Bool.t -> t
(** [c_bool b] make a int const of [b] *)

val c_int : St_Int.t -> t
(** [c_int i] make a int const of [i] *)

val c_char : St_Char.t -> t
(** [c_char c] make a char const of [c] *)

val c_string : St_String.t -> t
(** [c_string s] make a string const of [s] *)

val ty : t -> Type.t
(** [get_ty e] get type of [e] *)

val from_parse_tree : Frontend.ParseTree.expr_value -> t
(** can fail if is not a constant *)

val equal : t -> t -> bool
(** equality on constant *)

val pp_print : Format.formatter -> t -> unit
(** pretty print constant *)
