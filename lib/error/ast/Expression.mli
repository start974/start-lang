open InputUtils
module ErrorVar : ErrorBuilder.S

val error_unknown_var : Location.t -> string -> 'a ErrorVar.res
