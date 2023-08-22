type t = Position.t * Position.t
type 'a node_location = { node : 'a; location : t }

val unknown : t
(** make unknown location *)

val is_unknown : t -> bool
(** check if is unknown location *)

val file : t -> string option
(** file in location *)

val lines : t -> int * int
(** line in position *)

val chars : t -> int * int
(** char position in position *)

val make_node_loc : ?loc:t -> 'a -> 'a node_location
(** make a node location *)

val pp_print : Format.formatter -> t -> unit
(** pretty print location **)
