type u = Err_Lexer of Lexer.t | Err_Parser of Parser.t

include ErrorBuilder.S with type t = u

val error_lexer : Lexer.t -> 'a res
(* error lexing *)

val error_parser : Parser.t -> 'a res
(* error parsing *)
