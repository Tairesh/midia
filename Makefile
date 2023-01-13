export MIDIA_VERSION_POSTFIX=
export RUST_BACKTRACE=1
export CARGO_TERM_COLOR=always

build:
	cargo build --release

run:
	cargo run --release

before-commit: fmt check
check: fmt-check cargo-check test clippy

fmt:
	cargo fmt --

fmt-check:
	cargo fmt -- --check

test:
	cargo test

clippy:
	cargo clippy -- -D warnings -D clippy::pedantic -A clippy::cast_precision_loss -A clippy::cast_possible_truncation -A clippy::cast_possible_wrap -A clippy::cast_sign_loss -A clippy::cast_lossless -A clippy::module_name_repetitions --verbose --no-deps

clean:
	cargo clean

cargo-check:
	cargo check
