open Utils

let test_variable_def () =
  let msg = "definition of variable" in
  test_error_ast ~msg "variable_def"

let tests_cases =
  Alcotest.[ test_case "err-var-undefined-simple" `Quick test_variable_def ]
