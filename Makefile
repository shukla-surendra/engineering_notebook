.PHONY: help install serve build clean docs mkdocs

help:
	@echo "  make serve   - serve the whole site (live-reloading) at http://127.0.0.1:8000"
	@echo "  make build   - build (--strict) the static site into site/"
	@echo "  make clean   - remove site/"
	@echo "  make install - install mkdocs + mkdocs-material + pymdown-extensions"
	@echo ""
	@echo "All five tracks (DSA, System Design Foundation, System Design Practice,"
	@echo "Security, Low-Level Design) are one MkDocs site (mkdocs.yml), each its own nav tab."

install:
	python3 -m pip install mkdocs mkdocs-material pymdown-extensions

serve:
	mkdocs serve -a 127.0.0.1:8000

build:
	mkdocs build --strict

clean:
	rm -rf site

docs: build
	@echo ""
	@echo "Built. Run 'make serve' to preview at http://127.0.0.1:8000"

mkdocs: docs
