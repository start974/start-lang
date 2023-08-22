open InputUtils

module type LocationErr = sig
  type t

  val message : t -> string
  (** Message to display with error *)

  val location : t -> Location.t
  (** Location of error *)

  val hint : t -> string option
  (** Hint message *)

  val err_cathegory : ErrorCat.t
  (** Cathegory of error *)
end

module type S = sig
  include LocationErr

  exception Err of t

  val pp_print : Format.formatter -> t -> unit
  (** pretty printing of error *)

  val pp_color_cat :
    (Format.formatter -> 'a -> unit) -> Format.formatter -> 'a -> unit
  (** pretty print with error_cathegory color *)
end

module Make (LE : LocationErr) : S with type t = LE.t = struct
  include LE

  exception Err of t

  let pp_color_cat f fmt x = ErrorCat.pp_color err_cathegory fmt f x
  let pp_location fmt err = Location.pp_print fmt (location err)

  let pp_lines fmt err =
    let location = location err in
    Location.file location
    |> Option.iter @@ fun fname ->
       let l_start, l_end = Location.lines location in
       let size_num = Utils.max_digit_size l_start l_end in
       let input = Inputs.from_string fname in
       let lines = Inputs.get_lines input l_start l_end in
       List.fold_left
         (fun i line ->
           let padding = String.make (size_num - Utils.digit_size i) ' ' in
           Format.fprintf fmt "@{<blue>%s%d |@} %s\n" padding i line;
           succ i)
         l_start lines
       |> ignore;
       if l_start = l_end then
         let c_start, c_end = Location.chars location in
         let offset = size_num + 2 in
         let line_size = c_end + offset in
         let start_i = c_start + offset in
         let line_indication =
           String.init line_size (fun i -> if start_i > i then ' ' else '^')
         in
         Format.fprintf fmt "@{<bold>%a@}"
           (pp_color_cat Format.pp_print_string)
           line_indication

  let pp_message fmt err =
    Format.fprintf fmt "@{<bold>%a: %s@}" ErrorCat.pp_print err_cathegory
      (message err)

  let pp_hint fmt err =
    hint err
    |> Option.iter @@ fun hint ->
       Format.fprintf fmt "@{<aqua>%s@}: %s" "Hint" hint

  let pp_print fmt err =
    Format.fprintf fmt "@{<yellow>%a@}\n%a\n%a%a\n" pp_location err pp_lines err
      pp_message err pp_hint err
end
