use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

struct BLSeparatedData<'a, T>
where T: Iterator< Item=String > {
    line_iter: &'a mut T,
}

impl<'a, T> Iterator for BLSeparatedData<'a, T> 
where T: Iterator< Item=String > {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Vec<String>> {
        let mut collected = Vec::new();

        while let Some(line) = self.line_iter.next() {
            if line.is_empty() {
                break;
            }

            collected.push(line);
        }
    
        if !collected.is_empty() {
            return Some(collected);
        } 
        None
    }
}

fn collect_blank_line_separated_data<T>(line_iter: &mut T) -> BLSeparatedData<T>
where T: Iterator< Item=String > {
    BLSeparatedData {line_iter: line_iter}
}

fn to_set(line: &String) -> HashSet<char> {
    let mut answers = HashSet::new();
    for c in line.chars() {
        answers.insert(c);
    }
    answers
}

fn extract_answers1(lines: &Vec<String>) -> Vec<HashSet<char>> {
    let mut all_answers = Vec::new();

    for lines in collect_blank_line_separated_data(&mut lines.iter().cloned()) {
        let answers = lines.iter().map(|x| to_set(x))
            .fold(HashSet::new(), |x, y| x.union(&y).cloned().collect());

        all_answers.push(answers);
    }

    all_answers
}

fn all_chars() -> HashSet<char>
{
    let set: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    set
}

fn extract_answers2(lines: &Vec<String>) -> Vec<HashSet<char>> {
    let mut all_answers = Vec::new();

    for lines in collect_blank_line_separated_data(&mut lines.iter().cloned()) {
        let answers = lines.iter().map(|x| to_set(x))
            .fold(all_chars(), |x, y| x.intersection(&y).cloned().collect());

        all_answers.push(answers);
    }

    all_answers
}

fn main() {

    let lines = read_lines("./input.txt");

    let all_answers1 = extract_answers1(&lines);
    println!("{}", all_answers1.iter().map(|x| x.len()).sum::<usize>() );

    let all_answers2 = extract_answers2(&lines);
    println!("{}", all_answers2.iter().map(|x| x.len()).sum::<usize>() );

}

fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}