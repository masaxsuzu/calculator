bin="target/release/calculator"

tmp= out.s out

.PHONY: test clean
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

clean:
	cargo clean
	rm $(tmp)