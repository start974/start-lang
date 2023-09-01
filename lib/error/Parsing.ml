open InputUtils

type msg_type = Basic_Error | DefinitionName
type u = { location : Location.t; msg_type : msg_type }

include LocationError.Make (struct
  type t = u

  let message _ = "Syntax error."
  let location { location; _ } = location

  let hint { msg_type; _ } =
    match msg_type with
    | Basic_Error -> None
    | DefinitionName -> Some ", this definition has not name."

  let err_cathegory = ErrorCat.Error
end)

let fail_syntax location = fail { location; msg_type = Basic_Error }
let fail_definition_name location = fail { location; msg_type = DefinitionName }
