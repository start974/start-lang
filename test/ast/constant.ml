open Utils

let test_unit () =
  let msg = "check unit" in
  let expr_txt = "()" in
  let ty = Ast.Type.t_unit in
  test_expr ~msg expr_txt ty

let test_bool () =
  let msg = "check bool" in
  let ty = Ast.Type.t_bool in

  let expr_txt = "true" in
  let txt_expect = "⊤" in
  test_expr ~msg ~txt_expect expr_txt ty;

  let expr_txt = "false" in
  let txt_expect = "⊥" in
  test_expr ~msg ~txt_expect expr_txt ty

let test_int () =
  let msg = "check int" in
  let ty = Ast.Type.t_int in
  test_expr ~msg "10" ty;
  test_expr ~msg "0" ty;
  test_expr ~msg "-10" ty

let test_char () =
  let msg = "check char" in
  let ty = Ast.Type.t_char in
  test_expr ~msg "'c'" ty

let test_string () =
  let msg = "check string" in
  let ty = Ast.Type.t_string in
  test_expr ~msg "\"test\"" ty

let test_type () =
  let msg = "check type" in
  let ty = Ast.Type.t_type in
  test_expr ~msg "type" ty

let tests_cases =
  Alcotest.
    [
      test_case "test-unit" `Quick test_unit;
      test_case "test-bool" `Quick test_bool;
      test_case "test-int" `Quick test_int;
      test_case "test-char" `Quick test_char;
      test_case "test-string" `Quick test_string;
      test_case "test-type" `Quick test_type;
    ]
