open InputUtils

let make_path directory name ext = Format.sprintf "%s/%s.%s" directory name ext

let load ~msg directory file_name ext_in ext_out =
  let make_path = make_path directory file_name in
  let st_file = make_path ext_in and err_file = make_path ext_out in
  let msg = Format.sprintf "%s -- %s" msg file_name
  and input = Inputs.register_file st_file
  and output = In_channel.with_open_bin err_file In_channel.input_all in
  (msg, input, output)
