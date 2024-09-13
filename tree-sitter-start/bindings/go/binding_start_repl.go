package tree_sitter_start

// #cgo CFLAGS: -std=c11 -fPIC
// #include "../../grammars/start_repl/src/parser.c"
// // NOTE: if your language has an external scanner, add it here.
import "C"

import "unsafe"

// Get the tree-sitter Language for this grammar.
func StartReplLanguage() unsafe.Pointer {
	return unsafe.Pointer(C.tree_sitter_start())
}
