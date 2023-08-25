open InputUtils

type msg_type =
  | Char of char
  | Comment
  | EscapeChar of char
  | ReadStringTerm
  | ReadStringUnexpectedChar of char
  | ReadCharTerm
  | ReadCharEmpty
  | ReadCharFinish of char

type u = { position : Position.t; msg_type : msg_type }

module LexingError = PositionError.Make (struct
  type t = u

  let message { msg_type; _ } =
    "Lexing error, "
    ^
    match msg_type with
    | Comment -> "unterminated comment."
    | Char c -> Format.sprintf "unexpected character '%c'." c
    | EscapeChar c -> Format.sprintf "cannot escape char '\\%c'." c
    | ReadStringTerm -> "unterminated string."
    | ReadStringUnexpectedChar c ->
        Format.sprintf "illegal string character '%c'" c
    | ReadCharTerm -> "unterminated char."
    | ReadCharEmpty -> "char is empty."
    | ReadCharFinish c -> Format.sprintf "char is terminate, expected \"'\" not '%c'" c

  let position { position; _ } = position
  let hint _ = None
  let err_cathegory = ErrorCat.Error
end)

include LexingError

let fail_comment (position : Position.t) = fail { position; msg_type = Comment }
let fail_char position c = fail { position; msg_type = Char c }

let fail_escape_char position  c =
  fail { position; msg_type = EscapeChar c }

let fail_readstring_terminate position =
  fail { position; msg_type = ReadStringTerm }

let fail_readstring_unexpected_char position c =
  fail { position; msg_type = ReadStringUnexpectedChar c }

let fail_readchar_terminate position =
  fail { position; msg_type = ReadCharTerm }

let fail_readchar_empty position =
  fail { position; msg_type = ReadCharEmpty }

let fail_readchar_finish position  c =
  fail { position; msg_type = ReadCharFinish c }
