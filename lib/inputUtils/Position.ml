type t = Lexing.position

let unknown = Lexing.dummy_pos
let is_unknown x = x == unknown
let file pos = if is_unknown pos then None else Some pos.pos_fname
let line (pos : t) = pos.pos_lnum
let char (pos : t) = pos.pos_cnum - pos.pos_bol + 1

let pp_file fmt pos =
  Option.iter (Format.fprintf fmt "File \"%s\", ") (file pos)

let pp_line fmt pos = Format.fprintf fmt "line %d, " (line pos)
let pp_char fmt pos = Format.fprintf fmt "character %d:" (char pos)

let pp_print fmt pos =
  if is_unknown pos then Format.fprintf fmt "At an unknown position:\n"
  else Format.fprintf fmt "%a%a%a" pp_file pos pp_line pos pp_char pos
