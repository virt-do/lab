TMP_BUNDLE ?= /tmp/run0-bundle

# Helper to create a bundle and move it into /tmp.
# Simply call `make bundle` before running `make run`
bundle:
	../do-vmm/rootfs/mkbundle.sh $(TMP_BUNDLE)

# Helper to build run0.
build:
	cargo build

# Helper to run run0.
# Requires that `make bundle` was executed before.
run: build
	sudo ./target/debug/$(shell cat Cargo.toml | grep "name = " | sed 's/name = //g' | cut -d '"' -f2) -b $(TMP_BUNDLE)