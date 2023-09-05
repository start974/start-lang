let () =
  Alcotest.run "Error"
    [ ("Lexing", Lexing.tests_cases); ("Parsing", Parsing.tests_cases) ]
