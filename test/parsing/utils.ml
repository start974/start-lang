open Frontend
open InputUtils

let test_parsing ~msg ?(text_expect = "") text_to_parse =
  let input = Inputs.register_string text_to_parse in
  let prgm = Parse.program input in
  let text_get = Format.asprintf "%a" Ast.pp_program prgm in
  let text_expect = if text_expect == "" then text_to_parse else text_expect in
  let msg' = Format.sprintf "%s -- %s" msg text_to_parse in
  Alcotest.(check string) msg' text_expect text_get
