use crate::collectors_interface::interface::Collector;
use std::path::Path;
pub struct ProcessInfo{
    pub name:String,
    pub umask:String,
    pub pid:String,
    pub threads:u16,
    pub cpu_allowed:String,
    pub loginuid:String,
    
}
pub struct ProcessInfoCollector{
    pid :u16
}
impl Collector for ProcessInfoCollector{
    fn collect(&mut self){
        let dir=format!("/proc/{}",self.pid);

        let exists=Path::new(&dir).exists();
        if exists{
            println!("process is not found");
            return 
        }


    }
    
}
impl ProcessInfoCollector{
    pub fn new(pid:&u16)->Self{
        return ProcessInfoCollector{
            pid:*pid
        }
    }
    fn parse_status(){

    }
}