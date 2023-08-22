type t = Position.t * Position.t
type 'a node_location = { node : 'a; location : t }

let unknown = (Lexing.dummy_pos, Lexing.dummy_pos)
let make_node_loc ?(loc = unknown) node = { location = loc; node }

let is_unknown (p_start, p_end) =
  Position.is_unknown p_start || Position.is_unknown p_end

let file (p_start, _) = Position.file p_start
let lines (p_start, p_end) = (Position.line p_start, Position.line p_end)

let chars ((p_start, p_end) : Lexing.position * Lexing.position) =
  let c_start = p_start.pos_cnum - p_start.pos_bol + 1 in
  let c_end = p_end.pos_cnum - p_start.pos_bol + 1 in
  (c_start, c_end)

let pp_print_file fmt loc =
  Option.iter (fun file -> Format.fprintf fmt "File \"%s\", " file) (file loc)

let pp_print_lines fmt loc =
  let line_start, line_end = lines loc in
  if line_start <> line_end then
    Format.fprintf fmt "lines %d-%d, " line_start line_end
  else Format.fprintf fmt "line %d, " line_start

let pp_print_chars fmt loc =
  let char_start, char_end = chars loc in
  Format.fprintf fmt "characters %d-%d" char_start char_end

let pp_print fmt loc =
  if is_unknown loc then Format.fprintf fmt "At an unknown location:\n"
  else
    Format.fprintf fmt "%a%a%a:" pp_print_file loc pp_print_lines loc
      pp_print_chars loc
