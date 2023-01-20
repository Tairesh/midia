set dotenv-load := true

check: fmt-check cargo-check test clippy

fmt-check:
	cargo fmt -- --check

cargo-check:
	cargo check

test:
	cargo test

clippy:
	cargo clippy -- -D warnings -D clippy::pedantic -A clippy::cast_precision_loss -A clippy::cast_possible_truncation -A clippy::cast_possible_wrap -A clippy::cast_sign_loss -A clippy::cast_lossless -A clippy::module_name_repetitions --verbose --no-deps
