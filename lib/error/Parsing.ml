open InputUtils

type u = { location : Location.t; hint : string option }

include LocationError.Make (struct
  type t = u

  let message _ = "Syntax error."
  let location { location; _ } = location
  let hint { hint; _ } = hint
  let err_cathegory = ErrorCat.Error
end)

let fail_hint location hint = fail { location; hint = Some hint }
