open Frontend
open Ast

let get_str_ast program = Format.asprintf "%a" Program.pp_print program
let parse_program text = Parse.from_string text |> Parse.get_program

let test_var () =
  let text = "x := y." in
  let program = parse_program text in
  Alcotest.(check string) "same assign var" text (get_str_ast program)

let test_app () =
  let text = "x := y z." in
  let program = parse_program text in
  Alcotest.(check string) "same app" text (get_str_ast program)

let test_app3 () =
  let text = "f := g h x." in
  let text_expected = "f := g h x." in
  let program = parse_program text in
  Alcotest.(check string) "same app" text_expected (get_str_ast program)

let test_abs () =
  let text = "id := fn x => x ." in
  let text_expected = "id := λ x => x." in
  let program = parse_program text in
  Alcotest.(check string) "same abs" text_expected (get_str_ast program)

let test_abs_app () =
  let text = "z := (fn x => x) y." in
  let text_expected = "z := (λ x => x) y." in
  let program = parse_program text in
  Alcotest.(check string) "same abs app" text_expected (get_str_ast program)
