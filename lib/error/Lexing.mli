open InputUtils
include PositionError.S

val fail_char : Position.t -> char -> 'a
val fail_comment : Position.t -> 'a
