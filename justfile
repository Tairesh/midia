set dotenv-load := true

check: fmt-check cargo-check test clippy
before-commit: fix check

fix:
    cargo +nightly fix --allow-dirty --allow-staged
    cargo +nightly fmt --

fmt-check:
	cargo +nightly fmt -- --check

cargo-check:
	cargo +nightly check

test:
	cargo +nightly test --all -Z unstable-options --no-fail-fast

clippy:
	cargo +nightly clippy -- -D warnings -D clippy::pedantic -A clippy::cast_precision_loss -A clippy::cast_possible_truncation -A clippy::cast_possible_wrap -A clippy::cast_sign_loss -A clippy::cast_lossless -A clippy::module_name_repetitions -A clippy::unnecessary_box_returns --verbose --no-deps

update:
    cargo +nightly update
