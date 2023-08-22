type t = Lexing.position

val unknown : t
(** make unknown position *)

val is_unknown : t -> bool
(** check if is unknown position *)

val file : t -> string option
(** file in position *)

val line : t -> int
(** line in position *)

val char : t -> int
(** char position in position *)

val pp_print : Format.formatter -> t -> unit
(** pretty print position *)
