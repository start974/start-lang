open InputUtils
open Typing
open St_Stdlib

type const =
  | C_Unit
  | C_Bool of St_Bool.t
  | C_Int of St_Int.t
  | C_Char of St_Char.t
  | C_String of St_String.t

type t = { loc : Location.t; const : const }

val c_unit : ?loc:Location.t -> St_Unit.t -> t
(** [c_bool b] make a unit const of [b] *)

val c_bool : ?loc:Location.t -> St_Bool.t -> t
(** [c_bool b] make a int const of [b] *)

val c_int : ?loc:Location.t -> St_Int.t -> t
(** [c_int i] make a int const of [i] *)

val c_char : ?loc:Location.t -> St_Char.t -> t
(** [c_char c] make a char const of [c] *)

val c_string : ?loc:Location.t -> St_String.t -> t
(** [c_string s] make a string const of [s] *)

val const : t -> const
(** [const c] get const of t [c] *)

val ty : t -> Type.t
(** [get_ty e] get type of [e] *)

val loc : t -> Location.t
(** [loc c] get location of t [c] *)

val equal : t -> t -> bool
(** equality on constant *)

val pp_print : Format.formatter -> t -> unit
(** pretty print constant *)
