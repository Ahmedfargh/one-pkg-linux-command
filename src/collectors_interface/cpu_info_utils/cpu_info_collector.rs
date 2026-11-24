use crate::collectors_interface::interface::Collector;
use std::fs;
#[derive(Clone)]
pub struct CpuInfo{
   pub processor:u8,
   pub vendor_id:String,
   pub cpu_info:u8,
   pub model:String,
   pub model_name:String,
   pub cpu_core:u8,
   pub cache_size:f32,
}
impl CpuInfo{
    fn new()->Self{
        return Self{
                processor:0,
                vendor_id:String::from(""),
                cpu_info:0,
                model:String::from(""),
                model_name:String::from(""),
                cpu_core:0,
                cache_size:0.0,
            };
    }
    
}

pub struct CpuInfoCollector{
    pub cpu_info:CpuInfo
}
impl Collector for CpuInfoCollector{
    fn collect(&mut self){
        let content = fs::read_to_string("/proc/cpuinfo").expect("should have ability to read the file").replace("\t", "");
        let lines =content.split("\n");
        let info : Vec<&str> = lines.collect();
        for item in info{
            if let Some((left, right)) = item.split_once(':'){
                if left.eq("processor"){
                    self.cpu_info.processor=(self.string_to_integer(&right))+1;
                }
                if left.eq("cpu cores"){
                    self.cpu_info.cpu_core=self.string_to_integer(&right);
                }
                if left.eq("cache size"){
                    self.cpu_info.cache_size=self.string_to_f16(&right);
                }
            }
        }
    }
    
}
impl CpuInfoCollector{
    pub fn new()->Self{
        return Self{
            cpu_info:CpuInfo::new(),
        };
    }
    pub fn get_info(&self)->CpuInfo{
        return self.cpu_info.clone();
    }
    fn string_to_integer(&self,value:&str)->u8{
        return value.trim().parse().unwrap_or(0);
    }
    fn string_to_f16(&self,value:&str)->f32{
        let value:f32=value.trim().parse().unwrap_or(0.0);
        return value;
    }
}