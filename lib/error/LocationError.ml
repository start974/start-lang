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

  val fail : t -> 'a

  val pp_message : Format.formatter -> t -> unit
  (** pretty print of message *)

  val pp_hint : Format.formatter -> t -> unit
  (** pretty print of hint *)

  val pp_print : Format.formatter -> t -> unit
  (** pretty print of error *)
end

module Make (LE : LocationErr) : S with type t = LE.t = struct
  include LE

  exception Err of t

  let fail x = raise (Err x)
  let pp_header fmt err = Location.pp_print fmt (location err)
  let pp_color_cat f fmt x = ErrorCat.pp_color err_cathegory fmt f x

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
         let line_size = line_size + if c_start = c_end then 1 else 0 in
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
