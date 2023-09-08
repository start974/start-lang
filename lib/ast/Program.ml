open Typing
module Error = Error.Ast.Program

type t = Definition.t list
type res = t Error.res

let from_parse_tree pt_prgm =
  let fold_prgm (ty_env, prgm_res) def =
    match Definition.from_parse_tree ~ty_env def with
    | Ok def ->
        let name = Definition.name def and ty = Definition.ty def in
        let ty_env' = TypeEnv.add name ty ty_env in
        let prgm_res' = Result.map (List.cons def) prgm_res in
        (ty_env', prgm_res')
    | Error e1 ->
        let prgm_err =
          match prgm_res with Ok _ -> e1 | Error e2 -> Error.append e1 e2
        in
        (ty_env, Result.error prgm_err)
  in
  List.fold_left fold_prgm (St_Stdlib.Std_lib.env, Result.ok []) pt_prgm
  |> snd |> Result.map List.rev

let pp_print ?(show_type = false) =
  Format.pp_print_list
    ~pp_sep:(fun fmt () -> Format.pp_print_string fmt "\n")
    (fun fmt def ->
      Format.fprintf fmt "%a\n" (Definition.pp_print ~show_type) def)
