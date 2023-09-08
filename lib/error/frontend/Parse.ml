type u = Err_Lexer of Lexer.t | Err_Parser of Parser.t

include ErrorBuilder.Make (struct
  type t = u

  let pp_print fmt = function
    | Err_Lexer e -> Lexer.pp_print fmt e
    | Err_Parser e -> Parser.pp_print fmt e
end)

let error_lexer e = error (Err_Lexer e)
let error_parser e = error (Err_Parser e)
