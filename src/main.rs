use chrono::prelude::*;
use chrono::Duration;
use clap::{App, Arg};
use std::io::BufReader;

mod core;
use crate::core::*;

fn main() -> Result<(), ()> {
    let now = Local::now();
    let default_base = Local
        .ymd(now.year(), now.month(), now.day())
        .and_hms(0, 0, 0)
        .to_rfc3339();
    let app = App::new("Rotate Logs")
        .version("0.1.0")
        .arg(Arg::new("format").required(true))
        .arg(Arg::new("duration").short('d').default_value("1d"))
        .arg(
            Arg::new("base")
                .short('b')
                .default_value(default_base.as_str()),
        );

    let matches = app.get_matches();
    let format = matches.value_of("format").expect("format is required arg");
    let base = matches.value_of("base").expect("base is required arg");
    let base = DateTime::parse_from_rfc3339(base).map_err(|e| eprintln!("{}", e))?;
    let duration = matches
        .value_of("duration")
        .expect("duration is required arg");
    let duration = humantime::parse_duration(duration).map_err(|e| eprintln!("{}", e))?;
    let duration = Duration::from_std(duration).map_err(|e| eprintln!("{}", e))?;

    let stdin = BufReader::new(std::io::stdin());

    exec(stdin, format, base.with_timezone(&Local), duration).map_err(|e| {
        eprintln!("{}", e);
    })?;

    Ok(())
}
