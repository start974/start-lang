type t = Error | Warning

let pp_color e fmt =
  match e with
  | Error -> Format.fprintf fmt "@{<red>%a@}"
  | Warning -> Format.fprintf fmt "@{<orange>%a@}"

let pp_print fmt = function
  | Error -> Format.fprintf fmt "@{<red>%s@}" "Error"
  | Warning -> Format.fprintf fmt "@{<orange>%s@}" "Warning"
