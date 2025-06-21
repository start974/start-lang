Set DebugParser.

Type N1 := ℕ.

Type LongType_NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN :=
  N1.

Def long_number
    : N1 :=
  333_333_333_333_333_333_333_333_333_333_333_333_333_333_333_333_333_333_331.

Def b := (3 : ℕ).
Def b := 3 : ℕ.
Def b := ((3 : ℕ)).

Def c
  : LongType_NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN :=
  long_number.

$ long_number.

?: b.

Set DebugTyper.
Unset DebugParser.
