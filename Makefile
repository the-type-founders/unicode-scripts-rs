.PHONY: all
all: check test

.PHONY: benchmark
benchmark:
	cargo +nightly bench

.PHONY: check
check:
	cargo clippy -- -D warnings
	cargo fmt --all -- --check

.PHONY: test
test:
	cargo test
