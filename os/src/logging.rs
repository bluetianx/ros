
use log::*;

struct SimpleLogger;

impl Log for SimpleLogger{
    fn enabled(&self,_metadata: &Metadata) -> bool{
        true
    }
    fn log(&self,record: &Record){
        if !self.enabled(record.metadata()){
            return ;
        }
        match record.level(){
            Level::Error => println!("\x1b[31m[{}][{}]\x1b[0m",record.level(),record.args()),
            Level::Warn =>  println!("\x1b[93m[{}][{}]\x1b[0m",record.level(),record.args()),
            Level::Info =>  println!("\x1b[34m[{}][{}]\x1b[0m",record.level(),record.args()),
            Level::Debug => println!("\x1b[32m[{}][{}]\x1b[0m",record.level(),record.args()),
            Level::Trace => println!("\x1b[90m[{}][{}]\x1b[0m",record.level(),record.args()),
        }
    }

    fn flush(&self){

    }
}
pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Trace);
}
