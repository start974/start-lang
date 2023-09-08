open Frontend
module ErrorParser = Error.Frontend.Parser
module ErrorLexer = Error.Frontend.Lexer
module ErrorParse = Error.Frontend.Parse
module ErrorAst = Error.Ast.Program

let lexing_directory = "lexer_files"
let parsing_directory = "parser_files"
let ast_directory = "ast_files"

let get_inputs ~msg directory file_name =
  TestUtils.LoadFile.load ~msg directory file_name "st" "err"

let test_error_lexing ~msg file_name =
  let msg, input, expected = get_inputs ~msg lexing_directory file_name in
  match Parse.program input with
  | Ok prgm ->
      Alcotest.fail
        (Format.asprintf "%s@.%s@.%a" msg "is correctly parsed."
           ParseTree.pp_program prgm)
  | Error (ErrorParse.Err_Parser e) ->
      Alcotest.fail
        (Format.asprintf "%s@.%s@.%a" msg "as a parsing error."
           ErrorParser.pp_print e)
  | Error (ErrorParse.Err_Lexer e) ->
      let received = Format.asprintf "%a" ErrorLexer.pp_print e in
      Alcotest.(check string) msg expected received

let test_error_parsing ~msg file_name =
  let msg, input, expected = get_inputs ~msg parsing_directory file_name in
  match Parse.program input with
  | Ok prgm ->
      Alcotest.fail
        (Format.asprintf "%s@.%s@.%a" msg "is correctly parsed."
           ParseTree.pp_program prgm)
  | Error (ErrorParse.Err_Lexer e) ->
      Alcotest.fail
        (Format.asprintf "%s\n%s\n%a" msg "as a lexing error."
           ErrorLexer.pp_print e)
  | Error (ErrorParse.Err_Parser e) ->
      let received = Format.asprintf "%a" ErrorParser.pp_print e in
      Alcotest.(check string) msg expected received

let test_error_ast ~msg file_name =
  let msg, input, expected = get_inputs ~msg ast_directory file_name in
  let parse_tree = TestUtils.Parse.to_parse_tree ~msg input in
  match Ast.Program.from_parse_tree parse_tree with
  | Ok _ -> Alcotest.fail (Format.sprintf "%s : as no error" msg)
  | Error e ->
      let received = Format.asprintf "%a" ErrorAst.pp_print e in
      Alcotest.(check string) msg expected received
