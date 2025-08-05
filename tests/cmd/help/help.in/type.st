Type N0 := Nat.
Help N0.

(** document of a N*)
Type N1 := N0.
Help N1.

(** document of a N with many lines:
  lines 1
  lines 2
*)
Type N2 := Nat.
? N2.


(* not a documentation *)
Type N3 := N1.
? N3.
