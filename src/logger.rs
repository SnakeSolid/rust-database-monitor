use std::env;
use log::LogRecord;
use log::LogLevelFilter;
use log::SetLoggerError;
use env_logger::LogBuilder;

use time;


fn format(record: &LogRecord) -> String {
    let t = time::now();

    if let Ok(time_string) = time::strftime("%Y-%m-%d %H:%M:%S", &t) {
        format!("{}: {} - {}", time_string, record.level(), record.args())
    } else {
        format!("????-??-?? ??:??:??: {} - {}",
                record.level(),
                record.args())
    }
}


pub fn init() -> Result<(), SetLoggerError> {
    let mut builder = LogBuilder::new();
    builder.format(format);
    builder.filter(None, LogLevelFilter::Info);

    if let Ok(rust_log) = env::var("RUST_LOG") {
        builder.parse(&rust_log);
    }

    builder.init()
}
