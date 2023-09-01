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

module type S = LinesError.S

module Make (LE : LocationErr) : S with type t = LE.t = LinesError.Make (struct
  include LE

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
         let start_i = c_start + offset in
         let line_indication =
           String.init line_size (fun i -> if start_i > i then ' ' else '^')
         in
         Format.fprintf fmt "@{<bold>%a@}"
           (pp_color_cat Format.pp_print_string)
           line_indication
end)
