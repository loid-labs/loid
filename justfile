build:
    cargo build --all
    cargo install --path bins/cli

test:
    cargo nextest run

fmt:
    cargo fmt
    ruff format