open InputUtils
module Error = Error.Frontend.Parse

type 'a res = 'a Error.res

val program : Inputs.input -> ParseTree.program res
