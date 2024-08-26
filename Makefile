build:
	cargo run --bin plant_website_static

bot:
	cargo run --bin plant_updater_bot

port:
	- rm -r data_old
	- mkdir data_old
	cp ../Plants/Plants data_old -r
	cp ../Plants/PlantSpecies data_old -r 
	cp ../Plants/Logs data_old -r 
	cp ../Plants/html_out/img/plants/ data_old/img -r
	- rm -r data_old/img/small
	- rm -r data
	cargo run --bin port_data

check:
	cargo clippy 
	cargo fmt --all -- --check
