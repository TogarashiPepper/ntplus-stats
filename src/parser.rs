use std::{fmt::Debug, net::Ipv4Addr, str::FromStr};

use jiff::civil::date;

use crate::logitem::{LogItem, Status};

fn split_map<T>(inp: &str, sep: char) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    inp.split(sep).map(|s| s.parse::<T>().unwrap()).collect()
}

#[derive(Debug)]
pub enum LogItemParseErr {}

impl FromStr for LogItem {
    type Err = LogItemParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date_strs: Vec<i16> = split_map(&s[1..10], '/');

        let time_strs: Vec<i8> = split_map(&s[11..19], ':');

        let dtime = date(date_strs[2], date_strs[0] as i8, date_strs[1] as i8).at(
            time_strs[0],
            time_strs[1],
            time_strs[2],
            0,
        );

        assert_eq!(&s[20..26], "Upload");

        let mut iter = s[27..].chars();
        let start = 27;
        let mut end = start;

        for c in &mut iter {
            if c == ':' {
                break;
            }

            end += 1;
        }

        let status = match &s[start..end] {
            "started" => Status::Started,
            "finished" => Status::Finished,
            "aborted" => Status::Aborted,

            // TODO: proper error
            _ => panic!(),
        };

        let _ = iter.nth(6).unwrap();

        let start = end + 7;
        let mut end = start;

        for c in &mut iter {
            if c == ',' {
                break;
            }

            end += 1;
        }

        let user = s[start..end].to_owned();

        let l = iter.nth(12).unwrap();
        let start = end + 16;
        let mut end = start;

        let conn = if l == '(' {
            let _ = iter.next().unwrap();

            for c in &mut iter {
                if c == '\'' {
                    break;
                }

                end += 1;
            }

            let ip = s[start..end].parse::<Ipv4Addr>().unwrap();

            let start = end + 3;
            end = start;

            for c in &mut iter {
                if c == ')' {
                    break;
                }

                end += 1;
            }

            let port = s[start..end - 2].parse().unwrap();

            Some((ip, port))
        } else {
            let _ = iter.nth(4).unwrap();
            None
        };

        let len_skipped = (&mut iter).take_while(|c| *c != ' ').count();

        let file = s[end + len_skipped + 5..].to_owned();

        Ok(LogItem {
            time: dtime,
            status,
            conn,
            user,
            file,
        })
    }
}
