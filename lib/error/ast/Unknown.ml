open InputUtils

type kind = Type | Var

module type S = sig
  include LocationError.S

  val make : Location.t -> string -> t
end

module Make (K : sig
  val kind : kind
end) =
struct
  type u = { location : Location.t; var : string }

  include LocationError.Make (struct
    type t = u

    let kind_str = match K.kind with Var -> "Variable" | Type -> "Type"

    let message { var; _ } =
      Format.sprintf "%s \"%s\" is not define." kind_str var

    let location { location; _ } = location
    let hint _ = None
    let err_cathegory = Cathegory.Error
  end)

  let make location var = { location; var }
end
