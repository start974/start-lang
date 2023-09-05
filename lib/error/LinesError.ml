module type LinesErr = sig
  type t

  val pp_header : Format.formatter -> t -> unit
  (** pretty header *)

  val pp_lines : Format.formatter -> t -> unit
  (** pretty lines *)

  val message : t -> string
  (** Error message to display with error *)

  val hint : t -> string option
  (** Hint message *)

  val err_cathegory : ErrorCat.t
  (** Cathegory of error *)
end

module type S = sig
  include LinesErr

  exception Err of t

  val fail : t -> 'a

  val pp_message : Format.formatter -> t -> unit
  (** pretty print of message *)

  val pp_hint : Format.formatter -> t -> unit
  (** pretty print of hint *)

  val pp_print : Format.formatter -> t -> unit
  (** pretty print of error *)
end

module Make (LE : LinesErr) : S with type t = LE.t = struct
  include LE

  exception Err of t

  let fail x = raise (Err x)

  let pp_message fmt err =
    Format.fprintf fmt "@{<bold>%a: %s@}" ErrorCat.pp_print err_cathegory
      (message err)

  let pp_hint fmt err =
    hint err
    |> Option.iter @@ fun hint ->
       Format.fprintf fmt "@{<aqua>%s@}: " "Hint";
       match String.split_on_char '\n' hint with
       | [] -> assert false
       | hint :: rest ->
           Format.fprintf fmt "@{<bold>%s@}" hint;
           let space = "      " in
           List.iter (fun line -> Format.fprintf fmt "\n%s%s" space line) rest;
           Format.pp_print_string fmt "\n"

  let pp_print fmt err =
    Format.fprintf fmt "@{<yellow>%a@}\n%a\n%a\n%a" pp_header err pp_lines err
      pp_message err pp_hint err
end
