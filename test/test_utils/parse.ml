let to_parse_tree ~msg input =
  match Frontend.Parse.program input with
  | Ok prgm -> prgm
  | Error e ->
      let module Error = Error.Frontend.Parse in
      Alcotest.fail (Format.asprintf "%s@.%a" msg Error.pp_print e)

let to_ast ~msg input =
  let parse_tree = to_parse_tree ~msg input in
  match Ast.Program.from_parse_tree parse_tree with
  | Ok prgm -> prgm
  | Error e ->
      let module Error = Error.Ast.Program in
      Alcotest.fail (Format.asprintf "%s@.%a" msg Error.pp_print e)
