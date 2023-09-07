open InputUtils
open Typing

type t = { loc : Location.t; ty : Type.t; name : Ident.t; body : Expression.t }

let make ?(loc = Location.unknown) name body =
  let ty = Expression.ty body in
  { loc; ty; name; body }

let name { name; _ } = name
let ty { ty; _ } = ty
let body { body; _ } = body

let from_parse_tree ~ty_env
    ({ location = loc; node = { name; body; _ } } : Frontend.ParseTree.def_loc)
    =
  let body' = Expression.from_parse_tree ~ty_env body in
  make ~loc name body'

let pp_print ?(show_type = false) fmt ({ name; body; ty; _ } : t) =
    if show_type then Format.fprintf fmt "@[%s : %a.@]@." name Type.pp_print ty;
  Format.fprintf fmt "@[%s := %a.@]" name Expression.pp_print body
(*Format.fprintf fmt "@[%s@ @[%a@] := @[%a@].@]" name Pattern.pp_print*)
(*Expression.pp_print body*)
