use crate::collectors_interface::interface::Collector;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub name: String,
    pub umask: String,
    pub state: String,
    pub pid: String,
    pub threads: u16,
    pub cpu_allowed: String,
    pub loginuid: String,
}

impl ProcessInfo {
    pub fn new() -> Self {
        Self {
            name: String::from(""),
            umask: String::from(""),
            state: String::from(""),
            pid: String::from(""),
            threads: 0,
            cpu_allowed: String::from(""),
            loginuid: String::from(""),
        }
    }
}

pub struct ProcessInfoCollector {
    pub pid: u32,
    pub proccess_info: ProcessInfo,
    pub found: bool,
}

impl Collector for ProcessInfoCollector {
    fn collect(&mut self) {
        let dir = format!("/proc/{}", self.pid);

        if !Path::new(&dir).exists() {
            self.found = false;
            return;
        }

        self.found = true;
        let status_path = format!("{dir}/status");
        self.parse_state(&status_path);
    }
}

impl ProcessInfoCollector {
    pub fn new(pid: &u32) -> Self {
        Self {
            pid: *pid,
            proccess_info: ProcessInfo::new(),
            found: false,
        }
    }

    pub fn get_info(&self) -> ProcessInfo {
        self.proccess_info.clone()
    }

    fn parse_state(&mut self, file: &str) {
        let content = match fs::read_to_string(file) {
            Ok(c) => c.replace("\t", ""),
            Err(_) => return,
        };

        for line in content.lines() {
            if let Some((left, right)) = line.split_once(':') {
                let key = left.trim();
                let val = right.trim();

                match key {
                    "Name" => self.proccess_info.name = val.to_string(),
                    "State" => self.proccess_info.state = val.to_string(),
                    "Umask" => self.proccess_info.umask = val.to_string(),
                    "Pid" => self.proccess_info.pid = val.to_string(),
                    "Threads" => self.proccess_info.threads = val.parse().unwrap_or(0),
                    "Cpus_allowed_list" => self.proccess_info.cpu_allowed = val.to_string(),
                    "Cpus_allowed" => {
                        if self.proccess_info.cpu_allowed.is_empty() {
                            self.proccess_info.cpu_allowed = val.to_string();
                        }
                    }
                    "loginuid" => self.proccess_info.loginuid = val.to_string(),
                    _ => {}
                }
            }
        }
    }

    /// Prints a clean, formatted card to the console
    pub fn print_summary(&self) {
        println!("╭──────────────────────── Process Telemetry ────────────────────────╮");
        if !self.found {
            println!("│ {:<64} │", format!("Error             : Process PID {} was not found", self.pid));
        } else {
            let info = &self.proccess_info;
            let display_pid = if info.pid.is_empty() { format!("{}", self.pid) } else { info.pid.clone() };
            println!("│ {:<64} │", format!("PID               : {}", display_pid));
            println!("│ {:<64} │", format!("Process Name      : {}", info.name));
            println!("│ {:<64} │", format!("State             : {}", info.state));
            println!("│ {:<64} │", format!("Threads           : {}", info.threads));
            println!("│ {:<64} │", format!("Umask             : {}", info.umask));
            println!("│ {:<64} │", format!("CPU Affinity      : {}", if info.cpu_allowed.is_empty() { "N/A" } else { &info.cpu_allowed }));
            println!("│ {:<64} │", format!("Login UID         : {}", if info.loginuid.is_empty() { "N/A" } else { &info.loginuid }));
        }
        println!("╰──────────────────────────────────────────────────────────────────╯");
    }
}
