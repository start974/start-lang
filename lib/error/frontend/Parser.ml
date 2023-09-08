open InputUtils

type u = { location : Location.t; hint : string option }

include LocationError.Make (struct
  type t = u

  let message _ = "Syntax error."
  let location { location; _ } = location
  let hint { hint; _ } = hint
  let err_cathegory = Cathegory.Error
end)

let error_hint location hint = error { location; hint = Some hint }
