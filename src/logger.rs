use colored::{ColoredString, Colorize};

pub struct Logger {
    active: bool,
}

pub enum LogLevel {
    Info,
    Success,
    Warn,
    Error,
}

impl Logger {
    pub(crate) fn new(active: bool) -> Self {
        Self { active }
    }

    pub fn log(&self, msg: &str, lvl: LogLevel) {
        if !self.active {
            return;
        }
        println!("{}", Logger::colorize(msg, lvl));
    }

    pub(crate) fn lib_log(&self, msg: &str, lvl: LogLevel) {
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
