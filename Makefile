.PHONY: clean build full

MAINPY = "main.py"
OUTDIR = "html_out"
ADDSPECIES = "add_species.py"

new-species: 
	python $(ADDSPECIES)

clean: 
	rm -rf $(OUTDIR)

build: 
	python $(MAINPY)

full: clean build
