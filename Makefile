.PHONY: clean build full

MAINPY = "main.py"
OUTDIR = "html_out"
JSONUPDATER = "updater.py"

new-species: 
	python $(JSONUPDATER) add-species
new-plant:
	python $(JSONUPDATER) add-plant
new-activities:
	python $(JSONUPDATER) add-activities
new-growth:
	python $(JSONUPDATER) add-growth
move-to-graveyard:
	python $(JSONUPDATER) move-to-graveyard
update-plant:
	python $(JSONUPDATER) update-plant


clean: 
	rm -rf $(OUTDIR)/**/*.html

build: 
	python $(MAINPY)

full: clean build
