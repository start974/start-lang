let () =
  Alcotest.run "Error"
    [
      ("lexing", Lexing.tests_cases);
    ]
