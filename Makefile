SHELL := /bin/bash

run:
	cargo run

test: clean
	cargo test
	make clean

clean:
	true

build:
	cargo build --release

lazycommit:
	git add .
	git commit -m "..."
	git push
