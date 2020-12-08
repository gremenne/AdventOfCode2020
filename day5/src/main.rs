use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::sorted;

struct BordingPass {
    row: i32,
    column: i32,
    id: i32,
}

fn parse_bording_pass(line: &String) -> BordingPass {

    let chars : Vec<char> = line.chars().collect();
    let row : i32 = chars[..7].iter()
        .map(|x| match x {
            'B' => 1, 
            'F' => 0, 
            _ =>panic!("Bad Encoding")})
        .zip([64,32,16,8,4,2,1].iter())
        .map(|(x,y)| x*y)
        .sum();

    let column : i32 = chars[7..10].iter()
        .map(|x| match x {
            'R' => 1, 
            'L' => 0, 
            _ =>panic!("Bad Encoding")})
        .zip([4,2,1].iter())
        .map(|(x,y)| x*y)
        .sum();

    BordingPass {
        row: row, 
        column: column,
        id: row*8 + column }
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {

        let line_vector : Vec<String> = lines.map(|x| x.unwrap()).collect();

        let ids : Vec<i32> = line_vector.iter().map(|x| parse_bording_pass(x).id).collect();

        let max = ids.iter().max();

        println!("Max = {}", max.unwrap());

        for id in 0..1024 {
            if (!ids.contains(&id)) && ids.contains(&(id+1)) && ids.contains(&(id-1)) {
                println!("My Seat ID: {}", id)
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}