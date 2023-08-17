type t = Definition.t list

let rec equal f1 f2 =
  match (f1, f2) with
  | [], [] -> true
  | d1 :: f1', d2 :: f2' -> Definition.equal d1 d2 && equal f1' f2'
  | _ -> false

let pp_print =
  Format.pp_print_list
    ~pp_sep:(fun fmt () -> Format.fprintf fmt "@.")
    Definition.pp_print
