open Typing

let add_type = function
  | Type.T_Var x as t -> Env.add x (Type.ty t)
  | _ -> assert false

let env =
  Env.empty
  |> (* bool *)
  add_type St_Bool.ty
  |> (* char *)
  add_type St_Char.ty
  |> (* char *)
  add_type St_Int.ty
  |> (* string *)
  add_type St_String.ty
  |> (* unit *)
  add_type St_Unit.ty
