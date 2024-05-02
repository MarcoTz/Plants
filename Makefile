.PHONY: clean build full

MAINPY = "main.py"
OUTDIR = "html_out"

clean: 
	rm -rf $(OUTDIR)

build: 
	python $(MAINPY)

full: clean build
