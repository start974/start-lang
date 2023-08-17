open Frontend

let usage = Printf.sprintf "Usage: %s <options> [filename]\n" Sys.argv.(0)
let verbose = ref false
let parse_only = ref false

let spec =
  [
    ("--parse-only", Arg.Set parse_only, " parse only");
    ("--verbose", Arg.Set verbose, " show processing messages");
  ]

let file =
  let file_ref = ref None in
  let set_file file =
    match !file_ref with
    | None ->
        file_ref := Some file;
        ()
    | Some _ -> raise (Arg.Bad "Too many arguments")
  in
  Arg.parse spec set_file usage;
  !file_ref

let () =
  let program_ast_res =
    match file with
    | None -> Parse.from_stdin ()
    | Some file -> Parse.from_file file
  in
  try
    let program_ast = Parse.get_program program_ast_res in
    if !verbose then Format.printf "%a@." Ast.Program.pp_print program_ast;
    if !parse_only then exit 0
  with Parse.ParsingError msg ->
    prerr_endline msg;
    exit 1
