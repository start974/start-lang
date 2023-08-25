{
    open Lexing
    open Parser
}

let eol = '\n'
let white_space = [' ' '\t' '\r']

let alpha = ['a' - 'z' 'A' - 'Z']

(* digits *)
let digit_dec = ['0' - '9']
let digit_bin = ('0' | '1')
let digit_hex = ['0' - '9' 'A' - 'F' 'a' - 'f']
let digit_oct = ['0' - '7']

let exp_prefix = ('e' | 'E') ('+' | '-')?

let ident = ['a' - 'z'] (alpha | '_' | digit_dec)*

(* integer *)
let num_dec = digit_dec+
let num_bin = '0' ('b' | 'B') digit_bin+
let num_hex = '0' ('x' | 'X') digit_hex+
let num_oct = '0' ('o' | 'O') digit_oct+
let num = '-'? (num_dec | num_bin | num_hex | num_oct)

(* escaped special *)
let escaped_dec = digit_dec digit_dec digit_dec
let escaped_hex = ('X' | 'x') digit_hex digit_hex
let escaped_oct = ('O' | 'o') ['0' - '3'] digit_oct digit_oct
let escaped_code = escaped_dec | escaped_hex | escaped_oct

rule token = parse
  | eol
        { MenhirLib.LexerUtil.newline lexbuf ; token lexbuf }
  | white_space+
        { token lexbuf }
  | "--" [^ '\n']* ['\n']
        { MenhirLib.LexerUtil.newline lexbuf ; token lexbuf }
  | "(*"
        { let position = Lexing.lexeme_start_p lexbuf in
          comment position lexbuf; token lexbuf }
(* constant expr *)
  | '(' white_space* ')'
        { E_UNIT }
  | "true"
        { E_BOOL true }
  | "false"
        { E_BOOL false }
  | num as n
        { E_INT (Z.of_string n) }
  | "'"
        { let position = Lexing.lexeme_start_p lexbuf in
          read_char position lexbuf }
  | '"'
        { let position = Lexing.lexeme_start_p lexbuf in
          read_string position (Buffer.create 17) lexbuf }
(* tokens *)
  | "fn"
        { FN }
  | '.'
        { DOT }
  | "=>"
        { ARROW_FN }
  | ":="
        { EQUAL_DEF }
  | "*"
        { STAR }
(*  | "|"*)
      (*{ PIPE }*)
  | ","
        { COMMA }
  | "("
        { LPAR }
  | ")"
        { RPAR }
  | "->"
        { ARROW_TY }
  | "type"
        { TYPE }
  | ":"
        { SEMI }
  | ident as id
        { IDENT id }
  | _ as c
        { let position = Lexing.lexeme_start_p lexbuf in
          Error.Lexing.fail_char position c }
  | eof
        { EOF }

and comment start_pos = parse
  | "*)"
        { () }
  | '\n'
        { new_line lexbuf; comment start_pos lexbuf }
  | _
        { comment start_pos lexbuf }
  | eof
        { Error.Lexing.fail_comment start_pos }

and escaped_char = parse
  | '"'
        { '"' }
  | '\''
        { '\'' }
  | '\\'
        { '\\' }
  | 'n'
        { '\n' }
  | 'r'
        { '\r' }
  | 't'
        { '\t' }
  | 'b'
        { '\b' }
  | escaped_code as ascii_n
        { Char.chr (int_of_string ("0" ^ ascii_n)) }
  | _  as c
        { let position = Lexing.lexeme_start_p lexbuf in
          Error.Lexing.fail_escape_char position c }

and read_string start_pos buf = parse
  | '"'
        { E_STRING (Buffer.contents buf) }
  | '\\'
        { let c = escaped_char lexbuf in
          Buffer.add_char buf c;
          read_string start_pos buf lexbuf }
  | _ as c
        { Buffer.add_char buf c;
          read_string start_pos buf lexbuf }
  | eof
        { Error.Lexing.fail_readstring_terminate start_pos }

and finish_read_char c = parse
  | '\''
        { E_CHAR c }
  | _ as c
        { let position = Lexing.lexeme_start_p lexbuf in
          Error.Lexing.fail_readchar_finish position c }

and read_char start_pos = parse
  | '\\'
        { let c = escaped_char lexbuf in finish_read_char c lexbuf }
  | '\''
        { Error.Lexing.fail_readchar_empty start_pos }
  | ([^ '\'' '\\'] as c )
        { finish_read_char c lexbuf }
  | eof
        { Error.Lexing.fail_readchar_terminate start_pos }
