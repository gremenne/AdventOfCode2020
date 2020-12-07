#[macro_use] extern crate lazy_static;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

struct Rule {
    letter: char,
    password: String,
    min: usize,
    max: usize,
}

fn extract_rule(line: &String) -> Rule {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<letter>[a-z]): (?P<string>[a-z]+)").unwrap();
    }

    let cap = RE.captures(line).unwrap();

    Rule {
        letter: cap.name("letter").unwrap().as_str().chars().next().unwrap(),
        password: cap.name("string").unwrap().as_str().to_string(),
        min: cap.name("min").unwrap().as_str().parse::<usize>().unwrap(),
        max: cap.name("max").unwrap().as_str().parse::<usize>().unwrap()
    }
}

fn check_password1(line: &String) -> bool {

    let rule = extract_rule(line);
    let count = rule.password.matches(rule.letter).count();

    (count >= rule.min) && (count <= rule.max)
}

fn check_password2(line: &String) -> bool {

    let rule = extract_rule(line);
    let password_chars : Vec<char> = rule.password.chars().collect();
   
    (password_chars[rule.min-1] == rule.letter) != (password_chars[rule.max-1] == rule.letter)
}

fn main() {
   
    if let Ok(lines) = read_lines("./input.txt") {
        let rules : Vec<String> = lines.map(|x| x.unwrap()).collect();
        let valid1 = rules.iter().filter(|line| check_password1(&line));
        let valid2 = rules.iter().filter(|line| check_password2(&line));

        println!("{} {}", valid1.count(), valid2.count());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}