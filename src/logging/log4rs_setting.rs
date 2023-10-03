use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    Config, Handle,
};

pub fn init_log4rs_setting() -> Handle {
    let stdout = ConsoleAppender::builder().build();

    let logging_level = if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Error
    };

    let config = match Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(logging_level))
    {
        Ok(con) => con,
        Err(e) => panic!("{:?}", e),
    };

    match log4rs::init_config(config) {
        Ok(handle) => handle,
        Err(e) => panic!("{:?}", e),
    }
}
