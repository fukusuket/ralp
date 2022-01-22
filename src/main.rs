use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};

use chrono::DateTime;
use csv::{QuoteStyle, WriterBuilder};
use nom::bytes::complete::{is_not, take_until};
use nom::character::complete::char;
use nom::IResult;
use nom::sequence::delimited;

fn main() {
    let f = File::create("out.csv").unwrap();
    let mut wtr = WriterBuilder::new().quote_style(QuoteStyle::Always).from_writer(BufWriter::new(f));
    let _ = wtr.write_record(&["time(utc)", "time(local)", "remote_host", "http_method", "http_status", "bytes", "request_url", "referer", "user_agent", "http_version", "remote log name", "remote user"]);
    for result in BufReader::new(File::open("access_log").unwrap()).lines() {
        let s = result.unwrap();
        let (s, ip) = until_space(&s).unwrap_or_default();
        let (s, u1) = until_space(&s[1..]).unwrap_or_default();
        let (s, u2) = until_space(&s[1..]).unwrap_or_default();
        let (s, t) = pair('[', &s[1..], ']').unwrap_or_default();
        let t = DateTime::parse_from_str(t, "%d/%b/%Y:%H:%M:%S %z").unwrap();
        let (s, url) = pair('"', &s[1..], '"').unwrap_or_default();
        let v: Vec<String> = url.split_whitespace().map(|w| w.parse().ok().unwrap_or_default()).collect();
        let (method, url, version) = parse_request_first_line(&v);
        let (s, status) = until_space(&s[1..]).unwrap_or_default();
        let (s, bytes) = until_space(&s[1..]).unwrap_or_default();
        let (s, referer) = pair('"', &s[1..], '"').unwrap_or_default();
        let (_, user_agent) = pair('"', &s[1..], '"').unwrap_or_default();
        let _ = wtr.write_record(&[&t.naive_utc().to_string(), &t.to_string(), ip, method, status, bytes, url, referer, user_agent, version, u1, u2]);
    }
    let _ = wtr.flush();
}

fn pair(c1: char, input: &str, c2: char) -> IResult<&str, &str> {
    delimited(char(c1), is_not(String::from(c2).as_str()), char(c2))(input)
}

fn until_space(input: &str) -> IResult<&str, &str> {
    take_until(" ")(input)
}

fn parse_request_first_line(v: &Vec<String>) -> (&str, &str, &str) {
    if v.len() == 3 { (&v[0], &v[1], &v[2]) } else { ("-", "-", "-") }
}




