open Frontend
open InputUtils

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

let input =
  match file with
  | None -> Inputs.std_in
  | Some file -> Inputs.register_files file

let reset_ppf = Spectrum.prepare_ppf Format.std_formatter

let exit n =
  reset_ppf ();
  Stdlib.exit n

let () =
  try
    let program_ast = Parse.program input in
    if !verbose then Format.printf "%a@." Ast.Program.pp_print program_ast;
    if !parse_only then exit 0
  with
  | Error.Parsing.Err e ->
      Format.printf "%a" Error.Parsing.pp_print e;
      exit 1
  | Error.Lexing.Err e ->
      Format.printf "%a" Error.Lexing.pp_print e;
      exit 1
