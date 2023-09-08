open InputUtils
open Typing
module Error = Error.Ast.Definition

type t = {
  loc : Location.t;
  ty : Type.t;
  (*pattern : Pattern.t;*)
  name : Ident.t;
  body : Expression.t;
}

type res = t Error.res

let make ?(loc = Location.unknown) name (*pattern*) body =
  let ty = Expression.ty body in
  { loc; ty; name; (*pattern;*) body }

let name { name; _ } = name
let ty { ty; _ } = ty
let body { body; _ } = body
let loc { loc; _ } = loc

let from_parse_tree ~ty_env
    ({ location = loc; node = { name; body; _ (*patt*) } } :
      Frontend.ParseTree.def_loc) =
  Expression.from_parse_tree ~ty_env body
  (* to error *)
  |> Result.map_error (fun err_var -> Error.e_var err_var |> Error.singleton)
  (* make result *)
  |> Result.map (make ~loc name)

let pp_print ?(show_type = false) fmt ({ name; body; ty; _ } : t) =
  if show_type then Format.fprintf fmt "@[%s : %a.@]@." name Type.pp_print ty;
  Format.fprintf fmt "@[%s := %a.@]" name Expression.pp_print body
(*Format.fprintf fmt "@[%s@ @[%a@] := @[%a@].@]" name Pattern.pp_print*)
(*Expression.pp_print body*)
