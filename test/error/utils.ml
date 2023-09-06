open Frontend
open InputUtils

let make_path directory name ext = Format.sprintf "%s/%s.%s" directory name ext
let lexing_directory = "lexing_files"
let parsing_directory = "parsing_files"

let get_inputs ~msg directory file_name =
  let make_path = make_path directory file_name in
  let st_file = make_path "st" and err_file = make_path "err" in
  let msg = Format.sprintf "%s -- %s" msg file_name
  and input = Inputs.register_file st_file
  and expected = In_channel.with_open_bin err_file In_channel.input_all in
  (msg, input, expected)

let test_error_lexing ~msg file_name =
  let msg, input, expected = get_inputs ~msg lexing_directory file_name in
  match Parse.program input with
  | prgm ->
      Alcotest.fail
        (Format.asprintf "%s@.%s@.%a" msg "is correctly parsed." ParseTree.pp_program
           prgm)
  | exception Error.Parsing.Err e ->
      Alcotest.fail
        (Format.asprintf "%s@.%s@.%a" msg "as a parsing error."
           Error.Parsing.pp_print e)
  | exception Error.Lexing.Err e ->
      let received = Format.asprintf "%a" Error.Lexing.pp_print e in
      Alcotest.(check string) msg expected received

let test_error_parsing ~msg file_name =
  let msg, input, expected = get_inputs ~msg parsing_directory file_name in
  match Parse.program input with
  | prgm ->
      Alcotest.fail
        (Format.asprintf "%s@.%s@.%a" msg "is correctly parsed." ParseTree.pp_program
           prgm)
  | exception Error.Lexing.Err e ->
      Alcotest.fail
        (Format.asprintf "%s\n%s\n%a" msg "as a lexing error."
           Error.Lexing.pp_print e)
  | exception Error.Parsing.Err e ->
      let received = Format.asprintf "%a" Error.Parsing.pp_print e in
      Alcotest.(check string) msg expected received
