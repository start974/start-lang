#!/bin/sh

cd tree-sitter-start/ &&
tree-sitter generate &&
cd .. &&
cargo build
