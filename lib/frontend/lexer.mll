{
    open Lexing
    open Parser

    exception Lexical_error of string
}

let alpha = ['a' - 'z' 'A' - 'Z']
let digit = ['0' - '9']
let exponent = ('e' | 'E') ('+' | '-')? digit+

let ident = ['a' - 'z'] (alpha | '_' | digit)*

rule token = parse
  | '\n'
      { new_line lexbuf; token lexbuf }
  | [' ' '\t' '\r']+
      { token lexbuf }
  | eof
      { EOF }
  | "--" [^ '\n']* ['\n']
      { new_line lexbuf; token lexbuf }
  | "(*"
      { comment lexbuf; token lexbuf }
  | "fn"
      { FN }
  | '.'
      { DOT }
  | "=>"
      { ARROW_FN }
  | ":="
      { EQUAL_DEF }
  | "("
      { LPAR }
  | ")"
      { RPAR }
  | ident
      { IDENT (lexeme lexbuf) }
  | _
      { raise (Lexical_error (lexeme lexbuf)) }

and comment = parse
  | "*)" { () }
  | '\n' { new_line lexbuf; comment lexbuf }
  | _    { comment lexbuf }
  | eof  { raise (Lexical_error "unterminated comment") }
