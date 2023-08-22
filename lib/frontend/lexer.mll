{
    open Lexing
    open Parser
}

let alpha = ['a' - 'z' 'A' - 'Z']
let digit = ['0' - '9']
let exponent = ('e' | 'E') ('+' | '-')? digit+

let ident = ['a' - 'z'] (alpha | '_' | digit)*

rule token = parse
  | '\n'
      { MenhirLib.LexerUtil.newline lexbuf ; token lexbuf }
  | [' ' '\t' '\r']+
      { token lexbuf }
  | "--" [^ '\n']* ['\n']
      { MenhirLib.LexerUtil.newline lexbuf ; token lexbuf }
  | "(*"
      {  let position = Lexing.lexeme_start_p lexbuf in
         comment position lexbuf; token lexbuf }
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
  | ident as id
      { IDENT id }
  | _ as c
      { let position = Lexing.lexeme_start_p lexbuf in
        Error.Lexing.fail_char position c }
  | eof
      { EOF }

and comment start_pos = parse
  | "*)" { () }
  | '\n' { new_line lexbuf; comment start_pos lexbuf }
  | _    { comment start_pos lexbuf }
  | eof  { Error.Lexing.fail_comment start_pos }
