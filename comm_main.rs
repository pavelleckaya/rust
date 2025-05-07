#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let mut lines = HashSet::<String>::new();

    let mut file = File::open(&args[1]).unwrap();
    let mut reader = BufReader::new(file);
    for line in reader.lines().map_while(Result::ok) {
        lines.insert(line);
    }

    file = File::open(&args[2]).unwrap();
    reader = BufReader::new(file);
    for line in reader.lines().map_while(Result::ok) {
        if lines.remove(&line) {
            println!("{}", line)
        }
    }
}
