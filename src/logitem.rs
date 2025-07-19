use std::{fmt::Debug, net::Ipv4Addr};

use jiff::civil::DateTime;

#[derive(Debug, PartialEq)]
pub enum Status {
    Started,
    Finished,
    Aborted,
}

pub struct LogItem {
    pub time: DateTime,
    pub status: Status,
    pub conn: Option<(Ipv4Addr, u16)>,
    pub user: Box<str>,
    pub file: Box<str>,
}

impl Debug for LogItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value: String = match self.conn {
            Some(x) => format!("{x:?}"),
            None => "None".to_owned(),
        };

        let d_str = format!(
            "{}/{}/{} {:0>2}:{:0>2}",
            self.time.month(),
            self.time.day(),
            self.time.year(),
            self.time.hour(),
            self.time.minute()
        );

        f.debug_struct("LogItem")
            .field("time", &format_args!("{d_str}"))
            .field("status", &self.status)
            .field("conn", &format_args!("{value}"))
            .field("user", &format_args!("{}", self.user))
            .field("file", &format_args!("{}", self.file))
            .finish()
    }
}
