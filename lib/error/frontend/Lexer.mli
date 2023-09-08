open InputUtils
include PositionError.S

(* fail token *)
val fail_char : Position.t -> char -> 'a

(* fail comment *)
val fail_comment : Position.t -> 'a

(* char escape *)
val fail_escape_char : Position.t -> char -> 'a

(* fail on string char *)
val fail_readstring_terminate : Position.t -> 'a

(* fail on read char *)
val fail_readchar_terminate : Position.t -> 'a
val fail_readchar_empty : Position.t -> 'a
val fail_readchar_finish : Position.t -> char -> 'a
