build:
	cargo run --bin plant_website_static

bot:
	cargo run --bin plant_updater_bot

check:
	cargo clippy 
	cargo fmt --all -- --check
