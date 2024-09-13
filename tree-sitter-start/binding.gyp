{
  "targets": [
    {
      "target_name": "tree_sitter_start_binding",
      "dependencies": [
        "<!(node -p \"require('node-addon-api').targets\"):node_addon_api_except",
      ],
      "include_dirs": [
        "grammar/start/src",
      ],
      "sources": [
        "grammars/start/src/parser.c",
        "grammars/start/src/scanner.c",
        "grammars/start_repl/src/parser.c",
        "grammars/start_repl/src/scanner.c",
        "bindings/node/binding.cc",
      ],
      "conditions": [
        ["OS!='win'", {
          "cflags_c": [
            "-std=c11",
          ],
        }, { # OS == "win"
          "cflags_c": [
            "/std:c11",
            "/utf-8",
          ],
        }],
      ],
    }
  ]
}
