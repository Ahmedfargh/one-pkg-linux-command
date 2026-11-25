use crate::collectors_interface::interface::Collector;
use std::path::Path;
use std::fs::File;
use std::fs;
use std::string;
pub struct ProcessInfo{
    pub name:String,
    pub umask:String,
    pub pid:String,
    pub threads:u16,
    pub cpu_allowed:String,
    pub loginuid:String,

}
impl ProcessInfo{
    pub fn new()->Self{
        let process_info=ProcessInfo{
            name:string::String::from(""),
            umask:string::String::from(""),
            pid:string::String::from(""),
            threads:0,
            cpu_allowed:string::String::from(""),
            loginuid:string::String::from(""),
        };
        return process_info;
    }
}
pub struct ProcessInfoCollector{
    pid :u16,
    proccess_info:ProcessInfo,
}
impl Collector for ProcessInfoCollector{
    fn collect(&mut self){
        let dir=format!("/proc/{}",self.pid);

        let exists=Path::new(&dir).exists();
        if exists{
            println!("process is not found");
            return 
        }
        let stat_file=File::open(format!("{dir}/status"));
        if stat_file.is_err(){
            println!("error in getting status file");
        }else{
            self.parse_state(&format!("{dir}/status"));
        }
    }
    
}
impl ProcessInfoCollector{
    pub fn new(pid:&u16)->Self{
        return ProcessInfoCollector{
            pid:*pid,
            proccess_info:ProcessInfo::new(),
        }
    }
    fn parse_state(&mut self,file:&str){
        let content=fs::read_to_string(file).expect("can't parse state files").replace("\t", "");
        let splits=content.split("\n");
        for line in splits{
            let qutes_split=line.split(":").next().expect("NO_CODE");
            let left=qutes_split.split(":").next().unwrap();
            let right=qutes_split.split(":").next().unwrap();
            if left.eq("Name"){
                self.proccess_info.name=string::String::from(right);
            }
            if left.eq("Umask"){
                self.proccess_info.umask=string::String::from(right);
            }
            if left.eq("Pid"){
                self.proccess_info.pid=right.parse().unwrap();
            }
            if left.eq("Threads"){
                self.proccess_info.threads=right.parse().unwrap();
            }
            if left.eq("cpu_allowed"){
                self.proccess_info.cpu_allowed=string::String::from(right);
            }
            if left.eq("loginuid"){
                self.proccess_info.loginuid=string::String::from(right);
            }
        }

    }
}