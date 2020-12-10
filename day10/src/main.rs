use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input.txt");
    let mut values : Vec<i64> = lines.iter().map(|x|x.parse::<i64>().unwrap()).collect();
    values.sort();

    let mut one_jumps = 0;
    let mut three_jumps = 1;

    values.iter().fold(0, |x, y| {
        match y-x {
            1 => one_jumps+=1,
            2 => (),
            3 => three_jumps+=1,
            _ => panic!("The Math don't add!")
        }; 
        *y
     });

     println!("{}", one_jumps * three_jumps);

    values.insert(0, 0);

    let mut options_vec : Vec<i64> = Vec::new();
    options_vec.push(1);
    
    for i in 1..values.len() {
        let value = values[i];
        let options : i64 = vec![value-1, value-2, value-3].iter().map(|x| {
            match values.iter().position(|y| x==y) {
                Some(index) => options_vec[index],
                None => 0
            }
        }).sum();

        options_vec.push(options);
    }

     println!("{}", options_vec.last().unwrap());

}

fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}