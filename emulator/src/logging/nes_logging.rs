// Initialize log4rs for logging into logs/nes.log file

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;

pub fn init_logging() {
    let current_time = chrono::Local::now().format("%d%m%Y_%H%M%S_%3f").to_string();
    let log_file = format!("logs/nes_{}.log", current_time);

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%H:%M:%S.%3f)} | {l} | {M} | {m}{n}",
        )))
        .build(log_file)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .logger(
            Logger::builder()
                .appender("logfile")
                .build("nes", LevelFilter::Debug),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Debug),
        )
        .unwrap();

    match log4rs::init_config(config) {
        Ok(_) => (),
        Err(e) => {
            panic!("Error initializing log4rs: {}", e);
        }
    }
}
