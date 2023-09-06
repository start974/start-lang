let () =
  Alcotest.run "Ast"
    [
      ("constant", Constant.tests_cases);
    ]
