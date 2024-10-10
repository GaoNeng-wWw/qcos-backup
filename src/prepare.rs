extern crate simplelog;
use std::fs::File;
use simplelog::*;

pub fn run(){
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Debug, Config::default(), File::create("log.log").unwrap()),
        ]
    ).unwrap();
}