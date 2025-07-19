mod logitem;
mod parser;

use std::{collections::HashMap, fs};

use logitem::{LogItem, Status};
use parser::LogParseErr;

fn main() {
    let mut data: HashMap<String, Vec<LogItem>> = HashMap::new();
    let mut dir = std::env::home_dir().unwrap();

    dir.push(".local");
    dir.push("share");
    dir.push("nicotine");
    dir.push("logs");
    dir.push("transfers");

    let dir_iter = fs::read_dir(dir).unwrap().filter_map(|file| {
        let dir_entry = file.unwrap();
        let fname = dir_entry
            .file_name()
            .into_string()
            .expect("Non-UTF8 file paths are not allowed");
        let ftype = dir_entry.file_type().unwrap();

        if fname.starts_with("uploads_") && ftype.is_file() {
            let contents = fs::read_to_string(dir_entry.path()).unwrap();

            Some((fname[8..=17].to_owned(), contents))
        } else {
            None
        }
    });

    for (name, content) in dir_iter {
        let log_items: Result<Vec<LogItem>, LogParseErr> = content
            .lines()
            .filter_map(|line| {
                let item = line.parse::<LogItem>();
                if let Ok(item) = &item
                    && item.status != Status::Finished
                {
                    return None;
                }

                Some(item)
            })
            .collect();

        data.entry(name).or_default().extend(log_items.unwrap());
    }

    let mut freq_map = HashMap::new();

    for (_k, v) in data {
        for mut item in v {
            if let Some(x) = item.file.strip_prefix("downloads\\") {
                item.file = x.into();
            }
            if let Some(x) = item.file.strip_prefix("music\\") {
                item.file = x.into();
            }

            freq_map
                .entry(item.file)
                .and_modify(|e| *e += 1)
                .or_insert(0);
        }
    }

    let mut map: Vec<(Box<str>, usize)> = Vec::from_iter(freq_map);

    map.sort_by(|&(_, a), &(_, b)| a.cmp(&b));

    for x in map {
        println!("{}: {}", x.0, x.1);
    }
}
