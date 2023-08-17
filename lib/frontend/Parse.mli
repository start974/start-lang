type parsing_err = {
  location : Ast.Location.loc;
  indication : string;
  message : string;
}

type result_file = (Ast.Program.t, parsing_err) result

val from_file : string -> result_file
val from_stdin : unit -> result_file
val from_string : string -> result_file

exception ParsingError of string

(* raise [ParsingError] if result is error *)
val get_program : result_file -> Ast.Program.t
