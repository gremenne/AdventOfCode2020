#[macro_use] extern crate lazy_static;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

struct KeyValuePair { 
    key: String, 
    value: String,
}

fn extract_kvps(line: String) -> Vec<KeyValuePair> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<key>[^:\s]+):(?P<value>[^:\s]+)").unwrap();
    }

    let mut kvps = Vec::new();

    for cap in RE.captures_iter(line) {
        kvps.push( KeyValuePair{ cap["key"], cap["value"] })
    }

    kvps
}

struct KeyExtractor {
    lines: Iterator<item=String>,
}

impl Iterator for KeyExtractor {
    type item = ParseResult;

    fn next(&mut self) -> Option<ParseResult> {

    }
}

fn main() {
   
    if let Ok(lines) = read_lines("./input.txt") {
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}