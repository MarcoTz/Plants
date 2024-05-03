.PHONY: clean build full

MAINPY = "main.py"
OUTDIR = "html_out"
JSONUPDATER = "json_updater.py"

new-species: 
	python $(JSONUPDATER) add-species
new-plant:
	python $(JSONUPDATER) add-plant


clean: 
	rm -rf $(OUTDIR)

build: 
	python $(MAINPY)

full: clean build
