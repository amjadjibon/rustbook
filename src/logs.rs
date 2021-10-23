use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::json::JsonEncoder,
};

use crate::conf::config;

fn level() -> LevelFilter{
    let l = config().log_level;
    if l == "off".to_string() {
        LevelFilter::Off
    } else if l == "error".to_string() {
        LevelFilter::Error
    } else if l == "warn".to_string() {
        LevelFilter::Warn
    } else if l == "info".to_string() {
        LevelFilter::Info
    } else if l == "debug".to_string() {
        LevelFilter::Debug
    } else if l == "trace".to_string() {
        LevelFilter::Trace
    } else {
        LevelFilter::Debug
    }
}

pub fn init_logger() {
    let stdout: ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(JsonEncoder::new()))
        .build();
    let log_config = log4rs::config::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(level())).unwrap();
        
    log4rs::init_config(log_config).unwrap();
}