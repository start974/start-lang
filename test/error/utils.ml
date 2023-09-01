open Frontend
open InputUtils

let make_path directory name ext = Format.sprintf "%s/%s.%s" directory name ext
let lexing_directory = "lexing_files"

let test_error_lexing ~msg file_name =
  let make_path = make_path lexing_directory file_name in
  let st_file = make_path "st"
  and err_file = make_path "err"
  and msg = Format.sprintf "%s -- %s" msg file_name in
  let input = Inputs.register_file st_file in
  match Parse.program input with
  | prgm ->
      Alcotest.fail
        (Format.asprintf "%s@.%s@.%a" msg "is correctly parsed." Ast.pp_program
           prgm)
  | exception Error.Parsing.Err e ->
      Alcotest.fail
        (Format.asprintf "%s@.%s@.%a" msg "as a parsing error."
           Error.Parsing.pp_print e)
  | exception Error.Lexing.Err e ->
      let received = Format.asprintf "%a@." Error.Lexing.pp_print e
      and expected = In_channel.with_open_bin err_file In_channel.input_all in
      Alcotest.(check string) msg received expected
