build:
    cargo build --all
    cargo install --path bins/cli

test:
    cargo test

fmt:
    cargo fmt
    ruff format