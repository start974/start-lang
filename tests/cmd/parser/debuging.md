# Use debuging option

```
$ startlang run debuging.st
Type N1 := ℕ.
Type LongType_NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN :=
  N1.
Definition long_number : N1 :=
  333_333_333_333_333_333_333_333_333_333_333_333_333_333_333_333_333_333_331.
Definition n :=
  3 : ℕ.
Definition n :=
  3 : ℕ.
Definition n :=
  3 : ℕ.
Definition c
    : LongType_NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN :=
  long_number.
Eval long_number.
333_333_333_333_333_333_333_333_333_333_333_333_333_333_333_333_333_333_331
TypeOf n.
ℕ
Definition b :=
  true.
Definition c :=
  '//'.
Set DebugTyper.
Unset DebugParser.

```
