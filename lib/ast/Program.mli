module Error = Error.Ast.Program

type t
type res = t Error.res

val from_parse_tree : Frontend.ParseTree.program -> res
(** get program from parse tree *)

val pp_print : ?show_type:bool -> Format.formatter -> t -> unit
(** pretty print program *)
