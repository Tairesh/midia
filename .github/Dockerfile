FROM fedora:42

RUN dnf install -y \
    git \
    gcc \
    SDL3-devel \
    rustup \
    just \
    && dnf clean all

RUN rustup-init -y --default-toolchain nightly --component clippy,rustfmt
ENV PATH="/root/.cargo/bin:${PATH}"