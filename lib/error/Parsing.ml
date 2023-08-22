open InputUtils

include LocationError.Make (struct
  type t = Location.t

  let message _ = "Syntax error."
  let location l = l
  let hint _ = None
  let err_cathegory = ErrorCat.Error
end)

let fail loc = raise (Err loc)
