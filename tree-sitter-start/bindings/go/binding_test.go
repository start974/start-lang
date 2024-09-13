package tree_sitter_start_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	tree_sitter_start "github.com/tree-sitter/tree-sitter-start"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_start.StartLanguage())
	if language == nil {
		t.Errorf("Error loading Start grammar")
	}
}
