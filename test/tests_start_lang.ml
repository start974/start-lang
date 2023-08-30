open Parsing
open Alcotest

let () =
  run "Parsing"
    [
      ("expression", Expression.tests_cases);
      ("type", Type.tests_cases);
      ("pattern", Pattern.tests_cases);
    ]
