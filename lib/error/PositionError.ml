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

module type S = LinesError.S

module Make (PE : PositionErr) : S with type t = PE.t = LinesError.Make (struct
  include PE

  let pp_color_cat f fmt x = ErrorCat.pp_color err_cathegory fmt f x
  let pp_header fmt err = Position.pp_print fmt (position err)

  let pp_lines fmt err =
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
end)
