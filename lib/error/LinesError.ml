module type LinesErr = sig
  type t

  val pp_header : Format.formatter -> t -> unit
  (** pretty header *)

  val pp_lines : Format.formatter -> t -> unit
  (** pretty lines *)

  val message : t -> string
  (** Message to display with error *)

  val hint : t -> string option
  (** Hint message *)

  val err_cathegory : ErrorCat.t
  (** Cathegory of error *)
end

module type S = sig
  include LinesErr

  exception Err of t

  val fail : t -> 'a

  val pp_message: Format.formatter -> t -> unit
  (** pretty print of message *)

  val pp_hint: Format.formatter -> t -> unit
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
       Format.fprintf fmt "@{<aqua>%s@}: %s" "Hint" hint

  let pp_print fmt err =
    Format.fprintf fmt "@{<yellow>%a@}\n%a\n%a\n%a" pp_header err pp_lines err
      pp_message err pp_hint err
end
