## Test
run test with *nextest*
```sh
cargo nextest run
```

overwrite trycmd
```sh
TRYCMD=overwrite cargo test --test cli_tests
```

## Coverage
open coverage
```sh
cargo llvm-cov nextest --workspace --all-features --open
```

generate *lcov* file
```sh
cargo llvm-cov nextest --workspace --all-features --lcov --output-path target/lcov.info
```

