open Parsing
open S_Lambda_Calculus
open Alcotest

let () =
  run "Parsing"
    [
      ( "lambda-calculus",
        [
          test_case "test-var-def" `Quick test_var;
          test_case "test-app-2" `Quick test_app;
          test_case "test-app-3" `Quick test_app3;
          test_case "test-abs" `Quick test_abs;
          test_case "test-abs-and-app" `Quick test_abs_app;
        ] );
    ]
