build:
	cargo run --bin plant_website_static

bot:
	cargo run --bin plant_updater_bot

port:
	cargo run --bin port_data

check:
	cargo clippy 
	cargo fmt --all -- --check
