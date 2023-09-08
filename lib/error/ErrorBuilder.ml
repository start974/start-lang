module type E = sig
  type t

  val pp_print : Format.formatter -> t -> unit
  (** pretty print error *)
end

module type S = sig
  include E

  type 'a res = ('a, t) result

  exception Err of t

  val error : t -> 'a res
  (** result error of error *)

  val fail : t -> 'a
  (** fail with error *)
end

module Make (E : E) : S with type t = E.t = struct
  include E

  type 'a res = ('a, t) result

  exception Err of t

  let error = Result.error
  let fail x = raise (Err x)
end
