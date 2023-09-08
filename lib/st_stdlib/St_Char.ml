open Typing

type t = char

let ty = Type.t_var "char"

let pp_print fmt c =
  let s = if c = '\'' then "\\'" else Char.escaped c in
  Format.fprintf fmt "'%s'" s
