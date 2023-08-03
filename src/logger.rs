use chrono::Local;
use colored::{Color, Colorize};
use log::{set_logger, set_max_level, Level, LevelFilter, Log, Metadata, Record};

struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::max()
    }

    fn log(&self, record: &Record) {
        let level_color = match record.level() {
            Level::Info => Color::BrightCyan,
            Level::Warn => Color::BrightYellow,
            Level::Error => Color::BrightRed,
            Level::Debug => Color::BrightMagenta,
            Level::Trace => Color::BrightWhite,
        };

        let time = format!("[{}]", Local::now().format("%H:%M:%S")).bright_yellow();
        let level = format!("{}", record.level()).color(level_color);
        let message = record.args();
        println!("{} {} - {}", time, level, message);
    }

    fn flush(&self) {}
}

pub fn set_global_logger() -> Result<(), log::SetLoggerError> {
    set_logger(&Logger).map(|_| {
        if cfg!(debug_assertions) {
            set_max_level(LevelFilter::Debug)
        } else {
            set_max_level(LevelFilter::Info)
        }
    })?;

    Ok(())
}
