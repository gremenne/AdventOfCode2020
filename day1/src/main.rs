use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn balance_books2(values: &Vec<i32>) {
    for (i, value) in values.iter().enumerate() {
        for result in values[i..].iter().filter(|x| *x + value == 2020).into_iter() {
            println!("{} * {} = {}", result, value, result*value);
        }
    }
}

fn balance_books3(values: &Vec<i32>) {

    for (i, value1) in values.iter().enumerate() {
        for (j, value2) in values[i..].iter().enumerate() {
            for result in values[j..].iter().filter(|x| *x + value1 + value2 == 2020).into_iter() {
                println!("{} * {} * {} = {}", result, value1, value2, result*value1*value2);
            }
        }
    }
}

fn main() {

    if let Ok(lines) = read_lines("./input.txt") {
        let values = parse_lines(lines);
        balance_books2(&values);
        balance_books3(&values);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_lines<T> (lines: T) -> Vec<i32>
where T: IntoIterator<Item = std::result::Result<String, std::io::Error>>, {
    lines.into_iter().map(|x| as_int(x)).collect()
}

fn as_int(line: std::result::Result<String, std::io::Error>) -> i32 {
    line.unwrap().parse::<i32>().unwrap()
}