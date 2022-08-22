use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::PathBuf;
use std::time::Instant;

use chrono::DateTime;
use csv::{QuoteStyle, WriterBuilder};
use simple_logger::SimpleLogger;
use clap::{AppSettings, Parser};

use crate::parser::{extract, parse_request_first_line, until_space};

mod parser;

#[derive(Parser, Debug)]
#[clap(long_about = None)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
struct Args {
    /// Input Apache access log file.
    #[clap(short, long, value_parser)]
    access_log: PathBuf,

    /// Output csv file.
    #[clap(short, long, value_parser)]
    output_csv: PathBuf,
}

fn main() -> std::io::Result<()> {
    SimpleLogger::new().init().unwrap();
    log::info!("started.");
    let args: Args = Args::parse();
    let access_log = match File::open(&args.access_log) {
        Ok(f) => { f }
        Err(_) => {
            log::error!("failed to open[{:?}]", &args.access_log);
            panic!("please set valid access_log path.")
        }
    };
    let start = Instant::now();
    let o = match File::create(&args.output_csv) {
        Ok(f) => { f }
        Err(_) => {
            log::error!("failed to open[{:?}]", &args.output_csv);
            panic!("please set valid access_log path.")
        }
    };
    log::info!("output [{:?}].", o);
    let mut wtr = WriterBuilder::new().quote_style(QuoteStyle::Always).from_writer(BufWriter::new(o));
    let _ = wtr.write_record(&["time(utc)", "time(local)", "remote_host", "http_method", "http_status", "bytes", "request_url", "referer", "user_agent", "http_version", "remote log name", "remote user"]);
    for result in BufReader::new(access_log).lines() {
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

