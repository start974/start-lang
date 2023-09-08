open Utils

let test_unit_ident () =
  let msg = "unit not a correct identifier" in
  test_error_parsing ~msg "unit_def1";
  test_error_parsing ~msg "unit_def2"

let test_type_binding () =
  let msg = "type is an incorrect binding" in
  test_error_parsing ~msg "type_as_binding01";
  test_error_parsing ~msg "type_as_binding02";
  test_error_parsing ~msg "type_as_binding03";
  test_error_parsing ~msg "type_as_binding04";
  test_error_parsing ~msg "type_as_binding05";
  test_error_parsing ~msg "type_as_binding06";
  test_error_parsing ~msg "type_as_binding07";
  ()

let test_pattern_product () =
  let msg = "pattern product incorrect" in
  test_error_parsing ~msg "pattern_product01";
  test_error_parsing ~msg "pattern_product02";
  test_error_parsing ~msg "pattern_product03";
  ()

let test_error_semi () =
  let msg = "semi without type" in
  test_error_parsing ~msg "semi_err01";
  let msg = "semi after parenthesis" in
  test_error_parsing ~msg "semi_err02";
  let msg = "semi after =>" in
  test_error_parsing ~msg "semi_err03";
  let msg = "semi after double ," in
  test_error_parsing ~msg "semi_err04";
  let msg = "semi after ->" in
  test_error_parsing ~msg "semi_err05";
  let msg = "semi after arrow type" in
  test_error_parsing ~msg "semi_err06";
  let msg = "semi after expression" in
  test_error_parsing ~msg "semi_err07";
  let msg = "semi after simple ," in
  test_error_parsing ~msg "semi_err08";
  let msg = "semi after equaldef" in
  test_error_parsing ~msg "semi_err09"

let test_fun () =
  let msg = "incorrect separator fn" in
  test_error_parsing ~msg "fun_binder_sep"

let test_def () =
  let msg = "incorrect separator defintion" in
  test_error_parsing ~msg "def_binder_sep"

let test_parenthesis () =
  let msg = "parenthesis not close in pattern" in
  test_error_parsing ~msg "parenthesis1";
  let msg = "parenthesis not open in expression" in
  test_error_parsing ~msg "parenthesis2"

let test_definition_not_end () =
  let msg = "definition not end" in
  test_error_parsing ~msg "definition_not_end"

let tests_cases =
  Alcotest.
    [
      test_case "err-parsing-unit" `Quick test_unit_ident;
      test_case "err-type-binding" `Quick test_type_binding;
      test_case "err-pattern-product" `Quick test_pattern_product;
      test_case "err-semi" `Quick test_error_semi;
      test_case "err-lambda-fn" `Quick test_fun;
      test_case "err-definition" `Quick test_def;
      test_case "err-parenthesis" `Quick test_parenthesis;
      test_case "err-defintion-not-end" `Quick test_definition_not_end;
    ]
