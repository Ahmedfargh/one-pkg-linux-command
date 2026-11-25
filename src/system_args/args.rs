
pub struct ArgsV {
    pub monitor_process: bool,
    pub process_to_monitor: u32,
    pub cpu_info:bool,
    pub memory_info:bool
}

impl ArgsV {
    pub fn new() -> Self {
        Self {
            monitor_process: false,
            process_to_monitor:1,
            cpu_info:false,
            memory_info:false,
        }
    }
    pub fn collect_args(&mut self,_args:Vec<String>) {
        for arg in &_args{ 
            if arg.eq("--cpu-info") || arg.eq("cpu_info"){
                self.cpu_info=true;
            }
            if arg.contains("="){
                let mut split=arg.split("=");
                let _current_arg=arg.split("=").next().expect("Could not find a '='");
                let _left = split.next().unwrap();
                let _right = split.next().unwrap();
                if _left.eq("monitor_process"){
                    self.monitor_process=true;
                    self.process_to_monitor=_right.parse().unwrap_or(0);
                }
                
            }
            if arg.eq("--memory-info") || arg.eq("--memory_info") || arg.eq("memory_info") {
                self.memory_info=true;
            }
        }
    }
}