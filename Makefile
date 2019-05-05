bin="target/release/calculator"

.PHONY: test
test:
	cargo fmt
	cargo check
	cargo test --release
$(bin) : test
	cargo build --release

fulltest:$(bin)
	@bash tests/full.sh

run: test
	cargo run