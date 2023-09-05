type input

val to_string : input -> string
(** input to string *)

val from_string : string -> input
(** input get by string *)

val from_position : Position.t -> input option
(** input get by Position *)

val from_location : Location.t -> input option
(** input get by Location *)

val register_string : string -> input
(** register a string *)

val register_file : string -> input
(** register a file *)

val std_in : input
(** get stdin *)

val remove_input : input -> unit
(** remove input in table *)

val get_last_line : input -> string
(** get last line of input *)

val get_all : input -> string
(** get all text of input *)

val get_line : input -> int -> string
(** [get_line input l] get line [l] of input *)

val get_lines : input -> int -> int -> string list
(** [get_lines input l_start l_end] get list of lines beetween [l_start] and [l_end]*)

val extract : Location.t -> string
(** [extract location] extract text at location *)

val clean : unit -> unit
(** clean the table *)
