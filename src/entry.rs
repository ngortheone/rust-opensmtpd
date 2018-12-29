use crate::errors::Error;
use nom::{alt, call, cond, do_parse, error_position, map_res, named, tag, take_until, take_while};

#[derive(Debug, PartialEq)]
pub enum Kind {
    Report,
    Filter,
}

#[derive(Debug)]
pub enum Subsystem {
    SmtpIn,
}

#[derive(Debug)]
pub enum Event {
    TxMail,
    TxRcpt,
}

#[derive(Debug)]
pub struct Entry {
    pub kind: Kind,
    pub version: u8,
    pub timestamp: u64,
    pub subsystem: Subsystem,
    pub event: Event,
    pub token: Option<u64>,
    pub session_id: u64,
    pub params: String,
}

fn is_ascii_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_ascii_hexdigit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn to_u8(s: &str) -> Result<u8, std::num::ParseIntError> {
    s.parse()
}

fn to_u64(s: &str) -> Result<u64, std::num::ParseIntError> {
    s.parse()
}

fn to_u64_hex(s: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(s, 16)
}

named!(parse_kind<&str, Kind>,
    alt!(
        tag!("report") => { |_| Kind::Report } |
        tag!("filter") => { |_| Kind::Filter }
    )
);

named!(parse_subsystem<&str, Subsystem>,
    alt! (
        tag!("smtp-in") => { |_| Subsystem::SmtpIn }
    )
);

named!(parse_event<&str, Event>,
    alt!(
        tag!("tx-mail") => { |_| Event::TxMail } |
        tag!("tx-rcpt") => { |_| Event::TxRcpt }
    )
);

named!(parse_version<&str, u8>,
    map_res!(take_while!(is_ascii_digit), to_u8)
);

named!(parse_u64<&str, u64>,
    map_res!(take_while!(is_ascii_digit), to_u64)
);

named!(parse_u64_hex<&str, u64>,
    map_res!(take_while!(is_ascii_hexdigit), to_u64_hex)
);

named!(parse_token<&str, u64>,
    do_parse!(
        token: parse_u64_hex >>
        tag!("|") >>
        (token)
    )
);

named!(
    parse_entry<&str, Entry>,
    do_parse!(
        kind: parse_kind >>
        tag!("|") >>
        version: parse_version >>
        tag!("|") >>
        timestamp: parse_u64 >>
        tag!("|") >>
        subsystem: parse_subsystem >>
        tag!("|") >>
        event: parse_event >>
        tag!("|") >>
        token: cond!(kind == Kind::Filter, parse_token) >>
        session_id: parse_u64_hex >>
        tag!("|") >>
        params: take_until!("\n") >>
        (Entry {
            kind,
            version,
            timestamp,
            subsystem,
            event,
            token,
            session_id,
            params: params.to_string(),
        })
    )
);

impl Entry {
    pub fn from_str(entry: &str) -> Result<Entry, Error> {
        let (_, res) = parse_entry(entry)?;
        Ok(res)
    }
}
