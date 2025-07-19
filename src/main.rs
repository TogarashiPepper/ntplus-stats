mod parser;
mod logitem;

use std::{fs::File, io::Read};

use logitem::LogItem;

fn main() {
    let mut file = File::open("log.log").unwrap();
    let mut buf = String::new();

    file.read_to_string(&mut buf).unwrap();

    let mut items: Vec<LogItem> = vec![];

    for line in buf.lines() {
        items.push(line.parse().unwrap());
    }

    println!("{items:#?}");
}
