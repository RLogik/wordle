set shell := [ "bash", "-uc" ]

_default:
    @- just --unsorted --choose
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# Justfile
# NOTE: Do not change the contents of this file!
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# VARIABLES
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

ARTEFACT_NAME := "wordle"
PYTHON := if os_family() == "windows" { "py -3" } else { "python3" }

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# Macros
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

_create-file-if-not-exists fname:
    @touch "{{fname}}";

_create-folder-if-not-exists path:
    @if ! [ -d "{{path}}" ]; then mkdir "{{path}}"; fi

_delete-if-file-exists fname:
    @if [ -f "{{fname}}" ]; then rm "{{fname}}"; fi

_delete-if-folder-exists path:
    @if [ -d "{{path}}" ]; then rm -rf "{{path}}"; fi

_clean-all-files pattern:
    @find . -type f -name "{{pattern}}" -exec basename {} \; 2> /dev/null
    @- find . -type f -name "{{pattern}}" -exec rm {} \; 2> /dev/null

_clean-all-folders pattern:
    @find . -type d -name "{{pattern}}" -exec basename {} \; 2> /dev/null
    @- find . -type d -name "{{pattern}}" -exec rm -rf {} \; 2> /dev/null

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# TARGETS
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# TARGETS: build
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

setup: _check-system-requirements _setup-no-checks
_setup-no-checks:
    @cargo update
build: _check-all-system-requirements _build-no-checks
_build-no-checks:
    @just _delete-if-file-exists "dist/{{ARTEFACT_NAME}}"
    @cargo build --release
    @cp "target/release/{{ARTEFACT_NAME}}" "dist/"

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# TARGETS: run
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

run:
    @just _run-precheck
    @dist/{{ARTEFACT_NAME}}
_run-precheck:
    @if ! [ -f "dist/{{ARTEFACT_NAME}}" ]; then \
        echo "No artefact found! Run `make build` first"; \
        exit 1; \
    fi

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# TARGETS: tests
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

tests: unit-tests
unit-tests:
    @cargo test

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# AUXILIARY (INTERNAL TARGETS)
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

_check-all-system-requirements:
    @just _check-system-requirements
_check-system-requirements:
    #!/usr/bin/env bash
    if ! ( cargo version >> /dev/null 2> /dev/null ); then \
        echo "Install Rust cargo first. See README file — System Requirements / Rust."; \
        exit 1; \
    fi
    cargo version

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# AUXILIARY (INTERNAL TARGETS)
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

clean:
    @echo "All system artefacts will be force removed."
    @- just _clean-all-files ".DS_Store" 2> /dev/null
    @echo "All test artefacts will be force removed."
    @- just _delete-if-folder-exists "logs"
    @echo "All build artefacts will be force removed."
    @- just _clean-all-folders "target" 2> /dev/null
    @- just _clean-all-folders "__pycache__" 2> /dev/null
    @cargo clean
