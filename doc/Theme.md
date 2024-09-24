# Theme file

When **debug flags** or **repl** display program a theme is used.
By default is use [theme](../asserts/theme.yml).
To change this theme use flag `--theme <theme.yml>`

## Width
You can set with with key `width` and width need to be **positive**

## Colors
You can set color (`color`) using default color string:
```
color :=
  | "black"
  | "red"
  | "green"
  | "yellow"
  | "blue"
  | "magenta"
  | "purple"
  | "cyan"
  | "white"
  | "bright black"
  | "bright red"
  | "bright green"
  | "bright yellow"
  | "bright blue"
  | "bright magenta"
  | "bright cyan"
  | "bright white"
  | color_rgb
```

Or using a rgb color (`color_rgb`) with a list of **3 numbers between 0 and 255**.

## Styles
Style information (`style`) is in list of string:
```
style :=
  | "clear"
  | "bold"
  | "dimmed"
  | "italic"
  | "underline"
  | "blink"
  | "hidden"
  | "strikethrough"
```

## Style Information
Style information (`style_info`) is a combination of `color` and `style`.

```
style_info :=
    | "fg" : color
    | "bg" : color
    | "styles" : [ style* ]
```

## Theme
Theme can set somme expression or keyword of language:
- **keyword** keyword of languae
    - `def`
    - `type`
- **operator** operator of language
    - `:=`
    - `:`
- **def_var** variable in definition
- **expr_var** variable in expression
- **ty_var** variable in type
- **number** numbers in expression

## Example
This is an example of correct file
```
width : 80

keyword:
  fg: "magenta"

operator:
  fg: [255, 0, 0]

def_var:
  fg: "blue"
  styles:
    - "bold"
    - "underline"

expr_var:
  fg:
    - 0
    - 0
    - 255

ty_var:
  fg: "yellow"
  styles:
    - "italic"

number:
  fg: "green"
```
