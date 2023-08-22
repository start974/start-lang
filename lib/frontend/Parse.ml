open Lexing
open InputUtils
module I = Parser.MenhirInterpreter

module ParserInput = struct
  type t = { (*input : Inputs.input;*) lexbuf : Lexing.lexbuf }

  let init input =
    let text = Inputs.get_all input in
    let pos_fname = Inputs.to_string input in
    let lexbuf = Lexing.from_string text in
    lexbuf.lex_curr_p <- { pos_fname; pos_lnum = 1; pos_bol = 0; pos_cnum = 0 };
    { (*input;*) lexbuf }

  let next_token info = Lexer.token info.lexbuf

  let lexer_position { lexbuf; _ } =
    let p_start = lexbuf.lex_start_p and p_end = lexbuf.lex_curr_p in
    (p_start, p_end)

  let next_token_pos info =
    let token = next_token info in
    let p_start, p_end = lexer_position info in
    (token, p_start, p_end)
end

let handle_error ~parser_input env =
  let location =
    match I.top env with
    | None -> Location.unknown
    | Some (I.Element _) (*((*state*) _, (*sem_val*) _, p_start, p_end))*) ->
        ParserInput.lexer_position parser_input
  in
  Error.Parsing.fail location

let rec parse ~parser_input (checkpoint : _ I.checkpoint) =
  match checkpoint with
  | I.InputNeeded _env ->
      let token_pos = ParserInput.next_token_pos parser_input in
      let checkpoint' = I.offer checkpoint token_pos in
      parse ~parser_input checkpoint'
  | I.Shifting _ | I.AboutToReduce _ ->
      let checkpoint' = I.resume checkpoint in
      parse ~parser_input checkpoint'
  | I.HandlingError env -> handle_error ~parser_input env
  | I.Accepted res -> res
  | I.Rejected -> assert false

let program input =
  let parser_input = ParserInput.init input in
  let pos_lexer = parser_input.lexbuf.lex_curr_p in
  let checkpoint = Parser.Incremental.program pos_lexer in
  parse ~parser_input checkpoint
