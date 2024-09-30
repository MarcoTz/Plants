build: 
	cargo build --release 
	sudo systemctl restart bot_service

website:
	- rm -r html_out/img
	- rm -r html_out/js
	cargo run --bin plant_website_static
	cp data/Plants/ html_out/img -r 
	cp ./js html_out/js -r 

bot:
	cargo run --bin plant_updater_bot

port:
	- rm -r data_old
	- rm -r data/Plants
	- rm -r data/Species
	- rm -r data/Logs
	- rm data/Locations.csv
	- mkdir data_old
	cp ../Plants/Plants data_old -r
	cp ../Plants/PlantSpecies data_old -r 
	cp ../Plants/Logs data_old -r 
	cp ../Plants/html_out/img/plants/ data_old/img -r
	- rm -r data_old/img/small
	cargo run --bin port_data
	cp ../Plants/Logs/Graveyard.csv data/Logs


check:
	cargo clippy 
	cargo fmt --all -- --check

.PHONY: test
test:
	cargo test --all --no-fail-fast

.PHONY: coverage
coverage:
	@echo "Make sure to install via cargo install cargo-llvm-cov first"
	cargo llvm-cov --workspace --html
	cargo llvm-cov --workspace --open
