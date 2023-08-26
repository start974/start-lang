open Utils

let test_simple_type () =
  let msg = "same type" in
  test_parsing ~msg "t1 := type."

let test_var_type () =
  let msg = "same type var" in
  test_parsing ~msg "t1 := ta."

let test_arrow_type () =
  let msg = "same arrow type" in
  test_parsing ~msg "t1 := type -> type.";
  test_parsing ~msg "t1 := ta -> tb.";
  test_parsing ~msg "t1 := ta -> tb -> tc.";
  test_parsing ~msg ~text_expect:"t1 := ta -> tb -> tc."
    "t1 := (ta -> tb) -> tc.";
  test_parsing ~msg "t1 := ta -> (tb -> tc)."

let test_fun_with_type () =
  let msg = "same fun type" in
  test_parsing ~msg ~text_expect:"t1 := λ (a : type) => a."
    "t1 := fn (a: type) => a."

let test_app_type () =
  let msg = "same fun type" in
  test_parsing ~msg ~text_expect:"t1 := (λ (a : type) => a) t."
    "t1 := (fn (a: type) => a) t.";
  test_parsing ~msg "t1 := list t.";
  test_parsing ~msg "t1 := list (t1, t2)."

let tests_cases =
  Alcotest.
    [
      test_case "test-var-type" `Quick test_var_type;
      test_case "test-simple-type" `Quick test_simple_type;
      test_case "test-arrow-type" `Quick test_arrow_type;
      test_case "test-fun-with-type" `Quick test_fun_with_type;
      test_case "test-app-with-type" `Quick test_app_type;
    ]
