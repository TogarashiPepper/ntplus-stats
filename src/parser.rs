use std::{
    backtrace::Backtrace,
    fmt::Debug,
    net::{AddrParseError, Ipv4Addr},
    num::ParseIntError,
    str::{Chars, FromStr},
};

use crate::logitem::{LogItem, Status};
use jiff::civil::date;
use thiserror::Error;

fn split_map<T>(inp: &str, sep: char) -> Result<Vec<T>, LogItemParseErr>
where
    T: FromStr<Err = ParseIntError>,
{
    inp.split(sep).map(|s| Ok(s.parse::<T>()?)).collect()
}

#[derive(Error, Debug)]
pub enum LogItemParseErr {
    #[error("The input was malformed")]
    InputInvalid,

    #[error("Status `{0}` is invalid, expected 'started', 'finished', or 'aborted'")]
    InvalidStatus(String),

    #[error("Input unexpectedly terminated")]
    UnexpctedEof,

    #[error("Error parsing IP address: {0:?}")]
    Ipv4ParseErr(#[from] AddrParseError),

    #[error("Error parsing port: {0:?}")]
    PortParseError(#[from] ParseIntError),
}

pub struct LogParseErr(LogItemParseErr, Backtrace);

impl Debug for LogParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.0)?;
        writeln!(f, "{}", self.1)
    }
}

impl From<LogItemParseErr> for LogParseErr {
    fn from(value: LogItemParseErr) -> Self {
        LogParseErr(value, Backtrace::capture())
    }
}

fn parse_con<'a>(mut iter: &mut Chars<'a>) -> Result<Option<(Ipv4Addr, u16)>, LogItemParseErr> {
    let l = iter.nth(12).ok_or(LogItemParseErr::UnexpctedEof)?;

    let conn = if l == '(' {
        let _ = iter.next().ok_or(LogItemParseErr::UnexpctedEof)?;

        let ip = (&mut iter)
            .take_while(|c| *c != '\'')
            .collect::<Box<str>>()
            .parse::<Ipv4Addr>()?;

        let _ = iter.nth(1).ok_or(LogItemParseErr::UnexpctedEof)?;

        let port = (&mut iter)
            .take_while(|e| *e != ')')
            .collect::<Box<str>>()
            .parse::<u16>()?;

        Some((ip, port))
    } else {
        let _ = iter.nth(4).ok_or(LogItemParseErr::UnexpctedEof)?;
        None
    };

    Ok(conn)
}

impl FromStr for LogItem {
    type Err = LogParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date_strs: Vec<i16> = split_map(&s[1..10], '/')?;

        let time_strs: Vec<i8> = split_map(&s[11..19], ':')?;

        let dtime = date(date_strs[2], date_strs[0] as i8, date_strs[1] as i8).at(
            time_strs[0],
            time_strs[1],
            time_strs[2],
            0,
        );

        if &s[20..26] != "Upload" {
            return Err(LogItemParseErr::InputInvalid.into());
        }

        let mut iter = s[27..].chars();

        let mut skip_ip = false;
        let status = (&mut iter).take_while(|e| {
            if *e == ',' {
                skip_ip = true
            }
            *e != ':' && *e != ','
        });


        // TODO: would benefit from deref patterns
        let status = match &*status.collect::<Box<str>>() {
            "started" => Status::Started,
            "finished" => Status::Finished,
            "aborted" => Status::Aborted,

            invalid => Err(LogItemParseErr::InvalidStatus(invalid.to_owned()))?,
        };

        let _ = iter.nth(5).ok_or(LogItemParseErr::UnexpctedEof)?;

        let user = (&mut iter)
            .take_while(|e| {
                if skip_ip {
                    *e != ' '
                }
                else {
                    *e != ','
                }
            })
            .collect::<Box<str>>();

        let conn = if !skip_ip {
            parse_con(&mut iter)?
        } else {
            None
        };

        let _ = (&mut iter).take_while(|c| *c != ' ').count();
        if !skip_ip {
            let _ = iter.nth(4).unwrap();
        }

        let file = iter.collect::<Box<str>>();

        Ok(LogItem {
            time: dtime,
            status,
            conn,
            user,
            file,
        })
    }
}
