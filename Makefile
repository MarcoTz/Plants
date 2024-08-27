build:
	cargo run --bin plant_website_static

bot:
	cargo run --bin plant_updater_bot

port:
	- rm -r data_old
	- rm -r data
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
