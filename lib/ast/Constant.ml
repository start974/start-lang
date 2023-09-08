open St_Stdlib

type t =
  | C_Unit
  | C_Bool of St_Bool.t
  | C_Int of St_Int.t
  | C_Char of St_Char.t
  | C_String of St_String.t

let c_unit = C_Unit
let c_bool b = C_Bool b
let c_int i = C_Int i
let c_char c = C_Char c
let c_string s = C_String s

let from_parse_tree =
  let open Frontend.ParseTree in
  function
  | E_Unit -> c_unit
  | E_Bool b -> c_bool b
  | E_Int i -> c_int i
  | E_Char c -> c_char c
  | E_String s -> c_string s
  | _ -> assert false

let ty = function
  | C_Unit -> St_Unit.ty
  | C_Bool _ -> St_Bool.ty
  | C_Int _ -> St_Int.ty
  | C_Char _ -> St_Char.ty
  | C_String _ -> St_String.ty

let pp_print fmt = function
  | C_Unit -> St_Unit.pp_print fmt ()
  | C_Bool b -> St_Bool.pp_print fmt b
  | C_Int i -> St_Int.pp_print fmt i
  | C_Char c -> St_Char.pp_print fmt c
  | C_String s -> St_String.pp_print fmt s

let equal c1 c2 =
  match (c1, c2) with
  | C_Unit, C_Unit -> true
  | C_Bool b1, C_Bool b2 -> b1 = b2
  | C_Int i1, C_Int i2 -> i1 = i2
  | C_Char c1, C_Char c2 -> c1 = c2
  | C_String s1, C_String s2 -> s1 = s2
  | _ -> false
