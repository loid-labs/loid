build:
    cargo build --all
    cargo install --path crates/cli

test:
    cargo test

fmt:
    cargo fmt
    pnpx @biomejs/biome lint --write ./docs/src || true
    pnpx @biomejs/biome lint --write ./docs/.vitepress/config.mts || true
    pnpx @biomejs/biome lint --write ./docs/.vitepress/theme || true