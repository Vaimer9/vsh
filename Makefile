all: build

release: check
	cargo build --release --verbose

build: src/*.rs
	cargo build

install:
	cargo build --release
	sudo cp target/release/vsh /bin

check: format
	cargo clippy

format:
	cargo fmt
