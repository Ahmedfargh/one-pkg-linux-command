# One Package Linux Command (`commander-system-managment`)

[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org/)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)

A modular, lightweight Linux system management and monitoring CLI utility built with Rust. Designed with an extensible trait-based architecture for collecting system, hardware, and process metrics directly from the Linux `/proc` virtual filesystem.

---

## Features

- **CPU Information Gathering**: Inspects and parses `/proc/cpuinfo` to extract processor count, CPU cores, cache size, vendor ID, and model name.
- **Process Monitoring**: Inspects target processes via `/proc/{pid}` to track status, thread count, and resource allocation.
- **Extensible Architecture**: Built around a clean `Collector` trait interface, allowing new collectors (e.g., Memory, Disk, Network) to be added effortlessly.
- **Zero Heavy Dependencies**: Lightweight, fast startup, and native Linux integration using Rust's standard library.

---

## Project Structure

```
├── Cargo.toml
├── LICENSE
├── README.md
└── src
    ├── main.rs                                  # Application entry point & orchestration
    ├── system_args.rs                           # CLI argument handling
    │   └── system_args
    │       └── args.rs                          # Custom argument parsing logic
    └── collectors_interface.rs                  # Module registry for metric collectors
        └── collectors_interface
            ├── interface.rs                     # `Collector` trait definition
            ├── cpu_info_utils.rs
            ├── cpu_info_utils
            │   └── cpu_info_collector.rs        # CPU metric collector & /proc/cpuinfo parser
            ├── process_utils.rs
            └── process_utils
                └── process_info_collector.rs    # Process monitoring collector
```

---

## Getting Started

### Prerequisites

- **Linux OS** (relies on the `/proc` filesystem)
- **Rust & Cargo** (compatible with the 2024 edition):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Build

Clone the repository and build the binary:

```bash
git clone git@github.com:Ahmedfargh/one-pkg-linux-command.git
cd one-pkg-linux-command
cargo build --release
```

The compiled binary will be located at `target/release/commander-system-managment`.

---

## Usage

### 1. Collect CPU Info
Pass the `--cpu_info` (or `cpu_info`) flag to inspect CPU details:

```bash
cargo run -- --cpu_info
```

### 2. Monitor a Process
Pass `monitor_process=<PID>` to inspect a specific process:

```bash
cargo run -- monitor_process=1
```

### 3. Combine Arguments
You can run multiple collectors in one execution:

```bash
cargo run -- --cpu_info monitor_process=1234
```

---

## Architecture: Adding New Collectors

All collectors implement the standard `Collector` trait:

```rust
use crate::collectors_interface::interface::Collector;

pub struct MemoryCollector;

impl Collector for MemoryCollector {
    fn collect(&mut self) {
        // Read and parse /proc/meminfo
    }
}
```

---

## Roadmap

- [ ] Add Memory (`/proc/meminfo`) metrics collector
- [ ] Add Disk usage and I/O metrics collector
- [ ] Add Network interfaces and bandwidth collector
- [ ] Output formatting options (Plain Text, JSON, YAML)
- [ ] Interactive TUI dashboard mode

---

## License

This project is licensed under the terms of the GNU General Public License v3.0 ([LICENSE](LICENSE)).
