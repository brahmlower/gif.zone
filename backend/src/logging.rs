// -----------------------------------------------------------------------------
use std::fmt::Arguments;
use std::io::stdout;
// -----------------------------------------------------------------------------
use fern::Dispatch;
use fern::FormatCallback;
use fern::InitError;
use log::Level;
use log::LevelFilter;
use log::Record;
use serde_json::to_string;
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------

pub fn setup_logger() -> Result<(), InitError> {
    Dispatch::new()
        .format(log_formatter)
        .level(LevelFilter::Debug)
        .chain(stdout())
        .apply()?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct AppLog<'a, T> {
    level: Level,
    target: &'a str,
    event: T,
}

/// This log formatter builds an AppLog struct, which is then serialised to
/// JSON, then logged. This Provides a foundation for structured logging.
/// Most logged content is just a string, but a struct can be logged as well
fn log_formatter(out: FormatCallback, message: &Arguments, record: &Record) {
    let s = format!("{}", message);
    let al = AppLog {
        level: record.level(),
        target: record.target(),
        event: s,
    };
    let json_log = match to_string(&al) {
        Ok(x) => x,
        Err(_) => "".to_owned(),
    };

    out.finish(format_args!("{}", json_log))
}
