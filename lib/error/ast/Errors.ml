type err = E_Var of Expression.ErrorVar.t

module E : ErrorBuilder.S with type t = err = ErrorBuilder.Make (struct
  type t = err

  let pp_print fmt = function E_Var e -> Expression.ErrorVar.pp_print fmt e
end)

include ErrorStack.Make (E)

let e_var e = E_Var e
