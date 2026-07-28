.PHONY: help install docs mkdocs sd-serve sd-build sd-clean dsa-serve dsa-build dsa-clean practice-serve practice-build practice-clean security-serve security-build security-clean build clean

help:
	@echo "  make docs        - (alias: make mkdocs) build (--strict) then serve ALL FOUR sites together, live-reloading:"
	@echo "                       DSA                    -> http://127.0.0.1:8000"
	@echo "                       ML System Design       -> http://127.0.0.1:8001"
	@echo "                       System Design Practice -> http://127.0.0.1:8002"
	@echo "                       Security Engineering   -> http://127.0.0.1:8003"
	@echo "                     (Ctrl+C stops all four)"
	@echo ""
	@echo "System Design Practice docs (system_design_practice/, mkdocs-system-design-practice.yml):"
	@echo "  make practice-serve - serve at http://127.0.0.1:8002"
	@echo "  make practice-build - build static site into site-system-design-practice/"
	@echo "  make practice-clean - remove site-system-design-practice/"
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
	@echo "Security Engineering docs (security/, mkdocs-security.yml):"
	@echo "  make security-serve - serve at http://127.0.0.1:8003"
	@echo "  make security-build - build static site into site-security/"
	@echo "  make security-clean - remove site-security/"
	@echo ""
	@echo "  make build       - build all four sites (static, --strict)"
	@echo "  make clean       - remove all four built sites"
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

practice-serve:
	mkdocs serve -f mkdocs-system-design-practice.yml -a 127.0.0.1:8002

practice-build:
	mkdocs build -f mkdocs-system-design-practice.yml --strict

practice-clean:
	rm -rf site-system-design-practice

security-serve:
	mkdocs serve -f mkdocs-security.yml -a 127.0.0.1:8003

security-build:
	mkdocs build -f mkdocs-security.yml --strict

security-clean:
	rm -rf site-security

build: dsa-build sd-build practice-build security-build

clean: dsa-clean sd-clean practice-clean security-clean

mkdocs: docs

docs: build
	@echo ""
	@echo "DSA docs:                  http://127.0.0.1:8000"
	@echo "ML System Design docs:     http://127.0.0.1:8001"
	@echo "System Design Practice docs: http://127.0.0.1:8002"
	@echo "Security Engineering docs: http://127.0.0.1:8003"
	@echo "(Ctrl+C stops all four)"
	@echo ""
	@trap 'kill 0' EXIT INT TERM; \
	mkdocs serve -f mkdocs-dsa.yml -a 127.0.0.1:8000 & \
	mkdocs serve -f mkdocs-system-design.yml -a 127.0.0.1:8001 & \
	mkdocs serve -f mkdocs-system-design-practice.yml -a 127.0.0.1:8002 & \
	mkdocs serve -f mkdocs-security.yml -a 127.0.0.1:8003 & \
	wait
