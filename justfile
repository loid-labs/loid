build:
    cargo build --all
    cargo install --path crates/cli

test:
    cargo test

fmt:
    cargo fmt