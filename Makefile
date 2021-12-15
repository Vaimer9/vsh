release: check
	cargo build --release --verbose

check: format
	cargo clippy

format:
	cargo fmt
