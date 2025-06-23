set dotenv-load := true

just := just_executable()
cargo := `which cargo`

build:
    {{ cargo }} build --all
    {{ cargo }} install --path bins/cli

test:
    {{ cargo }} nextest run

fmt:
    {{ cargo }} fmt
