open Typing

type t = unit

let ty = Type.t_var "unit"
let pp_print fmt () = Format.pp_print_string fmt "()"
