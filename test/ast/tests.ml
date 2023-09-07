let () =
  Alcotest.run "Ast"
    [ ("constant", Constant.tests_cases); ("variable", Variable.tests_cases) ]
