build:
	- rm -r html_out/img
	cargo run --bin plant_website_static
	cp data/Plants/ html_out/img -r 

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
