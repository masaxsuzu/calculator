.PHONY: test
test:
	cargo fmt
	cargo check
	cargo test
