let () =
  Options.file
  |> (* get input *)
  Phase.Input.run
  |> (* parse input *)
  Phase.Parsing.run
  |> (* type parsing tree *)
  Phase.Typing.run
  |> (* ... *)
  ignore;
  Phase.exit 0
