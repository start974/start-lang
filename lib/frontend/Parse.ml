open Lexing
open InputUtils
open MenhirLib
module I = Parser.MenhirInterpreter
module ErrorParser = Error.Frontend.Parser
module ErrorLexer = Error.Frontend.Lexer
module Error = Error.Frontend.Parse

type 'a res = 'a Error.res

let get_lexbuf input =
  let text = Inputs.get_all input in
  let pos_fname = Inputs.to_string input in
  let lexbuf = Lexing.from_string text in
  lexbuf.lex_curr_p <- { pos_fname; pos_lnum = 1; pos_bol = 0; pos_cnum = 0 };
  lexbuf

let get_token env i =
  let location =
    match I.get i env with
    | Some (I.Element (_, _, s_pos, e_pos)) -> (s_pos, e_pos)
    | None when i = 0 || I.get (i - 1) env |> Option.is_some -> I.positions env
    | None -> assert false
  in
  let text = Inputs.extract location in
  ErrorReports.(text |> sanitize |> compress |> shorten 15)

let remove_last_eol s =
  let n = String.length s in
  if n > 0 && s.[n - 1] = '\n' then String.sub s 0 (n - 1) else s

let get_parse_error = function
  | I.HandlingError env ->
      (* name num *)
      I.current_state_number env
      (* message *)
      |> ParserMessages.message
      (* message expand *)
      |> ErrorReports.expand (get_token env)
      (* message exand without eol *)
      |> remove_last_eol
      (* error parsing *)
      |> ErrorParser.error_hint (I.positions env)
      |> Result.get_error
      (* error parse *)
      |> Error.error_parser
  | _ -> assert false

let parse input f_incr_parse =
  let lexbuf = get_lexbuf input in
  let position = lexeme_start_p lexbuf in
  let supplier = I.lexer_lexbuf_to_supplier Lexer.token lexbuf
  and checkpoint = f_incr_parse position
  and succed = Result.ok
  and fail = get_parse_error in
  match I.loop_handle succed fail supplier checkpoint with
  | x -> x
  | exception ErrorLexer.Err e -> Error.error_lexer e

let program input = parse input Parser.Incremental.program
