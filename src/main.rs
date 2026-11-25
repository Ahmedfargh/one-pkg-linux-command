use std::env;
pub mod system_args;
pub mod collectors_interface;
use crate::collectors_interface::{
    cpu_info_utils::cpu_info_collector::CpuInfoCollector,process_utils::process_info_collector,interface::Collector,
    memory_utils::memory_collector,
    
};
fn main() {
    let args:Vec<String> = env::args().collect();
    let mut system_args = system_args::args::ArgsV::new();
    let _args_clone:Vec<String>=args.clone();
    system_args.collect_args(args);
    if system_args.cpu_info{
        let mut cpu_collector=CpuInfoCollector::new();
        cpu_collector.collect();
        cpu_collector.print_summary();
    }
    if system_args.monitor_process{
        let pid =system_args.process_to_monitor;
        let mut process_collector=process_info_collector::ProcessInfoCollector::new(&pid);
        process_collector.collect();
        process_collector.print_summary();
    }
    if system_args.memory_info{
        let mut memory_collector=memory_collector::MemeoryCollector::new();
        memory_collector.collect();
        memory_collector.print_summary();
    }
}