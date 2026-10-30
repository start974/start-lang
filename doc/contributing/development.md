# Development

[← Home](../Home.md)

## Branches

One branch per pre-release or release, named `v<version>--<name>`:

```
v0.2.0-beta.1--compilation-phases
v0.2.0-beta.2--cleanup
v0.2.0-rc.1--release-candidate
v0.2.1--fix-lsp-hover
```

- `<version>` is the exact version the branch delivers (see
  [versioning](../roadmap/roadmap.md#versioning)); the `--` keeps the branch
  distinct from the git tag `v<version>`.
- `<name>` is short, lowercase, words joined by `-`.
- The task list of the branch is its `TODO.md`, removed before merging into
  `main`.

## Test
run test with *nextest*
```sh
cargo nextest run
```
or use `cargo test`

overwrite trycmd
```sh
TRYCMD=overwrite cargo test --test cli_tests
```

## Coverage
open coverage
```sh
cargo llvm-cov --open
```

generate *lcov* file
```sh
cargo llvm-cov nextest --workspace --all-features --lcov --output-path target/lcov.info
```

## format
use `cargo fmt` to format the code

## Lint
use `cargo clippy` to lint the code

## Install
use `cargo install --path .` to install startlang

## check package outdated
use `cargo oudated`

## upgradde package of project
use `cargo-edit` and run `cargo upgrade`
