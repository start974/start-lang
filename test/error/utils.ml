open Frontend

let lexing_directory = "lexing_files"
let parsing_directory = "parsing_files"

let get_inputs ~msg directory file_name =
  TestUtils.LoadFile.load ~msg directory file_name "st" "err"

let test_error_lexing ~msg file_name =
  let msg, input, expected = get_inputs ~msg lexing_directory file_name in
  match Parse.program input with
  | prgm ->
      Alcotest.fail
        (Format.asprintf "%s@.%s@.%a" msg "is correctly parsed."
           ParseTree.pp_program prgm)
  | exception Error.Parsing.Err e ->
      Alcotest.fail
        (Format.asprintf "%s@.%s@.%a" msg "as a parsing error."
           Error.Parsing.pp_print e)
  | exception Error.Lexing.Err e ->
      let received = Format.asprintf "%a" Error.Lexing.pp_print e in
      Alcotest.(check string) msg expected received

let test_error_parsing ~msg file_name =
  let msg, input, expected = get_inputs ~msg parsing_directory file_name in
  match Parse.program input with
  | prgm ->
      Alcotest.fail
        (Format.asprintf "%s@.%s@.%a" msg "is correctly parsed."
           ParseTree.pp_program prgm)
  | exception Error.Lexing.Err e ->
      Alcotest.fail
        (Format.asprintf "%s\n%s\n%a" msg "as a lexing error."
           Error.Lexing.pp_print e)
  | exception Error.Parsing.Err e ->
      let received = Format.asprintf "%a" Error.Parsing.pp_print e in
      Alcotest.(check string) msg expected received
