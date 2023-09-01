open InputUtils
include LocationError.S

val fail_syntax : Location.t -> 'a
val fail_definition_name : Location.t -> 'a
