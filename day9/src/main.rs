use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

fn main() {
    let lines = read_lines("./input.txt");
    let values : Vec<i64> = lines.iter().map(|x|x.parse::<i64>().unwrap()).collect();

    let mut preamble = VecDeque::<i64>::new();

    preamble.extend(values[..25].iter());

    for value in values[25..].iter() {
        let mut valid = false;
        for i in 0..24 {
            let part1 = preamble[i];
            for part2 in preamble.iter().skip(i+1) {
                if part1 + part2 == *value {
                    valid = true;
                }
            }
        }

        if !valid {
            println!("{}", value);
            break;
        }

        preamble.push_back(*value);
        preamble.pop_front();
    }

    let magic_value = 258585477;

    for start in 0..values.len(){
        let mut acc = magic_value - values[start];
        let mut end = start+1;

        while end < values.len() && acc > 0
        {
            acc -= values[end];
            end += 1;
        }

        if acc == 0
        {
            let weakness = values[start..end].to_vec();
            let min = weakness.iter().min().unwrap();
            let max = weakness.iter().max().unwrap();
            println!("{} {} {}", min, max, *min + *max);
            break;
        }

    }
}

fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}