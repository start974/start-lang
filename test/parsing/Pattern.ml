open Utils

let test_unit () =
  let msg = "same unit" in
  test_parsing ~msg "z () := 1.";
  test_parsing ~msg "z () : int := 1.";
  test_parsing ~msg "z (x : unit) := 1."

let test_vars () =
  let msg = "same unit" in
  test_parsing ~msg "z x y := 1.";
  test_parsing ~msg ~text_expect:"z x := 1." "z (x) := 1.";
  test_parsing ~msg "z (x : int) y : int := 1.";
  test_parsing ~msg "z (x : int) (_ : int) : int := 1."

let test_product () =
  let msg = "same unit" in
  test_parsing ~msg "z (x, y) := 1.";
  test_parsing ~msg "z (x, y, z) := 1.";
  test_parsing ~msg "z (x, (y, z)) := 1.";
  test_parsing ~msg "z ((x, y), z) := 1.";
  test_parsing ~msg "z ((x, y), z) : int := 1.";
  test_parsing ~msg "z ((x, y, z) : product 3 int) : int := 1."

let tests_cases =
  Alcotest.
    [
      test_case "test-unit" `Quick test_unit;
      test_case "test-vars" `Quick test_vars;
      test_case "test-product" `Quick test_product;
    ]
