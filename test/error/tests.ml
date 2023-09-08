let () =
  Alcotest.run "Error"
    [
      ("Lexing", Test_lexer.tests_cases);
      ("Parsing", Test_parser.tests_cases);
      ("Ast", Test_ast.tests_cases);
    ]
