# Visibility

No interface files: visibility is written with markers on declarations and
on `use` (re-exports).

| Marker              | Visible from                                 |
|---------------------|----------------------------------------------|
| (none)              | local: the module itself only                |
| `pub(super)`        | the parent module (and its descendants)      |
| `pub(super.super)`  | the grandparent module (and its descendants) |
| `pub(path)`         | the module or function at `path`             |
| `pub(path1, path2)` | every listed path                            |
| `pub(lib)`          | the whole library (project), not exported    |
| `pub(*)`            | everywhere, including other projects         |
| `pub`               | sugar for `pub(lib)`                         |

- A path is the same as in a `use`; `pub(path)` always includes the
  descendants of `path`. The module defining the name always sees it.
- The structure exposed to other projects is the file layout; a library
  exposes its `pub(*)` names (directly or re-exported).

```
(* src/A.scod *)
def helper := …                            (* local to A *)
pub(super) def g : ℕ := …                  (* parent of A *)
pub(lib.C, lib.D.f) def h : ℕ := …         (* module C and function D.f *)
pub def fast_add : ℕ -> ℕ -> ℕ := …        (* whole library *)
pub(*) def add : ℕ -> ℕ -> ℕ := …          (* exported *)

pub(*) use std.nat.mul                     (* re-export: A.mul *)
pub(lib) use self.B.g as g2
```

## Explicit types

Everything with a `pub` marker must be fully typed with an explicit
annotation on its definition; no inference across a module boundary.

```
pub def f : ℕ -> ℕ := x => x + 1     (* ok *)
pub def g := 2                       (* error: missing annotation *)
def h := 3                           (* local, no annotation needed *)
```
