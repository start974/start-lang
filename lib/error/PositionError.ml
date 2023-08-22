open InputUtils

module type PositionErr = sig
  type t

  val message : t -> string
  (** Message to display with error *)

  val position : t -> Position.t
  (** Position of error *)

  val hint : t -> string option
  (** Hint message *)

  val err_cathegory : ErrorCat.t
  (** Cathegory of error *)
end

module type S = sig
  include PositionErr

  exception Err of t

  val fail : t -> 'a
  val pp_print : Format.formatter -> t -> unit

  val pp_color_cat :
    (Format.formatter -> 'a -> unit) -> Format.formatter -> 'a -> unit
  (** pretty print with error_cathegory color *)
end

module Make (PE : PositionErr) : S with type t = PE.t = struct
  include PE

  exception Err of t

  let fail x = raise (Err x)
  let pp_color_cat f fmt x = ErrorCat.pp_color err_cathegory fmt f x
  let pp_position fmt err = Position.pp_print fmt (position err)

  let pp_line fmt err =
    let position = position err in
    Position.file position
    |> Option.iter @@ fun fname ->
       let l = Position.line position in
       let size_num = Utils.digit_size l in
       let input = Inputs.from_string fname in
       let line = Inputs.get_line input l in
       Format.fprintf fmt "@{<blue>%d |@} %s\n" l line;
       let c = Position.char position in
       let start_i = c + size_num + 2 in
       let line_size = start_i + 1 in
       let line_indication =
         String.init line_size (fun i -> if start_i > i then ' ' else '^')
       in
       Format.fprintf fmt "@{<bold>%a@}"
         (pp_color_cat Format.pp_print_string)
         line_indication

  let pp_message fmt err =
    Format.fprintf fmt "%a : %s" ErrorCat.pp_print err_cathegory (message err)

  let pp_hint fmt err = Option.iter (Format.pp_print_string fmt) (hint err)

  let pp_print fmt err =
    Format.fprintf fmt "@{<yellow>%a@}\n%a\n%a%a" pp_position err pp_line err
      pp_message err pp_hint err
end
