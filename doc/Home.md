# startlang wiki

startlang is a functional language with a proof assistant, built step by step.
This wiki is the documentation of the project: how to use it, how the language
is defined, where it is going and what changed.

Current version: **v0.2.0-beta.2** — see the [changelog](./changelog/README.md).

## Guide

How to use startlang today.

| Page | Content |
|------|---------|
| [Getting started](./guide/getting-started.md) | Install, first file, first interactive session |
| [Interactive session (TUI)](./guide/tui.md) | `startlang tui`: what an input gives, keys, history, scrolling |
| [Language tour](./guide/language-tour.md) | Commands, types, literals, comments and documentation |
| [Command line](./guide/cli.md) | `run`, `tui`, `format`, `lsp`, debug flags |
| [Theme](./guide/theme.md) | Colors used when printing programs |

## Reference

The formal definition of the language.

| Page | Content |
|------|---------|
| [Grammar](./reference/grammar.md) | Lexer and parser, in EBNF |
| [Typing](./reference/typing.md) | Typing rules |
| [Semantics](./reference/semantics.md) | Evaluation rules |

## Project

| Page | Content |
|------|---------|
| [Roadmap](./roadmap/roadmap.md) | Versions to come, one page per version |
| [Design](./design/README.md) | Decisions taken, by topic |
| [Changelog](./changelog/README.md) | What each version delivered, one page per version |
| [Development](./contributing/development.md) | Tests, coverage, lint, workspace layout |

## Conventions

- One topic per page; every page links back to this one.
- The **reference** describes the current version only; future work lives in
  the roadmap and the design pages.
- When a version is released, its changelog page is completed and the guide
  and reference are updated in the same change.
