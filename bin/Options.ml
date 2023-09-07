let verbose = ref false
let parse_only = ref false
let no_color = ref false
let print_parse_tree = ref false
let print_ast = ref false

let spec =
  [
    ("--parse-only", Arg.Set parse_only, " parse only");
    ("--print-parse-tree", Arg.Set print_parse_tree, " print parse tree");
    ("--print-ast", Arg.Set print_ast, " print typed ast");
    ("--no-color", Arg.Set no_color, " no color printing");
    ("--verbose", Arg.Set verbose, " show processing messages");
  ]

let usage = Printf.sprintf "Usage: %s <options> [filename]\n" Sys.argv.(0)

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

let verbose =
  Utils.Printing.verbose_active := !verbose;
  !verbose

let parse_only = !parse_only
let print_parse_tree = !print_parse_tree
let print_ast = !print_ast

let reset_ppf =
  if !no_color then fun () -> () else Spectrum.prepare_ppf Format.std_formatter
