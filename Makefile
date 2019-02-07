.PHONY: test
test:
	cargo fmt
	cargo check
	cargo test

run:
	cargo run