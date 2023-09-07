open Typing

type t = Definition.t list

let from_parse_tree pt_prgm =
  List.fold_left_map
    (fun ty_env def ->
      let def' = Definition.from_parse_tree ~ty_env def in
      let name = Definition.name def' and ty = Definition.ty def' in
      let ty_env' = TypeEnv.add name ty ty_env in
      (ty_env', def'))
    St_Stdlib.Std_lib.env pt_prgm
  |> snd

let pp_print ?(show_type = false) =
  Format.pp_print_list
    ~pp_sep:(fun fmt () -> Format.pp_print_string fmt "\n")
    (fun fmt def ->
      Format.fprintf fmt "%a\n" (Definition.pp_print ~show_type) def)
