let test_file ~msg directory file_name =
  let msg, input, expected =
    TestUtils.LoadFile.load ~msg directory file_name "st" "out"
  in
  let prgm = TestUtils.Parse.to_ast ~msg input in
  let receive =
    Format.asprintf "%a" (Ast.Program.pp_print ~show_type:true) prgm
  in
  Alcotest.(check string) msg expected receive
