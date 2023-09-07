let directory = "variable_files"
let test_file ~msg = Utils.test_file ~msg directory

let test_simple () =
    test_file ~msg:"variable simple" "simple"


let tests_cases =
  Alcotest.
    [
      test_case "variable-simple" `Quick test_simple;
    ]
