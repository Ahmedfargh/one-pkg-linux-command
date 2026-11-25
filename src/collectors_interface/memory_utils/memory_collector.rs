use crate::collectors_interface::interface::Collector;
use std::fs;

#[derive(Clone, Debug)]
pub struct MemoryInfo {
    pub total_memory: f32,
    pub free_memory: f32,
    pub ava_memory: f32,
    pub buffers: f32,
    pub cached: f32,
    pub swap_cached: f32,
}

impl MemoryInfo {
    pub fn new() -> Self {
        Self {
            total_memory: 0.0,
            free_memory: 0.0,
            ava_memory: 0.0,
            buffers: 0.0,
            cached: 0.0,
            swap_cached: 0.0,
        }
    }

    pub fn used_memory(&self) -> f32 {
        (self.total_memory - self.ava_memory).max(0.0)
    }

    pub fn used_percentage(&self) -> f32 {
        if self.total_memory > 0.0 {
            (self.used_memory() / self.total_memory) * 100.0
        } else {
            0.0
        }
    }
}

pub struct MemeoryCollector {
    pub memory_info: MemoryInfo,
}

pub type MemoryCollector = MemeoryCollector;

impl MemeoryCollector {
    pub fn new() -> Self {
        Self {
            memory_info: MemoryInfo::new(),
        }
    }

    pub fn get_info(&self) -> MemoryInfo {
        self.memory_info.clone()
    }

    pub fn to_giga_byte(&self, value: &str) -> f32 {
        let clean = value.trim().replace("kB", "");
        let number: f32 = clean.trim().parse().unwrap_or(0.0);
        number / (1024.0 * 1024.0)
    }

    pub fn to_mega_byte(&self, value: &str) -> f32 {
        let clean = value.trim().replace("kB", "");
        let number: f32 = clean.trim().parse().unwrap_or(0.0);
        number / 1024.0
    }

    pub fn print_summary(&self) {
        let total = self.memory_info.total_memory;
        let available = self.memory_info.ava_memory;
        let used = self.memory_info.used_memory();
        let used_pct = self.memory_info.used_percentage();
        let avail_pct = (100.0 - used_pct).max(0.0);
        let buffers_cache = self.memory_info.buffers + self.memory_info.cached;

        let total_blocks = 20;
        let filled_blocks = (((used_pct / 100.0) * total_blocks as f32).round() as usize).min(total_blocks);
        let empty_blocks = total_blocks - filled_blocks;
        let bar = format!("[{}{}]", "■".repeat(filled_blocks), "░".repeat(empty_blocks));

        println!("╭──────────────────────── Memory Telemetry ────────────────────────╮");
        println!("│ {:<64} │", format!("Total Memory      : {:>7.2} GB", total));
        println!("│ {:<64} │", format!("Used Memory       : {:>7.2} GB  ({:>5.1}%)", used, used_pct));
        println!("│ {:<64} │", format!("Available Memory  : {:>7.2} GB  ({:>5.1}%)", available, avail_pct));
        println!("│ {:<64} │", format!("Free Memory       : {:>7.2} GB", self.memory_info.free_memory));
        println!("│ {:<64} │", format!("Buffers & Cache   : {:>7.2} GB", buffers_cache));
        println!("│ {:<64} │", format!("Swap Cached       : {:>7.2} GB", self.memory_info.swap_cached));
        println!("│ {:<64} │", format!("Usage Status      : {} {:>5.1}%", bar, used_pct));
        println!("╰──────────────────────────────────────────────────────────────────╯");
    }
}

impl Collector for MemeoryCollector {
    fn collect(&mut self) {
        let path = "/proc/meminfo";
        let memory_content = match fs::read_to_string(path) {
            Ok(content) => content.replace("\t", ""),
            Err(err) => {
                eprintln!("can't get memory info: {}", err);
                return;
            }
        };

        for line in memory_content.lines() {
            if let Some((left, right)) = line.split_once(':') {
                let key = left.trim();
                let value = right.trim();

                match key {
                    "MemTotal" => self.memory_info.total_memory = self.to_giga_byte(value),
                    "MemFree" => self.memory_info.free_memory = self.to_giga_byte(value),
                    "MemAvailable" => self.memory_info.ava_memory = self.to_giga_byte(value),
                    "Buffers" => self.memory_info.buffers = self.to_giga_byte(value),
                    "Cached" => self.memory_info.cached = self.to_giga_byte(value),
                    "SwapCached" => self.memory_info.swap_cached = self.to_giga_byte(value),
                    _ => {}
                }
            }
        }
    }
}
