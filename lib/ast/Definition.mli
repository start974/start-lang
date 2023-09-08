open InputUtils
open Typing
module Error = Error.Ast.Definition

type t
type res = t Error.res

val make : ?loc:Location.t -> Ident.t -> Expression.t -> t
(* make a definition *)

val name : t -> Ident.t
(* name of definition *)

val ty : t -> Type.t
(* type of definition *)

val body : t -> Expression.t
(* body of definition *)

val loc : t -> Location.t
(* location of definition *)

val from_parse_tree : ty_env:Type.env -> Frontend.ParseTree.def_loc -> res
(** get program from parse tree *)

val pp_print : ?show_type:bool -> Format.formatter -> t -> unit
(** pretty print definition *)
