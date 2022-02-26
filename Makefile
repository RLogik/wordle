# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# Makefile
# NOTE: Do not change the contents of this file!
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

include .env

################################
# VARIABLES
################################

ARTEFACT_NAME:=${APPNAME}
PYTHON:=python3
ifeq ($(OS),Windows_NT)
ARTEFACT_NAME:=${APPNAME}.exe
PYTHON=py -3
endif

PATH_TO_DATA:=assets/words.txt

################################
# Macros
################################

define delete_if_file_exists
	@if [ -f "$(1)" ]; then rm "$(1)"; fi
endef

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# TARGETS
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

################################
# BASIC TARGETS: setup, build, run
################################
setup: check-system-requirements setup-no-checks
setup-no-checks:
	@cargo update
build: check-all-system-requirements build-no-checks
build-no-checks:
	@$(call delete_if_file_exists,"dist/${ARTEFACT_NAME}")
	@#$(call delete_if_file_exists,"Cargo.lock")
	@cargo build --release
	@cp "target/release/${ARTEFACT_NAME}" "dist/"
run-precheck:
	@if ! [ -f "dist/${ARTEFACT_NAME}" ]; then \
		echo "No artefact found! Run `make build` first"; \
		exit 1; \
	fi
run:
	@make run-precheck
	@dist/${ARTEFACT_NAME} --it --path=${PATH_TO_DATA}
all: setup build run
################################
# TARGETS: testing
################################
unit-test: unit-tests
unit-tests:
	cargo test
################################
# AUXILIARY (INTERNAL TARGETS)
################################
get-system-requirements:
	@echo "Not implemented."
	@#echo "Install imgui requirements via pip:"
	@#${PYTHON} -m pip install imgui[full]
check-all-system-requirements:
	@make check-system-requirements
check-system-requirements:
	@if ! ( cargo version >> /dev/null 2> /dev/null ); then \
		echo "Install Rust cargo first. See README file — System Requirements / Rust."; \
		exit 1; \
	fi
	@cargo version
################################
# TARGETS: clean
################################
clean:
	@echo "All system artefacts will be force removed."
	@find . -type f -name ".DS_Store" -exec basename {} \;
	@find . -type f -name ".DS_Store" -exec rm {} \; 2> /dev/null
	@echo "All build artefacts will be force removed."
	@cd dist && find . -type f -name "${ARTEFACT_NAME}" -exec basename {} \;
	@cd dist && find . -type f -name "${ARTEFACT_NAME}" -exec rm {} \;
	@find . -type d -name "target" -exec basename {} \;
	@find . -type d -name "target" -exec rm -rf {} \; 2> /dev/null
	cargo clean
	@exit 0
################################
# ACHIVE: Python variant
################################
setup-py:
	@${PYTHON} -m pip install -r "src-py/requirements"
run-py:
	@${PYTHON} src-py/main.py -it "${PATH_TO_DATA}"
