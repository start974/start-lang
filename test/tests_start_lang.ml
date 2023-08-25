open Parsing
open Alcotest

let () =
  run "Parsing"
    [
      ( "lambda-calculus", SimpleExpression.tests_cases);
    ]
