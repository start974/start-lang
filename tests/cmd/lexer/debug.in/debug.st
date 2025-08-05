Set DebugLexer.

Def a := 3.

Type N1 := Nat.
(** definition of is Nat *)
Definition b : Nat := ((a : Nat) : N1).
? b.
?: b.
$ b.

Definition c := 'c'.
$ c.
