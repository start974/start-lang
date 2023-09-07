let directory = "constant_files"
let test_file ~msg = Utils.test_file ~msg directory
let test_bool () = test_file ~msg:"bool constant" "bool"
let test_char () = test_file ~msg:"char constant" "char"
let test_int () = test_file ~msg:"int constant" "int"
let test_string () = test_file ~msg:"string constant" "string"
let test_type () = test_file ~msg:"type constant" "type"
let test_unit () = test_file ~msg:"unit constant" "unit"

let tests_cases =
  Alcotest.
    [
      test_case "constant-bool" `Quick test_bool;
      test_case "constant-char" `Quick test_char;
      test_case "constant-int" `Quick test_int;
      test_case "constant-string" `Quick test_string;
      test_case "constant-type" `Quick test_type;
      test_case "constant-unit" `Quick test_unit;
    ]
