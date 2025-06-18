Set DebugTyper.
Set DebugParser.

Def a : N := 1.

Def b : B := true.

Def c : N := b.

Def c' : B := (a : B).

Type N1 := N.

Def d : N1 := c.

