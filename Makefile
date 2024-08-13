build:
	cargo run 

check:
	cargo clippy 
	cargo fmt --all -- --check
