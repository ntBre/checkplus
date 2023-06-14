clippy:
	cargo clippy --workspace --all-targets --all-features

run:
	cargo run -- --depth 20 testfiles/sample.pgn

test:
	cargo test -- --nocapture --test-threads=1 $(ARGS)
