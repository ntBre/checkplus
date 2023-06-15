clippy:
	cargo clippy --workspace --all-targets --all-features

run:
	cargo run -- --depth 20 testfiles/sample.pgn

test:
	cargo test -- --nocapture --test-threads=1 $(ARGS)

build:
	cargo build --release

install:
	cp -i target/release/checkplus /usr/bin/checkplus
