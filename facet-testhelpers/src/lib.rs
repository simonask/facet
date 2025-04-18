use facet_ansi::{ColorStyle, Style, Stylize};
use log::{Level, LevelFilter, Log, Metadata, Record};
use std::io::Write;

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        // Create style based on log level
        let level_style = match record.level() {
            Level::Error => Style::new().with_red(),
            Level::Warn => Style::new().with_yellow(),
            Level::Info => Style::new().with_green(),
            Level::Debug => Style::new().with_cyan(),
            Level::Trace => Style::new().dimmed(),
        };

        // Convert level to styled display
        let styled_level = record.level().style(level_style);

        eprintln!(
            "{} - {}: {}",
            styled_level,
            record.target().blue(),
            record.args()
        );
    }

    fn flush(&self) {
        let _ = std::io::stderr().flush();
    }
}

pub fn setup() {
    #[cfg(not(miri))]
    color_backtrace::install();
    let logger = Box::new(SimpleLogger);
    _ = log::set_boxed_logger(logger);
    log::set_max_level(LevelFilter::Trace);
}
