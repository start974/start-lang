open InputUtils
open Typing

type t = { loc : Location.t; ty : Type.t; name : Ident.t; body : Expression.t }

val make : ?loc:Location.t -> Ident.t -> Expression.t -> t
(* make a definition *)

val name : t -> Ident.t
(* name of definition *)

val ty : t -> Type.t
(* type of definition *)

val body: t -> Expression.t
(* body of definition *)

val from_parse_tree : ty_env:Type.env -> Frontend.ParseTree.def_loc -> t
(** get program from parse tree *)

val pp_print : ?show_type:bool -> Format.formatter -> t -> unit
(** pretty print definition *)
