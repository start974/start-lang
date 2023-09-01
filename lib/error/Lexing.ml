open InputUtils

type msg_type =
  | Char of char
  | Comment
  | EscapeChar of char
  | ReadStringTerm
  | ReadCharTerm
  | ReadCharEmpty
  | ReadCharFinish of char

type u = { position : Position.t; msg_type : msg_type }

module LexingError = PositionError.Make (struct
  type t = u

  let message _ = "Lexing error."
  let position { position; _ } = position

  let hint { msg_type; _ } =
    Some
      (match msg_type with
      | Char c -> Format.sprintf "Unexpected character '%c'." c
      | Comment -> "Unterminated comment."
      | EscapeChar c -> Format.sprintf "Cannot escape char '\\%c'." c
      | ReadStringTerm -> "Unterminated string."
      | ReadCharTerm -> "Unterminated char."
      | ReadCharEmpty -> "Char is empty."
      | ReadCharFinish c ->
          Format.sprintf "Char is one character, expected \"'\" not '%c'." c)

  let err_cathegory = ErrorCat.Error
end)

include LexingError

let fail_comment (position : Position.t) = fail { position; msg_type = Comment }
let fail_char position c = fail { position; msg_type = Char c }
let fail_escape_char position c = fail { position; msg_type = EscapeChar c }

let fail_readstring_terminate position =
  fail { position; msg_type = ReadStringTerm }

let fail_readchar_terminate position =
  fail { position; msg_type = ReadCharTerm }

let fail_readchar_empty position = fail { position; msg_type = ReadCharEmpty }

let fail_readchar_finish position c =
  fail { position; msg_type = ReadCharFinish c }
