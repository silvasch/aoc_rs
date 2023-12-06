use clap::Parser;
use colored::{ColoredString, Colorize};

use crate::args::Args;

pub struct Logger;

pub enum LogLevel {
    Info,
    Success,
    Warn,
    Error,
}

impl Logger {
    pub fn log(msg: &str, lvl: LogLevel) {
        if Args::parse().hide_log {
            return;
        }
        println!("{}", Logger::colorize(msg, lvl));
    }

    pub(crate) fn lib_log(msg: &str, lvl: LogLevel) {
        println!(">> {}", Logger::colorize(msg, lvl));
    }

    fn colorize(msg: &str, lvl: LogLevel) -> ColoredString {
        match lvl {
            LogLevel::Info => msg.into(),
            LogLevel::Success => msg.green(),
            LogLevel::Warn => msg.yellow(),
            LogLevel::Error => msg.red(),
        }
    }
}
