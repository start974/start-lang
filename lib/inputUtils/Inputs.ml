type id_string = int
type input = I_String of id_string | I_File of string | I_Std_in

let to_string = function
  | I_String id -> Format.sprintf "<string-%id>" id
  | I_File fname -> fname
  | I_Std_in -> "<stdin>"

let from_string = function
  | "<stdin>" -> I_Std_in
  | str
    when String.starts_with ~prefix:"<string-" str
         && String.ends_with ~suffix:">" str ->
      let len = String.length str - 9 (* 9 is len of "<string->" *)
      and offset = 8 in
      let id = int_of_string (String.sub str offset len) in
      I_String id
  | fname -> I_File fname

module Input = struct
  type t = input

  let equal i1 i2 =
    match (i1, i2) with
    | I_String id1, I_String id2 -> Int.equal id1 id2
    | I_File f1, I_File f2 -> String.equal f1 f2
    | I_Std_in, I_Std_in -> true
    | _ -> false

  let hash input = String.hash (to_string input)
end

module InputTable = Hashtbl.Make (Input)

let table = InputTable.create 2

let register_string =
  let id_string_count = ref 0 in
  fun text ->
    let id = !id_string_count in
    let input = I_String id in
    let lines = String.split_on_char '\n' text |> Array.of_list in
    InputTable.add table input lines;
    incr id_string_count;
    input

let register_files file_name =
  let input = I_File file_name and channel = open_in file_name in
  let rec get_lines lines =
    match In_channel.input_line channel with
    | None -> List.rev lines |> Array.of_list
    | Some l -> get_lines (l :: lines)
  in
  let lines = get_lines [] in
  InputTable.add table input lines;
  input

let std_in = I_Std_in
let std_in_arr_len = ref 0
let std_in_extend_size = 20
let remove_input input = InputTable.remove table input

let get_std_in_array () =
  match InputTable.find table I_Std_in with
  | arr -> arr
  | exception Not_found -> Array.make std_in_extend_size ""

let get_lines_arr = function
  | I_Std_in -> get_std_in_array ()
  | input -> InputTable.find table input

let std_in_get_line () =
  let line = input_line stdin in
  let lines = get_std_in_array () in
  let i = !std_in_arr_len in
  incr std_in_arr_len;
  let size = Array.length lines in
  let lines =
    if !std_in_arr_len > size then
      let new_size = size + std_in_extend_size in
      Array.init new_size (Array.get lines)
    else lines
  in
  lines.(i) <- line;
  line

let get_last_line = function
  | I_Std_in -> std_in_get_line ()
  | input ->
      let lines = get_lines_arr input in
      lines.(Array.length lines - 1)

let get_all input =
  get_lines_arr input
  |> Array.fold_left
       (fun text line -> if text == "" then line else text ^ "\n" ^ line)
       ""

let get_line input line =
  let lines_arr = get_lines_arr input in
  assert (line <= Array.length lines_arr);
  lines_arr.(line - 1)

let get_lines input l_start l_end =
  assert (l_start <= l_end);
  let lines_arr = get_lines_arr input in
  assert (l_end <= Array.length lines_arr);
  let rec get_lines_aux lines i =
    if l_start - 1 <= i && i < l_end then
      get_lines_aux (lines_arr.(i) :: lines) (pred i)
    else lines
  in
  get_lines_aux [] (l_end - 1)

let clean () = InputTable.clear table
