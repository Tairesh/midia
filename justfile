set dotenv-load := true

check: fmt-check cargo-check test clippy
before-commit: update fix check

fix:
    cargo fix --allow-dirty --allow-staged
    cargo fmt --

fmt-check:
	cargo fmt -- --check

cargo-check:
	cargo check

test:
	cargo +nightly test --all -Z unstable-options --no-fail-fast

clippy:
	cargo +nightly clippy -- -D warnings -D clippy::pedantic -A clippy::cast_precision_loss -A clippy::cast_possible_truncation -A clippy::cast_possible_wrap -A clippy::cast_sign_loss -A clippy::cast_lossless -A clippy::module_name_repetitions -A clippy::unnecessary_box_returns --verbose --no-deps

update:
    cargo update
