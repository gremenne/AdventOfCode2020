use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum Symbol {
    Number{value:u64},
    Addition,
    Multiplication,
    OpenParentheses,
    CloseParentheses,
}


fn p1_next_value<T>(symbols: &mut T) -> u64 
where T: Iterator< Item=Symbol > {
    match symbols.next() {
        None => panic!("Missing Right-Hand Value for Operator!"),
        Some(Symbol::OpenParentheses) => p1_evaluate(p1_next_value(symbols), symbols),
        Some(Symbol::Number{value}) => value,
        Some(x) => panic!("Unexpected Symbol! Needed an Operator, got {:?}", x)
    }
}

fn p1_evaluate<T>( inital_value: u64, symbols: &mut T) -> u64 
where T: Iterator< Item=Symbol > {

    match symbols.next() {
        None => inital_value,

        Some(Symbol::Addition) => {
            let next = p1_next_value(symbols);
            //println!("{} + {} = {}", inital_value, next, inital_value + next);
            p1_evaluate(inital_value + next, symbols)},

        Some(Symbol::Multiplication) => {
            let next = p1_next_value(symbols);
            //println!("{} * {} = {}", inital_value, next, inital_value * next);
            p1_evaluate(inital_value * next, symbols)},

        Some(Symbol::CloseParentheses) => inital_value,
        Some(x) => panic!("Unexpected Symbol! Needed an Operator, got {:?}", x)
    }
}

fn p2_next_value<T>(symbols: &mut T) -> u64 
where T: Iterator< Item=Symbol > {
    match symbols.next() {
        None => panic!("Missing Right-Hand Value for Operator!"),
        Some(Symbol::OpenParentheses) => p2_evaluate(p2_next_value(symbols), symbols),
        Some(Symbol::Number{value}) => value,
        Some(x) => panic!("Unexpected Symbol! Needed an Operator, got {:?}", x)
    }
}

fn p2_evaluate<T>( inital_value: u64, symbols: &mut T) -> u64 
where T: Iterator< Item=Symbol > {

    match symbols.next() {
        None => inital_value,

        Some(Symbol::Addition) => {
            let next = p2_next_value(symbols);
            //println!("{} + {} = {}", inital_value, next, inital_value + next);
            p2_evaluate(inital_value + next, symbols)},

        Some(Symbol::Multiplication) => {
            let next = p2_next_value(symbols);
            //println!("{} * {} = {}", inital_value, next, inital_value * next);
            inital_value * p2_evaluate(next, symbols)},

        Some(Symbol::CloseParentheses) => inital_value,
        Some(x) => panic!("Unexpected Symbol! Needed an Operator, got {:?}", x)
    }
}

fn to_symbol(c: char) -> Symbol {
    match c {
        '+' => Symbol::Addition,
        '*' => Symbol::Multiplication,
        '(' => Symbol::OpenParentheses,
        ')' => Symbol::CloseParentheses,
        number @ '0'..='9' => Symbol::Number{value:number.to_digit(10).unwrap() as u64},
        _ => panic!("Unknown Symbol")
    }
}

fn parse_line<'a>(line: &'a String) -> impl Iterator<Item=Symbol> + 'a {
    line.chars().filter(|x| *x != ' ').map(|x| to_symbol(x))
}

fn p1_evaluate_line(line: &String) -> u64 {
    let mut iter = parse_line(line);

    p1_evaluate(p1_next_value(&mut iter), &mut iter)
}

fn p2_evaluate_line(line: &String) -> u64 {
    let mut iter = parse_line(line);

    p2_evaluate(p2_next_value(&mut iter), &mut iter)
}

fn part_1(lines: &Vec<String>) {
    let total : u64 = lines.iter().map(|x| p1_evaluate_line(x)).sum();
    println!("{}", total);
}

fn part_2(lines: &Vec<String>) {
    let total : u64 = lines.iter().map(|x| p2_evaluate_line(x)).sum();
    println!("{}", total);
}

fn main() {
    let lines = read_lines("./input.txt");

    part_1(&lines);
    part_2(&lines);
}

fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}

#[test]
fn p1_example_1 () {
    assert_eq!(p1_evaluate_line(&"1 + 2 * 3 + 4 * 5 + 6".to_string()), 71);
}

#[test]
fn p1_example_2 () {
    assert_eq!(p1_evaluate_line(&"1 + (2 * 3) + (4 * (5 + 6))".to_string()), 51);
}

#[test]
fn p1_example_3 () {
    assert_eq!(p1_evaluate_line(&"2 * 3 + (4 * 5)".to_string()), 26);
}

#[test]
fn p1_example_4 () {
    assert_eq!(p1_evaluate_line(&"5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()), 437);
}

#[test]
fn p1_example_5 () {
    assert_eq!(p1_evaluate_line(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()), 12240);
}

#[test]
fn p1_example_6 () {
    assert_eq!(p1_evaluate_line(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()), 13632);
}

#[test]
fn p2_example_1 () {
    assert_eq!(p2_evaluate_line(&"1 + 2 * 3 + 4 * 5 + 6".to_string()), 231);
}

#[test]
fn p2_example_2 () {
    assert_eq!(p2_evaluate_line(&"1 + (2 * 3) + (4 * (5 + 6))".to_string()), 51);
}

#[test]
fn p2_example_3 () {
    assert_eq!(p2_evaluate_line(&"2 * 3 + (4 * 5)".to_string()), 46);
}

#[test]
fn p2_example_4 () {
    assert_eq!(p2_evaluate_line(&"5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()), 1445);
}

#[test]
fn p2_example_5 () {
    assert_eq!(p2_evaluate_line(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()), 669060);
}

#[test]
fn p2_example_6 () {
    assert_eq!(p2_evaluate_line(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()), 23340);
}