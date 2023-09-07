open Utils

module type S = sig
  type t_in
  type t_out

  val run : t_in -> t_out
end

let exit n =
  Options.reset_ppf ();
  InputUtils.Inputs.clean ();
  Stdlib.exit n

module Input :
  S with type t_in = string option and type t_out = InputUtils.Inputs.input =
struct
  open InputUtils

  type t_in = string option
  type t_out = Inputs.input

  let run file =
    match file with
    | None -> Inputs.std_in
    | Some file -> Inputs.register_file file
end

module Parsing :
  S
    with type t_in = InputUtils.Inputs.input
     and type t_out = Frontend.ParseTree.program = struct
  open Frontend

  type t_in = InputUtils.Inputs.input
  type t_out = ParseTree.program

  let parse_print prgm =
    Printing.verbose "%a" Printing.head_1 "PARSING";
    let pp fmt =
      if Options.print_parse_tree then Format.printf fmt
      else Printing.verbose fmt
    in
    pp "%a@." ParseTree.pp_program prgm

  let run input =
    match Parse.program input with
    | prgm ->
        parse_print prgm;
        if Options.parse_only then exit 0;
        prgm
    | exception Error.Parsing.Err e ->
        Format.printf "%a" Error.Parsing.pp_print e;
        exit 1
    | exception Error.Lexing.Err e ->
        Format.printf "%a" Error.Lexing.pp_print e;
        exit 1
end

module Typing :
  S with type t_in = Frontend.ParseTree.program and type t_out = Ast.Program.t =
struct
  type t_in = Frontend.ParseTree.program
  type t_out = Ast.Program.t

  let typing_print ast =
    Printing.verbose "%a" Printing.head_1 "TYPING";
    let pp fmt =
      if Options.print_ast then Format.printf fmt else Printing.verbose fmt
    in
    pp "%a@." (Ast.Program.pp_print ~show_type:true) ast

  let run parse_tree =
    let ast = Ast.Program.from_parse_tree parse_tree in
    typing_print ast;
    ast
end
