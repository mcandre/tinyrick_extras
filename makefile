.POSIX:
.SILENT:
ALLTARGETS!=ls -a
.PHONY: $(ALLTARGETS)

all: crates rustup-components

crates:
	cargo install --force \
		cargo-audit \
		tinyrick@0.0.15 \
		unmake@0.0.18

rustup-components:
	rustup component add \
		clippy \
		rustfmt
