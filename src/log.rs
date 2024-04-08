use super::config::Config as Configuration;
use anyhow::Result;
use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use serde::{Deserialize, Serialize};
use std::env;

pub fn set_logger(config: &Configuration) -> Result<()> {
    let home_path = env::var("HOME").expect("An error occured while reading $HOME.");

    let pattern = if let LogLevel::DEBUG = config.log_level {
        "[{d(%Y-%m-%d %H:%M:%S)} {l} {f}:{L}] {m}{n}"
    } else {
        "[{d(%Y-%m-%d %H:%M:%S)} {l}] {m}{n}"
    };

    let log_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build(format!("{}/.local/share/dbv/logs", home_path))?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(log_file)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(config.log_level.into()),
        )?;

    log4rs::init_config(config)?;

    Ok(())
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    OFF,
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

impl LogLevel {
    pub fn default() -> Self {
        LogLevel::INFO
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<LogLevel> for LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::OFF => LevelFilter::Off,
            LogLevel::TRACE => LevelFilter::Trace,
            LogLevel::DEBUG => LevelFilter::Debug,
            LogLevel::INFO => LevelFilter::Info,
            LogLevel::WARN => LevelFilter::Warn,
            LogLevel::ERROR => LevelFilter::Error,
        }
    }
}
impl Into<LogLevel> for LevelFilter {
    fn into(self) -> LogLevel {
        match self {
            LevelFilter::Off => LogLevel::OFF,
            LevelFilter::Trace => LogLevel::TRACE,
            LevelFilter::Debug => LogLevel::DEBUG,
            LevelFilter::Info => LogLevel::INFO,
            LevelFilter::Warn => LogLevel::WARN,
            LevelFilter::Error => LogLevel::ERROR,
        }
    }
}
