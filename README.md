# One Package Linux Command (`commander-system-managment`)

[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org/)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![Platform: Linux](https://img.shields.io/badge/platform-Linux-lightgrey.svg)](https://kernel.org)

A high-performance, modular Linux system management and metrics collection CLI utility built with Rust. Designed with an extensible trait-based architecture for inspecting system hardware, resource metrics, and running processes directly from the Linux `/proc` virtual filesystem.

---

## Key Highlights

- **Direct `/proc` Virtual FS Parsing**: Reads directly from `/proc/cpuinfo` and `/proc/{pid}/status` with zero shell overhead and microsecond latency.
- **Zero External Runtime Dependencies**: Compiles to a single standalone static binary. Ideal for minimal Docker images (Alpine/Distroless), embedded Linux systems, and cloud instances.
- **Process Status Inspector**: Analyzes process status, threads count, umask, CPU affinity, and login UID for target PIDs.
- **Extensible Collector Interface**: Standardized `Collector` trait allows seamless integration of new telemetry modules (Memory, Disk, Network).

---

## Features

| Feature | Description | Source |
| :--- | :--- | :--- |
| **CPU Telemetry** | Core counts, logical processor count, vendor ID, model name, and cache size | `/proc/cpuinfo` |
| **Process Inspector** | Target process verification, process name, threads count, umask, and CPU affinity | `/proc/{pid}/status` |
| **Argument Dispatcher** | Custom CLI argument parser supporting boolean flags and key-value options | Native parser |

---

## Project Structure

```
├── Cargo.toml
├── Cargo.lock
├── LICENSE
├── README.md
└── src
    ├── main.rs                                  # Application entry point & orchestration
    ├── system_args.rs                           # CLI argument handling
    │   └── system_args
    │       └── args.rs                          # Custom argument parsing logic
    └── collectors_interface.rs                  # Module registry for metric collectors
        └── collectors_interface
            ├── interface.rs                     # `Collector` trait contract
            ├── cpu_info_utils.rs
            ├── cpu_info_utils
            │   └── cpu_info_collector.rs        # CPU telemetry & /proc/cpuinfo parser
            ├── process_utils.rs
            └── process_utils
                └── process_info_collector.rs    # Process inspector & /proc/{pid}/status parser
```

---

## Getting Started

### Prerequisites

- **Linux OS** (relies on the Linux kernel `/proc` filesystem)
- **Rust & Cargo** (compatible with 2024 edition):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Build & Installation

Clone the repository and build the release binary:

```bash
git clone git@github.com:Ahmedfargh/one-pkg-linux-command.git
cd one-pkg-linux-command
cargo build --release
```

The optimized binary will be available at:
```bash
./target/release/commander-system-managment
```

---

## Usage

### 1. Collect CPU Information
Query CPU architecture and core details with `--cpu_info` (or `cpu_info`):

```bash
cargo run -- --cpu_info
```

### 2. Inspect a Specific Process
Query state, thread count, and attributes of a target process by PID:

```bash
cargo run -- monitor_process=1
```

### 3. Run Multiple Collectors Simultaneously
Combine multiple commands in a single execution:

```bash
cargo run -- --cpu_info monitor_process=1234
```

---

## Extending: Adding New Collectors

Every metric provider implements the standard `Collector` interface:

```rust
use crate::collectors_interface::interface::Collector;

pub struct MemoryInfoCollector;

impl Collector for MemoryInfoCollector {
    fn collect(&mut self) {
        // Read and parse /proc/meminfo directly
    }
}
```

---

## Roadmap

- [x] CPU telemetry collector (`/proc/cpuinfo`)
- [x] Process state & thread inspector (`/proc/{pid}/status`)
- [ ] Memory metrics collector (`/proc/meminfo`)
- [ ] Disk usage and I/O statistics (`/proc/diskstats`)
- [ ] Network interface bandwidth monitor (`/proc/net/dev`)
- [ ] Structured export formats (`--json`, `--yaml`, tabular view)
- [ ] Process management actions (graceful termination, niceness adjustment)

---

## Contributing

Contributions and pull requests are welcome!
1. Fork the repository
2. Create your feature branch (`git checkout -b feature/memory-collector`)
3. Commit your changes (`git commit -m 'Add memory metrics collector'`)
4. Push to the branch (`git push origin feature/memory-collector`)
5. Open a Pull Request

---

## License

This project is licensed under the terms of the GNU General Public License v3.0 ([LICENSE](LICENSE)).
