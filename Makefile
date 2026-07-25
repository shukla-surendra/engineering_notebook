.PHONY: help install docs mkdocs sd-serve sd-build sd-clean dsa-serve dsa-build dsa-clean staff-serve staff-build staff-clean build clean

help:
	@echo "  make docs        - (alias: make mkdocs) build (--strict) then serve ALL THREE sites together, live-reloading:"
	@echo "                       DSA                    -> http://127.0.0.1:8000"
	@echo "                       ML System Design       -> http://127.0.0.1:8001"
	@echo "                       Staff System Design    -> http://127.0.0.1:8002"
	@echo "                     (Ctrl+C stops all three)"
	@echo ""
	@echo "Staff-Level System Design docs (staff_system_design/, mkdocs-staff-system-design.yml):"
	@echo "  make staff-serve - serve at http://127.0.0.1:8002"
	@echo "  make staff-build - build static site into site-staff-system-design/"
	@echo "  make staff-clean - remove site-staff-system-design/"
	@echo ""
	@echo "ML System Design docs (system_design/, mkdocs-system-design.yml):"
	@echo "  make sd-serve    - serve at http://127.0.0.1:8001"
	@echo "  make sd-build    - build static site into site-system-design/"
	@echo "  make sd-clean    - remove site-system-design/"
	@echo ""
	@echo "DSA docs (dsa_prep/, mkdocs-dsa.yml):"
	@echo "  make dsa-serve   - serve at http://127.0.0.1:8000"
	@echo "  make dsa-build   - build static site into site-dsa/"
	@echo "  make dsa-clean   - remove site-dsa/"
	@echo ""
	@echo "  make build       - build all three sites (static, --strict)"
	@echo "  make clean       - remove all three built sites"
	@echo "  make install     - install mkdocs + mkdocs-material + pymdown-extensions"

install:
	python3 -m pip install mkdocs mkdocs-material pymdown-extensions

sd-serve:
	mkdocs serve -f mkdocs-system-design.yml -a 127.0.0.1:8001

sd-build:
	mkdocs build -f mkdocs-system-design.yml --strict

sd-clean:
	rm -rf site-system-design

dsa-serve:
	mkdocs serve -f mkdocs-dsa.yml -a 127.0.0.1:8000

dsa-build:
	mkdocs build -f mkdocs-dsa.yml --strict

dsa-clean:
	rm -rf site-dsa

staff-serve:
	mkdocs serve -f mkdocs-staff-system-design.yml -a 127.0.0.1:8002

staff-build:
	mkdocs build -f mkdocs-staff-system-design.yml --strict

staff-clean:
	rm -rf site-staff-system-design

build: dsa-build sd-build staff-build

clean: dsa-clean sd-clean staff-clean

mkdocs: docs

docs: build
	@echo ""
	@echo "DSA docs:                  http://127.0.0.1:8000"
	@echo "ML System Design docs:     http://127.0.0.1:8001"
	@echo "Staff System Design docs:  http://127.0.0.1:8002"
	@echo "(Ctrl+C stops all three)"
	@echo ""
	@trap 'kill 0' EXIT INT TERM; \
	mkdocs serve -f mkdocs-dsa.yml -a 127.0.0.1:8000 & \
	mkdocs serve -f mkdocs-system-design.yml -a 127.0.0.1:8001 & \
	mkdocs serve -f mkdocs-staff-system-design.yml -a 127.0.0.1:8002 & \
	wait
