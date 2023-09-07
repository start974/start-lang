open InputUtils

module type PositionErr = sig
  type t

  val message : t -> string
  (** Message to display with error *)

  val position : t -> Position.t
  (** Position of error *)

  val hint : t -> string option
  (** Hint message *)

  val err_cathegory : Cathegory.t
  (** Cathegory of error *)
end

module type S = sig
  include PositionErr

  exception Err of t

  val fail : t -> 'a

  val pp_message : Format.formatter -> t -> unit
  (** pretty print of message *)

  val pp_hint : Format.formatter -> t -> unit
  (** pretty print of hint *)

  val pp_print : Format.formatter -> t -> unit
  (** pretty print of error *)
end

module Make (PE : PositionErr) : S with type t = PE.t = struct
  include LocationError.Make (struct
    include PE

    let location x =
      let pos = position x in
      (pos, pos)
  end)

  let position = PE.position
end
