.POSIX:
.SILENT:
.PHONY: all

all:
	cargo install --force \
		cargo-audit \
		cargo-cache \
		cargo-edit \
		tinyrick@0.0.30
	rustup component add \
		clippy \
		rustfmt
