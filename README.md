# codebase

## Automatic Code coverage

https://app.codecov.io/gh/cs-25-sw-4-09/codebase/

## Generate Manual Code Coverage

First some dependencies need to be installed

### Install Dependencies

```sh
rustup component add llvm-tools-preview
```

```sh
cargo install grcov
```

### Delete past coverage

```sh
cargo clean && mkdir -p coverage/ && rm -r coverage/*
```

### Calculate new coverage

```sh
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='coverage/cargo-test-%p-%m.profraw' cargo test
```

### Generate HTML report

```sh
grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/
```

HTML coverage report will now be available in /target/coverage/index.html
