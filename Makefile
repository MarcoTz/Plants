.PHONY: clean build full

MAINPY = "main.py"
OUTDIR = "html_out"
ADDSPECIES = "json_updater.py"

new-species: 
	python $(ADDSPECIES)

clean: 
	rm -rf $(OUTDIR)

build: 
	python $(MAINPY)

full: clean build
