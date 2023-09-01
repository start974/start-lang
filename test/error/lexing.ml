open Utils

let test_error_char () = test_error_lexing ~msg:"error char" "char"

let test_comment () =
  let msg = "error comment unclosed" in
  test_error_lexing ~msg "unterminated_comment"

let test_escapedchar () =
  let msg = "error escape char" in
  test_error_lexing ~msg "escaped_char"

let test_string_unterminaded () =
  let msg = "error unterminated string" in
  test_error_lexing ~msg "unterminated_string"

let test_char_unterminaded () =
  let msg = "error unterminated char" in
  test_error_lexing ~msg "unterminated_char"

let test_char_empty () =
  let msg = "error char is empty" in
  test_error_lexing ~msg "empty_char"

let test_char_too_long () =
  let msg = "error char is too long" in
  test_error_lexing ~msg "long_char"

let tests_cases =
  Alcotest.
    [
      test_case "test-err-lexing-char" `Quick test_error_char;
      test_case "test-err-lexing-comment" `Quick test_comment;
      test_case "test-err-lexing-escapechar" `Quick test_escapedchar;
      test_case "test-err-lexing-string-unterminated" `Quick
        test_string_unterminaded;
      test_case "test-err-lexing-char-unterminated" `Quick
        test_char_unterminaded;
      test_case "test-err-lexing-char-is-empty" `Quick test_char_empty;
      test_case "test-err-lexing-char-too-long" `Quick test_char_too_long;
    ]
