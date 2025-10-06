set dotenv-load := true

check: fmt-check cargo-check test clippy

check-local: cargo-check test-local clippy

before-commit: fix check-local

fix:
    cargo +nightly fix --allow-dirty --allow-staged
    cargo +nightly fmt --

fmt-check:
    cargo +nightly fmt -- --check

cargo-check:
    cargo +nightly check

test:
    cargo +nightly test --all -Z unstable-options --no-fail-fast --features sdl_build_from_source

test-local:
    cargo +nightly test --all -Z unstable-options --no-fail-fast

clippy:
    cargo +nightly clippy -- -D warnings -D clippy::pedantic -A clippy::cast_precision_loss -A clippy::cast_possible_truncation -A clippy::cast_possible_wrap -A clippy::cast_sign_loss -A clippy::cast_lossless -A clippy::module_name_repetitions -A clippy::unnecessary_box_returns --verbose --no-deps

update:
    cargo +nightly update
