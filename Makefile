install:
	cargo add clap --features derive
	cargo add walkdir
	cargo add regex
	cargo add dirs

build:
	cargo build
