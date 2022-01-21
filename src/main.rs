use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use nom::bytes::complete::{is_not, take_until};
use nom::character::complete::char;
use nom::IResult;
use nom::sequence::delimited;

fn main() {
    let f = File::create("out.csv").unwrap();
    let mut wtr = csv::Writer::from_writer(BufWriter::new(f));
    for result in BufReader::new(File::open("access_log").unwrap()).lines() {
        let s = result.unwrap();
        let (s, ip) = until_space(&s).unwrap_or_default();
        let (s, u1) = until_space(&s[1..]).unwrap_or_default();
        let (s, u2) = until_space(&s[1..]).unwrap_or_default();
        let (s, timestamp) = pair('[', &s[1..], ']').unwrap_or_default();
        let (s, url) = pair('"', &s[1..], '"').unwrap_or_default();
        let _: Vec<String> = url.split_whitespace().map(|w| w.parse().ok().unwrap_or_default()).collect();
        let (s, http_status) = until_space(&s[1..]).unwrap_or_default();
        let (s, bytes) = until_space(&s[1..]).unwrap_or_default();
        let (s, referer) = pair('"', &s[1..], '"').unwrap_or_default();
        let (_, user_agent) = pair('"', &s[1..], '"').unwrap_or_default();
        println!("{}, {}, {}, {}, {}, {}, {}, {}, {}", ip, u1, u2, timestamp, url, http_status, bytes, referer, user_agent);
        let _ = wtr.write_record(&[ip, u1, u2, timestamp, url, http_status, bytes, referer, user_agent]);
    }
    wtr.flush();
}

fn pair(c1: char, input:&str, c2:char) -> IResult<&str,&str> {
    delimited(char(c1),
              is_not(String::from(c2).as_str()),
              char(c2))(input)
}

fn until_space(input:&str) -> IResult<&str,&str> {
    take_until(" ")(input)
}