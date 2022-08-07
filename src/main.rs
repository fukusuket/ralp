use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::time::Instant;

use chrono::DateTime;
use csv::{QuoteStyle, WriterBuilder};
use simple_logger::SimpleLogger;

use crate::parser::{extract, parse_request_first_line, until_space};

mod parser;

fn main() -> std::io::Result<()> {
    SimpleLogger::new().init().unwrap();
    log::info!("started.");
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        log::error!("args not set.");
        panic!("please set access_log path.")
    }
    let access_log = match File::open(&args[1]) {
        Ok(f) => { f }
        Err(_) => {
            log::error!("failed to open[{}]", &args[1]);
            panic!("please set valid access_log path.")
        }
    };
    let start = Instant::now();
    let o = File::create("out.csv")?;
    log::info!("output [{:?}].", o);
    let mut wtr = WriterBuilder::new().quote_style(QuoteStyle::Always).from_writer(BufWriter::new(o));
    let _ = wtr.write_record(&["time(utc)", "time(local)", "remote_host", "http_method", "http_status", "bytes", "request_url", "referer", "user_agent", "http_version", "remote log name", "remote user"]);
    for (i, result) in BufReader::new(access_log).lines().enumerate() {
        let s = result?;
        let (s, ip) = until_space(&s).unwrap_or_default();
        let (s, u1) = until_space(&s[1..]).unwrap_or_default();
        let (s, u2) = until_space(&s[1..]).unwrap_or_default();
        let (s, t) = extract('[', &s[1..], ']').unwrap_or_default();
        let t = DateTime::parse_from_str(t, "%d/%b/%Y:%H:%M:%S %z").unwrap();
        let (s, req) = extract('"', &s[1..], '"').unwrap_or_default();
        let v: Vec<String> = req.split_whitespace().map(|w| w.parse().ok().unwrap_or_default()).collect();
        let (method, url, version) = parse_request_first_line(&v);
        let (s, status) = until_space(&s[1..]).unwrap_or_default();
        let (s, bytes) = until_space(&s[1..]).unwrap_or_default();
        let (s, referer) = extract('"', &s[1..], '"').unwrap_or_default();
        let (_, user_agent) = extract('"', &s[1..], '"').unwrap_or_default();
        let _ = wtr.write_record(&[&t.naive_utc().to_string(), &t.to_string(), ip, method, status, bytes, url, referer, user_agent, version, u1, u2]);
    }
    let _ = wtr.flush();
    log::info!("end, took[{:?}].", start.elapsed());
    Ok(())
}

