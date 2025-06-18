# Use debuging option

```
$ startlang debuging.st
Type N1 := ℕ.
Type LongType_NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN :=
  N1.
Definition long_number : N1 :=
  333_333_333_333_333_333_333_333_333_333_333_333_333_333_333_333_333_333_331.
Definition b :=
  3 : N.
Definition b :=
  3 : N.
Definition b :=
  3 : N.
Definition c
    : LongType_NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN :=
  long_number.
Eval long_number.
333_333_333_333_333_333_333_333_333_333_333_333_333_333_333_333_333_333_331
TypeOf b.
N
Set DebugTyper.
Set DebugParser.

```
