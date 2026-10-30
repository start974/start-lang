# Changelog

[← Home](../Home.md)

One page per **released version**, newest first. Each page has one section
per pre-release (`beta.N`, `rc.N`), each listing what was **added**,
**changed**, **removed** and **fixed**; the **known issues** at release close
the page. The plan for future versions is in the [roadmap](../roadmap/roadmap.md),
and the numbering in [versioning](../roadmap/roadmap.md#versioning).

| Version | Date | Summary |
|---------|------|---------|
| [v0.2.0](./v0.2.0.md) | *unreleased* | Cleanup and preparation: compilation phases, new syntax, `.scod` / `.snbx`, machine integers, values, Cranelift, tooling |
| [v0.1.0](./v0.1.0.md) | 2026-01-11 | First version: single-file language, interpreter, REPL, formatter, LSP diagnostics |

## Writing a changelog page

Create `vX.Y.0.md` with the first beta of a version, and add a row above:

```markdown
# vX.Y.0 — Title

[← Changelog](./README.md) · [Roadmap](../roadmap/roadmap.md)

Status: **unreleased** | Released: YYYY-MM-DD

| Pre-release | Date | Summary |
|-------------|------|---------|

## vX.Y.0-beta.1 — Title
### Added
### Changed
### Removed
### Fixed

## vX.Y.0-rc.1 — Release candidate
### Fixed

## Known issues
```

A patch release (`vX.Y.1`, fixes only) gets a section at the top of the page
of its version.
