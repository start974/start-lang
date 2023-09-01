open Lexing
open InputUtils

let get_lexbuf input =
  let text = Inputs.get_all input in
  let pos_fname = Inputs.to_string input in
  let lexbuf = Lexing.from_string text in
  lexbuf.lex_curr_p <- { pos_fname; pos_lnum = 1; pos_bol = 0; pos_cnum = 0 };
  lexbuf

let parse input f_parse =
  let lexbuf = get_lexbuf input in
  match f_parse Lexer.token lexbuf with
  | r -> r
  | exception Parser.Error ->
      let p_start = lexbuf.lex_start_p and p_end = lexbuf.lex_curr_p in
      Error.Parsing.fail_syntax (p_start, p_end)

let program input = parse input Parser.program
