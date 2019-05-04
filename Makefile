.PHONY: test
test:
	cargo fmt
	cargo check
	cargo test --release

run: test
	cargo run