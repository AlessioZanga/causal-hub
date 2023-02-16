# Contributing

##  Issues and Pull Requests

### Forking

To contribute to the repository, fork it, solve an issue and open a pull request.

### Bug Report

TODO:

### Feature Request

TODO:

### Pull Request

Before opening a pull request:

0. Add/change/remove/fix code.
1. Add documentation and tests for the modified code.
2. Execute tests and check the code coverage. If coverage is unsatisfactory, repeat from step 1.
3. Execute linting and formatting. If linting and/or formatting change the code, repeat from step 1. 
4. Open a pull request.

After opening a pull request:

0. Wait for pull request review.
1. If reviewer raises issues, try to address the raised issues and repeat from step 0.
2. The pull request has been merged, celebrate 🎉

## Building and Testing

### Building

To build the crate, run the following `cargo` command:

    cargo build

### Testing

Before writing tests, read the [How to Write Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) chapter of the Rust Book.

In `Rust` projects, tests are split into unit, integration and doc tests. While it is true that unit tests are usually written in the same file of the code to be tested, here unit tests are placed along integration tests in the `tests` folder. Please, note that some tests may require assets to be executed, therefore, decompress them before the execution:

- Unzip assets with `unzip -o tests/assets -d tests`,
- To execute all tests, run the `cargo test` command,
- To execute only unit/integration tests, run `cargo test --tests`,
- To execute only doc tests, run `cargo test --doc`.

Beware that doc tests may require relevant amount of time and memory since each doc test is linked individually and the [polars](https://github.com/pola-rs/polars) dependency is huge. It could be easier to write unit/integration tests first and leave doc test at the end.

### Coverage

To compute the code coverage, run the following [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) command:

    cargo llvm-cov --html
    open target/llvm-cov/html/index.html

Or output the `lcov.info` and use an appropriate coverage extension for your IDE:

    cargo llvm-cov --lcov --output-path lcov.info

### Linting

To execute code linting, run [cargo-clippy](https://github.com/rust-lang/rust-clippy):

    cargo clippy --no-deps -- -D warnings -v

### Formatting

To execute code formatting, run [rustfmt](https://github.com/rust-lang/rustfmt):

    cargo +nightly fmt

The [+nighlty](https://doc.rust-lang.org/cargo/commands/cargo.html?highlight=toolchain#common-options) toolchain option is needed only for the formatting step.

### Documenting

The `#![warn(missing_docs)]` lint has been enforced crate-wide to ensure high-quality documentation. This means that linting will fail if there is undocumented code.

### Profiling

Currently, there are no `benches` for code profiling.
