# ping-rs

`ping-rs` is a Rust implementation of the classic **`ping` utility** found on Linux.
The goal is to **faithfully recreate everything the GNU/Linux `ping` tool can do** — from sending ICMP Echo requests to providing detailed network statistics.

This project focuses on **performance, correctness, and learning**, exploring raw sockets, networking internals, and safe abstractions in Rust.

---

## Features (current & planned)

* [x] Send ICMP Echo requests (IPv4)
* [x] Receive and parse ICMP Echo replies
* [ ] Measure round-trip time (RTT)
* [ ] IPv6 support
* [ ] TTL and packet size configuration (`-t`, `-s`)
* [ ] Interval and timeout options (`-i`, `-W`)
* [ ] Packet loss, jitter, and summary statistics
* [ ] Verbose/debug output (`-v`)
* [ ] Flood ping (`-f`)
* [ ] Deadline and count limits (`-w`, `-c`)
* [ ] Source address binding (`-I`)
* [ ] Record route / timestamp options (if supported by OS)
* [ ] Full compatibility with GNU/Linux `ping` flags and output

---

## Getting Started

### Prerequisites

* **Rust** (latest stable recommended)
* **Linux system** (for raw socket support)
* **Root privileges** or correct binary capabilities

### Installation

Clone the repository and build with Cargo:

```bash
git clone https://github.com/0xA1M/ping-rs
cd ping-rs
cargo build --release
```

Optionally, install locally:

```bash
cargo install --path .
```

### Running

By default, `ping-rs` needs root privileges to open raw sockets:

```bash
sudo ./target/release/ping-rs 8.8.8.8
```

Alternatively, grant the binary raw socket capability so it can run without `sudo`:

```bash
sudo setcap cap_net_raw+ep ./target/release/ping-rs
./target/release/ping-rs 8.8.8.8
```

---

## Development

Run in debug mode:

```bash
cargo run -- 127.0.0.1
```

With verbose logging:

```bash
RUST_LOG=debug cargo run -- 1.1.1.1
```

---

## Dependencies

* [`socket2`](https://crates.io/crates/socket2) → Safe and flexible interface for raw sockets
* [`clap`](https://crates.io/crates/clap) → Command-line argument parsing (for full GNU/Linux `ping` flag support)

---

## Learning Goals

This project serves as a hands-on way to:

* Work with **raw sockets** safely in Rust
* Understand **ICMP and networking internals**
* Recreate a **well-known Linux utility** from scratch
* Explore **low-level system programming** with modern abstractions using Rust

---

## References

* [Wikipedia page on `ping`](https://en.wikipedia.org/wiki/Ping_(networking_utility))
* [Linux `ping` man page](https://man7.org/linux/man-pages/man8/ping.8.html)
* [RFC 792 - Internet Control Message Protocol](https://datatracker.ietf.org/doc/html/rfc792)
* [RFC 4443 - ICMPv6](https://datatracker.ietf.org/doc/html/rfc4443)
* [socket2 crate](https://docs.rs/socket2/latest/socket2/)
* [clap crate](https://docs.rs/clap/latest/clap/)

---

## License

MIT License. See [LICENSE](LICENSE) for details.
