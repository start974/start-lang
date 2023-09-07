open Frontend
open InputUtils

let test_parsing ~msg ?(text_expect = "") text_to_parse =
  let msg = Format.sprintf "%s -- %s" msg text_to_parse in
  let input = Inputs.register_string text_to_parse in
  let prgm = TestUtils.Parse.to_parse_tree ~msg input in
  let text_get = Format.asprintf "%a" ParseTree.pp_program prgm in
  let text_expect = if text_expect == "" then text_to_parse else text_expect in
  Alcotest.(check string) msg text_expect text_get
