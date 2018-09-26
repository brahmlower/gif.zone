
// -----------------------------------------------------------------------------
use std::io::stdout;
use std::fmt::Arguments;
// -----------------------------------------------------------------------------
use fern::Dispatch;
use fern::InitError;
use fern::FormatCallback;
use log::LevelFilter;
use log::Record;
use log::Level;
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
pub struct LogEvent<'a> {
    message: &'a str
}

#[derive(Debug, Serialize, Deserialize)]
struct AppLog<'a> {
    level: Level,
    target: &'a str,
    event: String
}

fn log_formatter(out: FormatCallback, message: &Arguments, record: &Record) {
    let s = format!("{}", message);
    let al = AppLog {
        level:  record.level(),
        target: record.target(),
        event:  s
    };
    let json_log = match to_string(&al) {
        Ok(x)   => x,
        Err(_)  => "".to_owned(),
    };

    out.finish(format_args!("{}", json_log))
}
