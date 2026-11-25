use crate::collectors_interface::interface::Collector;
use std::fs;

#[derive(Clone, Debug)]
pub struct CpuInfo {
    pub processor: u8,
    pub vendor_id: String,
    pub cpu_info: u8,
    pub model: String,
    pub model_name: String,
    pub cpu_core: u8,
    pub cache_size: f32,
}

impl CpuInfo {
    fn new() -> Self {
        Self {
            processor: 0,
            vendor_id: String::from(""),
            cpu_info: 0,
            model: String::from(""),
            model_name: String::from(""),
            cpu_core: 0,
            cache_size: 0.0,
        }
    }
}

pub struct CpuInfoCollector {
    pub cpu_info: CpuInfo,
}

impl Collector for CpuInfoCollector {
    fn collect(&mut self) {
        let content = match fs::read_to_string("/proc/cpuinfo") {
            Ok(c) => c.replace("\t", ""),
            Err(e) => {
                eprintln!("Error reading /proc/cpuinfo: {}", e);
                return;
            }
        };

        for item in content.lines() {
            if let Some((left, right)) = item.split_once(':') {
                let key = left.trim();
                let val = right.trim();

                match key {
                    "processor" => {
                        let id = self.string_to_integer(val);
                        if id + 1 > self.cpu_info.processor {
                            self.cpu_info.processor = id + 1;
                        }
                    }
                    "cpu cores" => {
                        self.cpu_info.cpu_core = self.string_to_integer(val);
                    }
                    "cache size" => {
                        self.cpu_info.cache_size = self.string_to_f16(val);
                    }
                    "model name" => {
                        if self.cpu_info.model_name.is_empty() {
                            self.cpu_info.model_name = val.to_string();
                        }
                    }
                    "vendor_id" => {
                        if self.cpu_info.vendor_id.is_empty() {
                            self.cpu_info.vendor_id = val.to_string();
                        }
                    }
                    "model" => {
                        if self.cpu_info.model.is_empty() {
                            self.cpu_info.model = val.to_string();
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

impl CpuInfoCollector {
    pub fn new() -> Self {
        Self {
            cpu_info: CpuInfo::new(),
        }
    }

    pub fn get_info(&self) -> CpuInfo {
        self.cpu_info.clone()
    }

    fn string_to_integer(&self, value: &str) -> u8 {
        value.trim().parse().unwrap_or(0)
    }

    fn string_to_f16(&self, value: &str) -> f32 {
        let clean = value.trim().replace("KB", "").replace("kB", "");
        clean.trim().parse().unwrap_or(0.0)
    }

    /// Prints a clean, formatted card to the console
    pub fn print_summary(&self) {
        let info = &self.cpu_info;
        let cache_mb = info.cache_size / 1024.0;
        let cache_str = if info.cache_size > 0.0 {
            format!("{:.2} KB ({:.2} MB)", info.cache_size, cache_mb)
        } else {
            "N/A".to_string()
        };

        println!("╭────────────────────────── CPU Telemetry ─────────────────────────╮");
        println!("│ {:<64} │", format!("Model Name        : {}", info.model_name));
        println!("│ {:<64} │", format!("Vendor ID         : {}", info.vendor_id));
        println!("│ {:<64} │", format!("Physical Cores    : {} cores", info.cpu_core));
        println!("│ {:<64} │", format!("Logical Processors: {} threads", info.processor));
        println!("│ {:<64} │", format!("Cache Size        : {}", cache_str));
        println!("╰──────────────────────────────────────────────────────────────────╯");
    }
}
