open InputUtils
open Typing

type expr = E_Const of Constant.t | E_Type of Type.t | E_Var of Ident.t
(*| E_App of t * t*)
(*| E_Abs of Pattern.t * t*)
(*| E_Prod of t list*)
(*| E_Type of t*)

and t = { loc : Location.t; expr : expr; ty : Type.t }

val e_const : ?loc:Location.t -> Constant.t -> t
(** [e_const c] make a constant [c] *)

val e_type : ?loc:Location.t -> Type.t -> t
(** [e_type t] make a type expresion of type [t] *)

val e_var : ?loc:Location.t -> ty_env:Type.env -> Ident.t -> t
(** [e_var ~ty_env x] make a variable [x] and get type with type environement *)
(*val e_app : ?loc:Location.t -> ty_env:ty_env -> t -> t -> t*)
(*(** [e_app ?loc e1 e2] make application of [e1] and [e2] *)*)

(*val e_abs : ?loc:Location.t -> ty_env:ty_env -> Pattern.t -> t -> t*)
(*(** [e_abs p e] make abstraction with pattern and expression *)*)

(*val e_prod : ?loc:Location.t -> ty_env:ty_env -> t list -> t*)
(*(** [e_abs p e] make abstraction with pattern and expression *)*)

val from_parse_tree : ty_env:Type.env -> Frontend.ParseTree.expr_loc -> t
(** get expressing from parse tree *)

val ty : t -> Type.t
(** [get_ty e] get type of [e] *)

val expr : t -> expr
(** [get_expr e] get expression of [e] *)

val location : t -> Location.t
(** [get_location e] get location of [e] *)

val pp_print : Format.formatter -> t -> unit
(** pretty print with type expressions typed by default is false *)
