#[macro_use] extern crate lazy_static;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;

fn extract_kvps(line: &String, passport_data: &mut HashMap<String, String>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<key>[^:\s]+):(?P<value>[^:\s]+)").unwrap();
    }

    for cap in RE.captures_iter(line) {
        passport_data.insert( cap["key"].to_string() , cap["value"].to_string() );
    }
}

fn extract_one_passport<T>(line_iter: &mut T) -> Option<HashMap<String, String>>
where T: Iterator< Item=String > {

    let mut passport_data = HashMap::new();
    while let Some(line) = line_iter.next() {
        if line.is_empty() {
            return Some(passport_data);
        } else {
            extract_kvps(&line, &mut passport_data);
        }
    }

    if passport_data.is_empty() {
        return None; 
    } else {
        return Some(passport_data);
    }
}

fn extract_passport_data(lines: Vec<String>) -> Vec<HashMap<String, String>> {
    let mut all_passport_data = Vec::new();
    let mut line_iter = lines.iter().cloned();

    while let Some(passport_data) = extract_one_passport(&mut line_iter) {
        all_passport_data.push(passport_data);
    }
    all_passport_data
}

fn is_valid_passport1(passport_data: &HashMap<String, String>) -> bool {
    let keys : Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    keys.iter().map(|x| x.to_string()).all(|key| passport_data.contains_key(&key))
}

fn is_valid_year(value: &String, range: std::ops::Range<i32>) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\d\d\d\d$").unwrap();
    }

    RE.find(&value).is_some() && range.contains(&value.parse::<i32>().unwrap())
}

fn is_valid_height(value: &String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
    }

    let cap_opt = RE.captures(&value);

    if cap_opt.is_none() {
        return false;
    }

    let cap = cap_opt.unwrap();

    let range = match &cap[2] {
        "in" => 59..77,
        "cm" => 150..194,
        _ => panic!("Invalid Height")
    };

    range.contains(&cap[1].parse::<i32>().unwrap())
}

fn is_valid_hair_color(value: &String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[\da-f]{6}$").unwrap();
    }

    RE.find(&value).is_some()
}

fn is_valid_eye_color(value: &String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    }

    RE.find(&value).is_some()
}

fn is_valid_pid(value: &String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    RE.find(&value).is_some()
}

fn is_valid_passport2(passport_data: &HashMap<String, String>) -> bool {
    let keys : Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    if !keys.iter().map(|x| x.to_string()).all(|key| passport_data.contains_key(&key)) {
        return false;
    }

    return is_valid_year(&passport_data["byr"], 1920..2003) &&
           is_valid_year(&passport_data["iyr"], 2010..2021) &&
           is_valid_year(&passport_data["eyr"], 2020..2031) &&
           is_valid_height(&passport_data["hgt"]) &&
           is_valid_hair_color(&passport_data["hcl"]) &&
           is_valid_eye_color(&passport_data["ecl"]) &&
           is_valid_pid(&passport_data["pid"]);
}

fn main() {
   
    if let Ok(lines) = read_lines("./input.txt") {
        let all_passport_data = extract_passport_data(lines.map(|x| x.unwrap()).collect());
        let valid_passports1 = all_passport_data.iter().filter(|x| is_valid_passport1(x));
        let valid_passports2 = all_passport_data.iter().filter(|x| is_valid_passport2(x));

        println!("{} {}", valid_passports1.count(), valid_passports2.count());

    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}