(* Comment before *)
(* other comment *)

(* comment with lines *)
(** doc *)
Definition (* comment1 *) a (* comment2 *) : (* comment 3 *) ℕ (* comment 4 *) := (* comment 5 *) 3 (* comment 6 *).
Eval (* Eval var a *) a (* Eval last comment*).
Def b : Nat :=
  (* many comment

  bla bla bla
  bla bla bla blaaa

  end of many comment *)




  a.

(** doc type *)
Type (* comment 1 *) N1 (* comment 2 *) :=


(* comment 3 *) Nat (* comment 4*).


TypeOf (* type of a *) a.

   Set (* comment set *)    DebugTyper.
   Unset (* comment unset *) DebugTyper        .
