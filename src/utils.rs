use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

pub fn read_lines(filename: impl AsRef<Path>) -> Box<dyn Iterator<Item = String>> {
    let file = File::open(filename).expect("unable to open file");
    let buf = BufReader::new(file);
    Box::new(buf.lines().map(|l| l.expect("unable to parse line")))
}
