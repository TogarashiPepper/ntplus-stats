// TODO: visalization w/web ui w/axum
// TODO: parallelize parsing log files
// TODO: parallelize reading log files
// TODO: parallelize reading music files
// TODO: async IO perhaps

mod logitem;
mod metadata;
mod parser;

use std::{collections::HashMap, fs, io::stdin, path::PathBuf};

use logitem::{LogItem, Status};
use metadata::get_meta;
use parser::LogParseErr;

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

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
                let item = match line.parse::<LogItem>() {
                    Ok(item) => item,
                    Err(err) => return Some(Err(err)),
                };

                if item.status != Status::Finished
                    || (item.file.ends_with(".jpg")
                        || item.file.ends_with(".jpeg")
                        || item.file.ends_with(".txt")
                        || item.file.ends_with(".cue")
                        || item.file.ends_with(".lrc")
                        || item.file.ends_with(".log")
                        || item.file.ends_with(".png")
                        || item.file.ends_with(".m3u")
                        || item.file.ends_with(".wav"))
                {
                    return None;
                }

                Some(Ok(item))
            })
            .collect();

        data.entry(name).or_default().extend(log_items.unwrap());
    }

    let mut freq_map = HashMap::new();

    for (_k, v) in data {
        for mut item in v {
            if let Some(x) = item.file.strip_prefix("downloads\\") {
                item.file = format!("music/{x}").into();
            }
            // if let Some(x) = item.file.strip_prefix("music\\") {
            //     item.file = x.into();
            // }

            freq_map
                .entry(item.file)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }

    // TODO: filter files that dont exist
    let mut map: Vec<(Box<str>, usize)> = Vec::from_iter(freq_map);

    map.sort_by(|&(_, a), &(_, b)| a.cmp(&b));

    let mut void = String::new();
    let stdin = stdin();

    clear();

    for elem in map.into_iter().rev() {
        let mut path = PathBuf::from("/Volumes/OrangDrive/");
        path.extend(elem.0.split('\\'));

        if path.exists() {
            let d = get_meta(path);
            println!("{d}");

            // stdin.read_line(&mut void).unwrap();
            // clear();
        }
    }
}
