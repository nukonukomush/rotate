use chrono::prelude::*;
use chrono::Duration;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::LineWriter;

pub fn exec<T>(
    mut input: T,
    format: &str,
    base: DateTime<Local>,
    duration: Duration,
) -> Result<(), std::io::Error>
where
    T: BufRead,
{
    let mut end = next_end(base, duration);
    let start = end - duration;
    let mut file = open_file(format, start)?;
    let mut buf = String::new();
    loop {
        let len = input.read_line(&mut buf)?;
        if len == 0 {
            break;
        }
        if Local::now() >= end {
            end = next_end(base, duration);
            let start = end - duration;
            file = open_file(format, start)?;
        }
        file.write_all(buf.as_bytes())?;
        buf.clear();
    }
    file.flush()?;

    Ok(())
}

pub fn div_duration(lhs: Duration, rhs: Duration) -> f64 {
    lhs.num_milliseconds() as f64 / rhs.num_milliseconds() as f64
}

pub fn open_file(format: &str, start: DateTime<Local>) -> Result<impl Write, std::io::Error> {
    let filename = start.format(format).to_string();
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;
    Ok(LineWriter::new(file))
}

pub fn next_end(base: DateTime<Local>, duration: Duration) -> DateTime<Local> {
    base + duration * div_duration(Local::now() - base, duration).ceil() as i32
}
