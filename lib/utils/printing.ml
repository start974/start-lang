open Format

let debug_level = ref (-1)
let verbose_active = ref false

let debug ?(level = 1) fmt =
  if !debug_level >= level then fprintf err_formatter fmt
  else ifprintf err_formatter fmt

let verbose format =
  if !verbose_active then kasprintf (printf "@{<grey>%s@}") format
  else ifprintf std_formatter format

let head_1 fmt head =
  let line = "**************************************" in
  fprintf fmt "@.%s@." line;
  fprintf fmt "@[<v5>   %s@]" head;
  fprintf fmt "@.%s@." line

let head_2 fmt head =
  fprintf fmt "@.@[<v5>   %s@]@." head;
  fprintf fmt "====================@."
