module type S = sig
  include ErrorBuilder.S

  type e
  (* type of errors *)

  val singleton : e -> t
  (* make the first error *)

  val push : e -> t -> t
  (* push an error *)

  val append : t -> t -> t
  (* concat to stack of error *)
end

module Make (E : ErrorBuilder.S) : S with type e = E.t = struct
  type e = E.t

  include ErrorBuilder.Make (struct
    type t = e list

    let pp_print =
      Format.pp_print_list
        ~pp_sep:(fun fmt () -> Format.pp_print_string fmt "\n")
        E.pp_print
  end)

  let push = List.cons
  let append = List.append
  let singleton e = [ e ]
end
