type loc = Lexing.position * Lexing.position
type t = Loc of loc | Unknown
type 'a node_loc = { node : 'a; location : t }

val unknown : t
val loc : unit -> t
val make_node_loc : ?loc:loc -> 'a -> 'a node_loc
