use std::io::Result as IoResult;
use std::io::Write;

use env_logger::Builder;
use env_logger::Formatter;
use log::Record;

use time;

fn format(f: &mut Formatter, record: &Record) -> IoResult<()> {
    let t = time::now();

    if let Ok(time_string) = time::strftime("%Y-%m-%d %H:%M:%S", &t) {
        write!(
            f,
            "{}: {} [{}] - {}\n",
            time_string,
            record.level(),
            record.target(),
            record.args()
        )
    } else {
        write!(
            f,
            "????-??-?? ??:??:??: {} [{}] - {}\n",
            record.level(),
            record.target(),
            record.args()
        )
    }
}

pub fn init() {
    Builder::from_default_env().format(format).init();
}
