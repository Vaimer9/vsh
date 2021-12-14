release: check
	cargo build --release --verbose

check: format
	cargo clippy

format:
	cargo fmt

ship: commit
	git push -u origin main

commit:
	git commit -m 
