mod parser;

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::time::Instant;

use chrono::DateTime;
use csv::{QuoteStyle, WriterBuilder};
use crate::parser::{parse_request_first_line, extract, until_space};


fn main() {
    let f = File::create("out.csv").unwrap();
    let mut wtr = WriterBuilder::new().quote_style(QuoteStyle::Always).from_writer(BufWriter::new(f));
    let rdr = BufReader::new(File::open("access_log").unwrap());
    let start = Instant::now();
    for (i, result) in rdr.lines().enumerate() {
        let s = result.unwrap();
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
        if i == 0 {
            let _ = wtr.write_record(&["time(utc)", "time(local)", "remote_host", "http_method", "http_status", "bytes", "request_url", "referer", "user_agent", "http_version", "remote log name", "remote user"]);
        }
        let _ = wtr.write_record(&[&t.naive_utc().to_string(), &t.to_string(), ip, method, status, bytes, url, referer, user_agent, version, u1, u2]);
    }
    let _ = wtr.flush();
    let end = start.elapsed();
    println!("{:?}", end);
}

