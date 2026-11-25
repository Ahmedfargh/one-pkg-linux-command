# One Package Linux Command (`commander-system-managment`)

[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org/)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![Platform: Linux](https://img.shields.io/badge/platform-Linux-lightgrey.svg)](https://kernel.org)

A high-performance, modular Linux system management and metrics collection CLI utility built with Rust. Designed with an extensible trait-based architecture for inspecting system hardware, resource metrics, and running processes directly from the Linux `/proc` virtual filesystem.

---

## Key Highlights

- **Direct `/proc` Virtual FS Parsing**: Reads directly from `/proc/cpuinfo`, `/proc/meminfo`, and `/proc/{pid}/status` with zero shell overhead and microsecond latency.
- **Zero External Runtime Dependencies**: Compiles to a single standalone static binary. Ideal for minimal Docker images (Alpine/Distroless), embedded Linux systems, and cloud instances.
- **Clean Aligned Telemetry Cards**: Renders structured, terminal-friendly telemetry cards with dynamic usage progress bars and aligned borders.
- **Extensible Collector Interface**: Standardized `Collector` trait allows seamless integration of new telemetry modules (Disk, Network, Sensors).

---

## Features

| Feature | Description | Source |
| :--- | :--- | :--- |
| **CPU Telemetry** | Model name, vendor ID, physical core count, logical threads, and cache size | `/proc/cpuinfo` |
| **Memory Telemetry** | Total, used, available, free, buffer/cache, swap metrics, and visual progress bar | `/proc/meminfo` |
| **Process Inspector** | Process verification, state, threads count, umask, CPU affinity, and login UID | `/proc/{pid}/status` |
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
            ├── memory_utils.rs
            ├── memory_utils
            │   └── memory_collector.rs          # Memory telemetry & /proc/meminfo parser
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

## Usage & Output Cards

### 1. Collect CPU Information
Query CPU architecture, core counts, and cache details:

```bash
cargo run -- --cpu-info
```

```text
╭────────────────────────── CPU Telemetry ─────────────────────────╮
│ Model Name        : Intel(R) Core(TM) i7-8850H CPU @ 2.60GHz     │
│ Vendor ID         : GenuineIntel                                 │
│ Physical Cores    : 6 cores                                      │
│ Logical Processors: 12 threads                                   │
│ Cache Size        : 9216.00 KB (9.00 MB)                         │
╰──────────────────────────────────────────────────────────────────╯
```

### 2. Inspect a Specific Process
Query state, thread count, and attributes of a target process by PID (supports PIDs up to `pid_max`):

```bash
cargo run -- monitor_process=1
```

```text
╭──────────────────────── Process Telemetry ────────────────────────╮
│ PID               : 1                                            │
│ Process Name      : systemd                                      │
│ State             : S (sleeping)                                 │
│ Threads           : 1                                            │
│ Umask             : 0000                                         │
│ CPU Affinity      : 0-11                                         │
│ Login UID         : N/A                                          │
╰──────────────────────────────────────────────────────────────────╯
```

### 3. Collect Memory Information
Query memory allocation, available RAM, buffers/cache, and usage status:

```bash
cargo run -- --memory-info
```

```text
╭──────────────────────── Memory Telemetry ────────────────────────╮
│ Total Memory      :   15.40 GB                                   │
│ Used Memory       :    9.22 GB  ( 59.8%)                         │
│ Available Memory  :    6.19 GB  ( 40.2%)                         │
│ Free Memory       :    0.94 GB                                   │
│ Buffers & Cache   :    6.31 GB                                   │
│ Swap Cached       :    0.00 GB                                   │
│ Usage Status      : [■■■■■■■■■■■■░░░░░░░░]  59.8%                │
╰──────────────────────────────────────────────────────────────────╯
```

### 4. Run Multiple Collectors Simultaneously
Combine multiple flags in a single command:

```bash
cargo run -- --cpu-info --memory-info monitor_process=1
```

---

## Extending: Adding New Collectors

Every metric provider implements the standard `Collector` interface:

```rust
use crate::collectors_interface::interface::Collector;

pub struct DiskInfoCollector;

impl Collector for DiskInfoCollector {
    fn collect(&mut self) {
        // Read and parse /proc/diskstats directly
    }
}
```

---

## Roadmap

- [x] CPU telemetry collector (`/proc/cpuinfo`)
- [x] Process state & thread inspector (`/proc/{pid}/status`)
- [x] Memory metrics collector (`/proc/meminfo`)
- [ ] Disk usage and I/O statistics (`/proc/diskstats`)
- [ ] Network interface bandwidth monitor (`/proc/net/dev`)
- [ ] Structured export formats (`--json`, `--yaml`, tabular view)
- [ ] Process management actions (graceful termination, niceness adjustment)

---

## Contributing

Contributions and pull requests are welcome!
1. Fork the repository
2. Create your feature branch (`git checkout -b feature/disk-collector`)
3. Commit your changes (`git commit -m 'Add disk metrics collector'`)
4. Push to the branch (`git push origin feature/disk-collector`)
5. Open a Pull Request

---

## License

This project is licensed under the terms of the GNU General Public License v3.0 ([LICENSE](LICENSE)).
