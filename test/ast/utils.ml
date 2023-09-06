open Ast
open InputUtils
open Frontend

let type_testable = Alcotest.testable Ast.Type.pp_print Ast.Type.equal

let parse_handle ~msg text_to_parse =
  let input = Inputs.register_string text_to_parse in
  match Parse.program input with
  | prog ->
      let ({ node = { body; _ }; _ } : ParseTree.def_loc) = List.hd prog in
      body
  | exception Error.Lexing.Err e ->
      Alcotest.fail (Format.asprintf "%s@.%a" msg Error.Lexing.pp_print e)
  | exception Error.Parsing.Err e ->
      Alcotest.fail (Format.asprintf "%s@.%a" msg Error.Parsing.pp_print e)

let test_expr ~msg ?(txt_expect = "") expr_txt ty_expect =
  let msg = Format.sprintf "%s -- %s" msg expr_txt in
  let def_txt = Format.sprintf "test := %s." expr_txt in
  let expr_parse = parse_handle ~msg def_txt in
  let expr_ast = Expression.from_parse_tree expr_parse in
  let ty_receive = Expression.ty expr_ast in
  Alcotest.check type_testable msg ty_expect ty_receive;

  let txt_expect = if txt_expect == "" then expr_txt else txt_expect in
  let txt_receive = Format.asprintf "%a" Expression.pp_print expr_ast in
  Alcotest.(check string) msg txt_expect txt_receive
