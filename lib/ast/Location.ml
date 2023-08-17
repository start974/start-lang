type loc = Lexing.position * Lexing.position
type t = Loc of loc | Unknown
type 'a node_loc = { node : 'a; location : t }

let unknown = Unknown
let loc () = Loc (Parsing.symbol_start_pos (), Parsing.symbol_end_pos ())

let make_node_loc ?loc node =
  let location = match loc with None -> Unknown | Some l -> Loc l in
  { location; node }
