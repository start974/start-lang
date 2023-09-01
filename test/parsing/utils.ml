open Frontend
open InputUtils

let parse_handle ~msg text_to_parse =
  let input = Inputs.register_string text_to_parse in
  match Parse.program input with
  | prgm -> prgm
  | exception Error.Lexing.Err e ->
      Alcotest.fail (Format.asprintf "%s@.%a" msg Error.Lexing.pp_print e)
  | exception Error.Parsing.Err e ->
      Alcotest.fail (Format.asprintf "%s@.%a" msg Error.Parsing.pp_print e)

let test_parsing ~msg ?(text_expect = "") text_to_parse =
  let msg = Format.sprintf "%s -- %s" msg text_to_parse in
  let prgm = parse_handle ~msg text_to_parse in
  let text_get = Format.asprintf "%a" Ast.pp_program prgm in
  let text_expect = if text_expect == "" then text_to_parse else text_expect in
  Alcotest.(check string) msg text_expect text_get
