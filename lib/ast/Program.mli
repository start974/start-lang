type t

val from_parse_tree : Frontend.ParseTree.program -> t
(** get program from parse tree *)

val pp_print : ?show_type:bool -> Format.formatter -> t -> unit
(** pretty print program *)
