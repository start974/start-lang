open Utils

let test_unit () =
  let msg = "same unit" in
  test_parsing ~msg "z := ().";
  test_parsing ~msg ~text_expect:"z := ()." "z := (    )."
(*test_parsing ~msg "z := f ()."*)

let test_bool () =
  let msg = "same bool" in
  test_parsing ~msg ~text_expect:"z := ⊤." "z := true.";
  test_parsing ~msg ~text_expect:"z := ⊥." "z := false."

let test_int () =
  let msg = "same int" in
  (* test positive *)
  let text_expect = "z := 20." in
  test_parsing ~msg ~text_expect "z := 20.";
  test_parsing ~msg ~text_expect "z := 0b10100.";
  test_parsing ~msg ~text_expect "z := 0B10100.";
  test_parsing ~msg ~text_expect "z := 0x14.";
  test_parsing ~msg ~text_expect "z := 0X14.";
  test_parsing ~msg ~text_expect "z := 0o24.";
  test_parsing ~msg ~text_expect "z := 0O24.";

  (* test negative *)
  let text_expect = "z := -20." in
  test_parsing ~msg ~text_expect "z := -20.";
  test_parsing ~msg ~text_expect "z := -0b10100.";
  test_parsing ~msg ~text_expect "z := -0B10100.";
  test_parsing ~msg ~text_expect "z := -0x14.";
  test_parsing ~msg ~text_expect "z := -0X14.";
  test_parsing ~msg ~text_expect "z := -0o24.";
  test_parsing ~msg ~text_expect "z := -0O24."

let test_char () =
  let msg = "same char" in
  test_parsing ~msg "z := 'c'.";
  test_parsing ~msg "z := '\\''.";
  test_parsing ~msg ~text_expect:"z := '\"'." "z := '\\\"'.";
  test_parsing ~msg "z := '\\n'.";
  test_parsing ~msg "z := '\\r'.";
  test_parsing ~msg "z := '\\t'.";
  let text_expect = "z := 'A'." in
  test_parsing ~msg ~text_expect "z := '\\065'.";
  test_parsing ~msg ~text_expect "z := '\\x41'.";
  test_parsing ~msg ~text_expect "z := '\\X41'.";
  test_parsing ~msg ~text_expect "z := '\\o101'."

let test_string () =
  let msg = "same char" in
  test_parsing ~msg "z := \"test\".";
  test_parsing ~msg "z := \"\".";
  test_parsing ~msg "z := \"\\\"\".";
  test_parsing ~msg ~text_expect:"z := \"'\"." "z := \"\\'\".";
  test_parsing ~msg "z := \"\\n\".";
  test_parsing ~msg "z := \"\\r\".";
  test_parsing ~msg "z := \"\\t\".";
  let text_expect = "z := \"A\"." in
  test_parsing ~msg ~text_expect "z := \"\\065\".";
  test_parsing ~msg ~text_expect "z := \"\\x41\".";
  test_parsing ~msg ~text_expect "z := \"\\X41\".";
  test_parsing ~msg ~text_expect "z := \"\\o101\".";
  test_parsing ~msg ~text_expect "z := \"\\O101\"."

let test_var () =
  let msg = "same assign var" in
  test_parsing ~msg "x := y."

let test_app () =
  let msg = "same app" in
  test_parsing ~msg "x := f z.";
  test_parsing ~msg "x := f y z.";
  test_parsing ~msg ~text_expect:"x := f y z." "x := (f y) z.";
  test_parsing ~msg "x := f (g z)."

let test_abs () =
  let msg = "same abs" in
  let text_expect = "id := λ x => x." in
  test_parsing ~msg ~text_expect "id := fn x => x ."

let test_abs_app () =
  let msg = "same abs app" in
  test_parsing ~msg ~text_expect:"z := (λ x => x) y." "z := (fn x => x) y."

let test_tupple () =
  let msg = "same tupple" in
  test_parsing ~msg "z := x, y.";
  test_parsing ~msg "z := x, y, z.";
  test_parsing ~msg "z := (x, y), z."

let tests_cases =
  Alcotest.
    [
      test_case "test-unit" `Quick test_unit;
      test_case "test-bool" `Quick test_bool;
      test_case "test-int" `Quick test_int;
      test_case "test-char" `Quick test_char;
      test_case "test-string" `Quick test_string;
      test_case "test-var-def" `Quick test_var;
      test_case "test-app" `Quick test_app;
      test_case "test-abs" `Quick test_abs;
      test_case "test-abs-and-app" `Quick test_abs_app;
      test_case "test-tupple" `Quick test_tupple;
    ]
