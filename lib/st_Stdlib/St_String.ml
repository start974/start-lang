open Typing

type t = string

let ty = Type.t_var "string"

let pp_print fmt s =
  let buffer = Buffer.create 17 in
  let update_buffer first s =
    let es = String.escaped s in
    if not first then Buffer.add_string buffer "\\\"";
    Buffer.add_string buffer es;
    false
  in
  ignore @@ List.fold_left update_buffer true (String.split_on_char '"' s);
  Format.fprintf fmt "\"%s\"" (Buffer.contents buffer)
