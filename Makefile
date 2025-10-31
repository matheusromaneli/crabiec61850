release:
	cargo build --release && sudo ./target/release/crabiec61850

run:
	cargo build && sudo ./target/debug/crabiec61850

test:
	cargo test

.PHONY: on off show
on:
	sudo ip link set dev $(iface) promisc on
show:
	ip link show $(iface)
off:
	sudo ip link set dev $(iface) promisc off