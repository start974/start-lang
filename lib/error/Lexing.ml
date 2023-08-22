open InputUtils

type msg_type = Char of char | Comment
type u = { position : Position.t; msg_type : msg_type }

module LexingError = PositionError.Make (struct
  type t = u

  let message { msg_type; _ } =
    "Lexing error, "
    ^
    match msg_type with
    | Comment -> "unterminated comment."
    | Char c -> Format.sprintf "unexpected character '%c'." c

  let position { position; _ } = position
  let hint _ = None
  let err_cathegory = ErrorCat.Error
end)

include LexingError

let fail_comment (position : Position.t) = fail { position; msg_type = Comment }
let fail_char position c = fail { position; msg_type = Char c }
