use nom::bytes::complete::{is_not, take_until};
use nom::character::complete::char;
use nom::IResult;
use nom::sequence::delimited;

pub fn extract(start: char, input: &str, end: char) -> IResult<&str, &str> {
    delimited(char(start),
              is_not(String::from(end).as_str()),
              char(end))(input)
}

pub fn until_space(input: &str) -> IResult<&str, &str> {
    take_until(" ")(input)
}

pub fn parse_request_first_line(v: &Vec<String>) -> (&str, &str, &str) {
    if v.len() == 3 { (&v[0], &v[1], &v[2]) } else { ("-", "-", "-") }
}


#[test]
fn test_extract() {
    let r = extract('[', "[ab]c", ']');
    assert_eq!(r.ok(), Some(("c","ab")));

    let r = extract('[', "abc", ']');
    assert_eq!(r.unwrap_or_default(), ("",""));
}
#[test]
fn test_extract_not_match() {
    let r = extract('[', "abc", ']');
    assert_eq!(r.unwrap_or_default(), ("",""));
}

#[test]
fn test_until_space() {
    let r = until_space("abc def");
    assert_eq!(r.ok(), Some((" def","abc")));
}
