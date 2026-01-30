# crabiec61850

High-performance Rust components for IEC 61850 Sampled Values (SV) over Ethernet. This repository provides building blocks to publish and subscribe to SV streams, with a focus on zero-copy packet handling and OS-level socket operations for low-latency processing. Work in progress to support another protocols

The project is organized into modules for network IO (raw Ethernet sockets, packet parsing), protocol models (Ethernet and IEC 61850 SV), and higher-level publisher/subscriber utilities.

## Features

- Raw Ethernet frame construction and parsing
- IEC 61850-9-2 / 61869 Sampled Values modeling
- Publisher and Subscriber helpers for SV streams
- Benchmarks for packet and socket operations (Criterion)
- No-std friendly submodules where possible (core modeling)

## Repository layout

- src/
  - main.rs: Example binary entry point
  - lib.rs: Library entry for reusable components
  - network/
    - socket.rs: Raw socket utilities (open, bind, send/recv)
    - packet.rs: Low-level Ethernet packet helpers
    - eth_types.rs: Ethernet type definitions and constants
    - publisher/: SV TX utilities
    - subscriber/: SV RX utilities
  - protocols/
    - ethernet/: Ethernet frame model
    - sampled_values/: SV models (ASDU, phases, dataset)
  - standards/: ASN.1 helpers and related utilities
- benches/: Criterion benchmarks (nanosleep, packet, socket)
- Cargo.toml: Crate configuration
- Makefile: Common development tasks

Note: This layout description is derived from the repository tree at the time of writing.

## Getting started

Prerequisites:
- Rust toolchain (stable). Install via https://rustup.rs
- Linux is required for raw socket operations used by publisher/subscriber examples

Clone and build:

```bash
git clone <repo-url>
cd crabiec61850
cargo build --release
```

To recieve packets from the same device, you should enable interface promiscuos mode:

```bash
make on iface=lo # Enable promiscuos
make show iface=lo # Debug if it's on promiscuous mode
make off iface=lo # Disable promiscuos
```

Run example binary:

```bash
make run
```


## Benchmarks

This project uses Criterion for microbenchmarks located in benches/.

Run all benchmarks:

```bash
cargo bench
```

After running, open the HTML report:

- target/criterion/report/index.html


## Tests

Run tests:

```bash
cargo test
```

## Publisher/Subscriber overview

- network/publisher/sampled_value_pub.rs: Utilities to construct and send SV frames via raw Ethernet sockets. Expect configuration for interface name, destination MAC, VLAN (if used), APPID, SVID, and sampling rate.
- network/subscriber/sampled_value_sub.rs: Utilities to receive and parse SV frames from an interface, with fast filtering for APPID and VLAN.
- protocols/sampled_values/: Models for SV, ASDU, and phases to represent samples and metadata in a type-safe way.

Exact usage APIs can be found in the source files. The library entry (lib.rs) re-exports core types to make integration easier.

## Safety and permissions

- Raw sockets typically require CAP_NET_RAW (and often CAP_NET_ADMIN) on Linux. Use setcap as shown above.
- Exercise caution when sending frames on a production network. Validate VLAN/APPID and destination addresses according to your lab setup.

## Makefile shortcuts

Common tasks (inspect Makefile for up-to-date targets):

```bash
make build      # cargo build --release
make bench      # cargo bench
make test       # cargo test
make fmt        # cargo fmt
make clippy     # cargo clippy -- -D warnings
```

## Roadmap ideas

- busywait option instead of OS sleep
- VLAN tagging support and configuration helpers
- Wrapper for python

## Contributing

Issues and PRs are welcome. Please run fmt, clippy, tests, and include Criterion benchmark deltas when changing performance-critical code.

## License

Specify your chosen license here (e.g., MIT/Apache-2.0). Add a LICENSE file at the repository root.
