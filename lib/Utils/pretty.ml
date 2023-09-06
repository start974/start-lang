let pp_const_string fmt s =
  let buffer = Buffer.create 17 in
  let update_buffer first s =
    let es = String.escaped s in
    if not first then Buffer.add_string buffer "\\\"";
    Buffer.add_string buffer es;
    false
  in
  ignore @@ List.fold_left update_buffer true (String.split_on_char '"' s);
  Format.fprintf fmt "\"%s\"" (Buffer.contents buffer)

let pp_const_char fmt c =
  let s = if c = '\'' then "\\'" else Char.escaped c in
  Format.fprintf fmt "'%s'" s

let pp_const_bool fmt = function
  | true -> Format.pp_print_string fmt "⊤"
  | false -> Format.pp_print_string fmt "⊥"
