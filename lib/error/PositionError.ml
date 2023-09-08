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
  include ErrorBuilder.S

  val message : t -> string
  (** Message to display with error *)

  val position : t -> Position.t
  (** Position of error *)

  val hint : t -> string option
  (** Hint message *)

  val err_cathegory : Cathegory.t
  (** Cathegory of error *)
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
