# Modules and imports

## Layout

Module `A` is made of the files `A.scod`, `A.sprp`, … (one per kind); its
sub-modules live in the directory `A/` next to them.

```
src/A.scod          (* names of A: A.f *)
src/A.sprp
src/A/B.scod        (* names of A.B: A.B.g *)
src/A/B/C.sprf      (* A.B.C *)
```

- A directory without sibling files is a module with only sub-modules.
- A name defined in `A.*` cannot be the same as a sub-module of `A/`.

## `use`

A single `use` command (Rust `use`-style): it brings the last segment of the
path into scope.

```
use std.nat                         (* nat.add *)
use std.nat.add                     (* add *)
use std.nat.*                       (* every public name *)
use std.nat.(add, mul as times)     (* group, renaming *)
use std.(bool.*, nat.(add, mul))    (* nested groups *)
use std.nat as N                    (* N.add *)
use code std.nat.add                (* explicit kind *)
use prop std.nat.add_comm
```

- `use A.*` imports both the names of `A` and its sub-modules.
- **Kind**: optional keyword right after `use` (`synt`, `code`, `prop`),
  only required when several kinds share the path; shared by the whole
  group.
- **Local open**: `path.( expr )` puts the names of `path` in scope inside
  the parentheses only.
- **Conflicts**: importing a name already in scope is an immediate error;
  `as` solves it. Explicit imports shadow glob names; two globs bringing the
  same name only conflict if that name is used.
- **Position**: a `use` can appear anywhere and applies to the whole file;
  the formatter moves every `use` to the top.

## Path roots

```
(* in src/A/B.scod *)
use lib.C.f              (* lib = project root, src/C.scod *)
use super.g              (* parent module, src/A.scod *)
use self.D               (* current module *)
use std.nat.add          (* otherwise: a dependency *)
```

## Cycles

Import cycles between files are allowed: the files of a cycle (strongly
connected component) are checked together.

## `projet.toml`

```toml
[project]
name = "mylib"
src = "src"
prelude = "lib.prelude"     # default "std.prelude", false to disable

[dependencies]
std = { git = "https://github.com/start974/startlang-std", tag = "v0.3" }
mylib = { path = "../mylib" }

[extensions]                # optional overrides
code = "stc"

[[bin]]
name = "app"
main = "lib.app"
```

- Dependencies come from a local path or a git repository; a package
  registry may come later.
- **Prelude**: a module implicitly imported in every file (`use <prelude>.*`);
  `std.prelude` by default, overridable or disabled per project.

## Standard library

`std` is a normal dependency (added by `startlang new`): one module per type
and a prelude made of re-exports.

```
std/
  projet.toml
  src/
    int.scod       (* i8 … i64, u8 … u64 *)
    bool.scod      (* 𝔹, Bool, true, false *)
    char.scod      (* Char *)
    prelude.scod   (* pub(*) use lib.(int.*, bool.*, char.*) *)
```

## Open questions

- [ ] Where does `std` live: separate repository or a path in this one?
- [ ] Incremental compilation / cache of already-checked files?
