use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ops::RangeFrom;

fn is_tree(space: char) -> bool {
    match space {
        '#' => true,
        '.' => false,
        _ => panic!("Illegal Input"),
    }
}

fn check_tree(row: &String, column: usize) -> bool {
    is_tree(row.chars().cycle().nth(column).unwrap())
}

fn main() {
   
    if let Ok(lines) = read_lines("./input.txt") {

        let row_vec : Vec<String> = lines.map(|x| x.unwrap()).collect();

        let trees1 = row_vec.iter()
            .zip(RangeFrom{ start:0 })
            .filter(|(row, column)| check_tree( row, *column));

        let trees2 = row_vec.iter()
            .zip(RangeFrom{ start:0 }.map(|x| x*3))
            .filter(|(row, column)| check_tree( row, *column));

        let trees3 = row_vec.iter()
            .zip(RangeFrom{ start:0 }.map(|x| x*5))
            .filter(|(row, column)| check_tree( row, *column));

        let trees4 = row_vec.iter()
            .zip(RangeFrom{ start:0 }.map(|x| x*7))
            .filter(|(row, column)| check_tree( row, *column));

        let trees5 = row_vec.iter()
            .step_by(2)
            .zip(RangeFrom{ start:0 })
            .filter(|(row, column)| check_tree( row, *column));

        //println!("{} {} {} {} {}", trees1.count(), trees2.count(), trees3.count(), trees4.count(), trees5.count());
        println!("{}", trees1.count() * trees2.count() * trees3.count() * trees4.count() * trees5.count());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}