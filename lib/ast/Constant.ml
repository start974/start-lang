open InputUtils

type const =
  | C_Unit
  | C_Bool of bool
  | C_Int of Z.t
  | C_Char of char
  | C_String of string

type t = { loc : Location.t; const : const }

let c_unit ?(loc = Location.unknown) () = { loc; const = C_Unit }
let c_bool ?(loc = Location.unknown) b = { loc; const = C_Bool b }
let c_int ?(loc = Location.unknown) i = { loc; const = C_Int i }
let c_char ?(loc = Location.unknown) c = { loc; const = C_Char c }
let c_string ?(loc = Location.unknown) s = { loc; const = C_String s }
let const { const; _ } = const

let ty { const; _ } =
  match const with
  | C_Unit -> Type.t_unit
  | C_Bool _ -> Type.t_bool
  | C_Int _ -> Type.t_int
  | C_Char _ -> Type.t_char
  | C_String _ -> Type.t_string

let loc { loc; _ } = loc

let pp_print fmt { const; _ } =
  match const with
  | C_Unit -> Format.pp_print_string fmt "()"
  | C_Bool b -> Utils.Pretty.pp_const_bool fmt b
  | C_Int i -> Z.pp_print fmt i
  | C_Char c -> Utils.Pretty.pp_const_char fmt c
  | C_String s -> Utils.Pretty.pp_const_string fmt s

let equal c1 c2 =
  match (const c1, const c2) with
  | C_Unit, C_Unit -> true
  | C_Bool b1, C_Bool b2 -> b1 = b2
  | C_Int i1, C_Int i2 -> i1 = i2
  | C_Char c1, C_Char c2 -> c1 = c2
  | C_String s1, C_String s2 -> s1 = s2
  | _ -> false
