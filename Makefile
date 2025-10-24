release:
	cargo build --release && sudo ./target/release/crabiec61850

test:
	cargo test