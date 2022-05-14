#!/usr/bin/env just --justfile

# Setup the development environment
setup-dev:
    @echo Installing nightly \`rustfmt\`
    @rustup toolchain install nightly --component rustfmt
    @echo Nightly \`rustfmt\` successfully installed!

    @echo Installing \`pre-commit\`
    @pip install pre-commit
    @pre-commit install
    @echo \`pre-commit\` hooks successfully installed!

    @echo Installing \`codespell\`
    @pip install codespell
    @echo \`codespell\` successfully installed!

    @echo Development environment installed successfully!

# Run checks
check: fmt clippy test
    @echo Checks were successful!

clean:
    @cargo clean
    @echo Done!

distclean:
    @rm --recursive --force .cargo target
    @echo Done!

# Build the project
build:
    @cargo build
    @echo Project successfully built!

# Run the tests
test +ARGS="":
    @cargo test --all-features --workspace {{ARGS}}

# Lint the codebase
clippy:
    @cargo clippy --all-targets --all-features --workspace -- --deny clippy::pedantic
    @echo Lint successful!

# Format the codebase
fmt:
    @cargo +nightly fmt --all
    @echo Codebase formatted successfully!
