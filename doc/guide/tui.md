# Interactive session (TUI)

[← Home](../Home.md)

> Describes v0.2.0-beta.2. The design is in [design/tooling](../design/tooling.md#repl).

```sh
startlang tui
```

`startlang tui` opens a full-screen session in the terminal: you type an input
at the bottom, run it, and see under each command what it gave. The inputs run
one after the other: a definition of an input is known in the next ones.

```
┌ input 1 ───────────────────────────────────────┐
│   1 │ Type N := Nat.                           │
│   2 │ Def a : N := 3.                          │
│       ┌──────────────────────────────────────┐ │
│       │a : N                                 │ │
│       └──────────────────────────────────────┘ │
│   3 │ Eval a.                                  │
│       ┌──────────────────────────────────────┐ │
│       │3 : N                                 │ │
│       └──────────────────────────────────────┘ │
└────────────────────────────────────────────────┘
┌ input 2 ───────────────────────────────────────┐
│   1 │ Eval   c.                                │
│       ┌──────────────────────────────────────┐ │
│       │[301] Error: Variable not found.      │ │
│       │   ╭─[ input 2:1:8 ]                  │ │
│       │ …                                    │ │
│       └──────────────────────────────────────┘ │
└────────────────────────────────────────────────┘
┌ input ─────────────────────────────────────────┐
│   1 │ Help a.                                  │
└────────────────────────────────────────────────┘
Enter run · Shift+Enter / Alt+Enter new line · ↑ ↓ history · …
```

## What an input gives

Each input is shown in a frame named after it (`input 1`, `input 2`, …):

- **Without errors**: each command, formatted, and under it its summary in a
  green frame:

  | Command           | Summary                         |
  |-------------------|---------------------------------|
  | `Def a : N := 3.` | `a : N`                         |
  | `Eval a.`         | `3 : N`                         |
  | `TypeOf a.`       | `N`                             |
  | `Help a.`         | `a : N` and its documentation   |
  | `Type N := Nat.`  | none                            |

- **With errors**: the input as you typed it, then the errors in a red frame;
  they point inside the input (`input 2:1:8`: input 2, line 1, column 8). An
  input with errors is forgotten: none of its definitions is kept.

The newest input is at the bottom; the older ones scroll up.

## Keys

The keys are those of a classic shell (readline).

| Key                         | Action                                     |
|-----------------------------|--------------------------------------------|
| Enter                       | run the whole input                        |
| Shift+Enter, Alt+Enter      | new line                                   |
| ↑ / Ctrl+P                  | previous input (↑ on the first line)       |
| ↓ / Ctrl+N                  | next input (↓ on the last line)            |
| Ctrl+A / Ctrl+E             | start / end of the line                    |
| Ctrl+U / Ctrl+K             | delete to the start / end of the line      |
| Ctrl+W                      | delete the word before                     |
| Alt+← / Alt+→               | word back / forward (also Alt+B / Alt+F)   |
| Ctrl+Y                      | paste what was deleted                     |
| Ctrl+C                      | clear the input; quit on an empty one      |
| Ctrl+D                      | quit on an empty input                     |
| Ctrl+L                      | clear the outputs                          |
| PageUp / PageDown, wheel    | scroll the outputs                         |
| F1, Alt+H                   | the help: the keys (the same key or Esc closes it) |

## History

The inputs you run are kept, oldest first, and saved in `.start-history.txt`
in the working directory when you quit; the next session starts with them.

- ↑ on the first line of the input (or Ctrl+P anywhere) brings back the
  previous input, whole, with all its lines; ↓ on the last line (or Ctrl+N)
  the next one. Past the newest, the input you were typing comes back.
- A recalled input is a copy: edit it and run it, it is added to the history,
  the old one stays as it was.
- The same input run twice in a row is kept once.

## Pasting and scrolling

- Pasted text goes into the input with its line breaks and does not run:
  check it, then press Enter.
- PageUp / PageDown scroll the outputs by half their height, the mouse wheel
  by 3 lines; running an input (or Ctrl+L) brings the view back to the
  newest.

## Terminals

- **Shift+Enter** is only seen by terminals that report it (the kitty keyboard
  protocol: kitty, WezTerm, foot, Ghostty, Alacritty, recent iTerm2, Windows).
  In the others (GNOME Terminal, Konsole, xterm), Shift+Enter runs the input:
  use **Alt+Enter** for a new line.
- **F1** is kept by some terminals (GNOME Terminal opens its own help): use
  **Alt+H**.
- The TUI uses the mouse (for the wheel): to select text in the terminal,
  hold **Shift** while dragging.
- `startlang tui` needs a terminal: with its input or output redirected, it
  stops with error 103.

Not done yet: completion (Tab) and search in the history (Ctrl+R), see the
[design](../design/tooling.md#repl).
