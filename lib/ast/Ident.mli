type t = string

val equal : t -> t -> bool
val compare : t -> t -> int
val pp_print : Format.formatter -> t -> unit
