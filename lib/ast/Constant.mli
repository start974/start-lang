open InputUtils

type const =
  | C_Unit
  | C_Bool of bool
  | C_Int of Z.t
  | C_Char of char
  | C_String of string

type t = { loc : Location.t; const : const }

val c_unit : ?loc:Location.t -> unit -> t
(** [c_bool b] make a unit const of [b] *)

val c_bool : ?loc:Location.t -> bool -> t
(** [c_bool b] make a int const of [b] *)

val c_int : ?loc:Location.t -> Z.t -> t
(** [c_int i] make a int const of [i] *)

val c_char : ?loc:Location.t -> char -> t
(** [c_char c] make a char const of [c] *)

val c_string : ?loc:Location.t -> string -> t
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
