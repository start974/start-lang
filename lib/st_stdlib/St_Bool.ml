open Typing

type t = bool

let ty = Type.t_var "bool"

let pp_print fmt = function
  | true -> Format.pp_print_string fmt "⊤"
  | false -> Format.pp_print_string fmt "⊥"
