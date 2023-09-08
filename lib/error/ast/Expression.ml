module ErrorVar = Unknown.Make (struct
  let kind = Unknown.Var
end)

let error_unknown_var loc x = ErrorVar.error (ErrorVar.make loc x)
