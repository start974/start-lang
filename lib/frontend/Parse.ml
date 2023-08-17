open Lexing
open Lexer
open Parser
open MenhirLib
module I = Parser.MenhirInterpreter

type parsing_err = {
  location : Ast.Location.loc;
  indication : string;
  message : string;
}

type result_file = (Ast.Program.t, parsing_err) result

let make_supplier = I.lexer_lexbuf_to_supplier Lexer.token

let env checkpoint =
  match checkpoint with I.HandlingError env -> env | _ -> assert false

let show text positions =
  let open ErrorReports in
  extract text positions |> sanitize |> compress
  |> shorten 20 (* max width 43 *)

let state checkpoint : int =
  match I.top (env checkpoint) with
  | Some (I.Element (s, _, _, _)) -> I.number s
  | None ->
      (* Hmm... The parser is in its initial state. The incremental API
         currently lacks a way of finding out the number of the initial
         state. It is usually 0, so we return 0. This is unsatisfactory
         and should be fixed in the future. *)
      0

let get text checkpoint i =
  match I.get i (env checkpoint) with
  | Some (I.Element (_, _, pos1, pos2)) -> show text (pos1, pos2)
  | None ->
      (* The index is out of range. This should not happen if [$i]
         keywords are correctly inside the syntax error message
         database. The integer [i] should always be a valid offset
         into the known suffix of the stack. *)
      "???"

let handle_fail text buffer (checkpoint : _ I.checkpoint) : result_file =
  let location = ErrorReports.last buffer in
  let indication_txt = ErrorReports.show (show text) buffer in
  let indication = Format.sprintf "Syntax error %s.\n" indication_txt in
  let message =
    ParserMessages.message (state checkpoint)
    |> ErrorReports.expand (get text checkpoint)
  in
  Result.Error { location; indication; message }

let parse text lexbuf =
  let supplier = make_supplier lexbuf in
  let buffer, supplier = ErrorReports.wrap_supplier supplier in
  let succeed = Result.ok
  and failed = handle_fail text buffer
  and checkpoint = Incremental.program lexbuf.lex_curr_p in
  try MenhirInterpreter.loop_handle succeed failed supplier checkpoint
  with Lexical_error s ->
    (* TODO return error token *)
    let error_msg = Format.sprintf "Lexical error: %s@." s in
    failwith error_msg

let from_file file_name =
  let text, lexbuf = LexerUtil.read file_name in
  parse text lexbuf

let from_string text =
  let lexbuf = Lexing.from_string text |> LexerUtil.init "<string>" in
  parse text lexbuf

let from_stdin () = failwith "WIP"

exception ParsingError of string

let get_program = function
  | Ok res -> res
  | Error { location; indication; message } ->
      let msg =
        Format.sprintf "%s%s%s%!" (LexerUtil.range location) indication message
      in
      raise (ParsingError msg)
