run:
	cargo run

test:
	cargo test -- --nocapture --test-threads=1 $(ARGS)
