mod parser;
mod logitem;

use logitem::LogItem;

fn main() {
    let teststring = r"05/07/2025 21:03:43 Upload started: user tekken0892, IP address ('73.75.226.107', 52605), file downloads\Lil Shine\Losing Myself\09 It's Over.flac";
    // let teststring = r"05/07/2025 21:03:52 Upload finished: user tekken0892, IP address None, file downloads\Lil Shine\Losing Myself\09 It's Over.flac";

    let log_item: LogItem = teststring.parse().unwrap();

    println!("{log_item:#?}");
}
