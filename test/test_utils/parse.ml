let to_parse_tree ~msg input =
  match Frontend.Parse.program input with
  | prgm -> prgm
  | exception Error.Lexing.Err e ->
      Alcotest.fail (Format.asprintf "%s@.%a" msg Error.Lexing.pp_print e)
  | exception Error.Parsing.Err e ->
      Alcotest.fail (Format.asprintf "%s@.%a" msg Error.Parsing.pp_print e)

let to_ast ~msg input =
  input |> to_parse_tree ~msg |> Ast.Program.from_parse_tree
