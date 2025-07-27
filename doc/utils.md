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
