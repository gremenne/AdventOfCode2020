use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct State1 {
    x: f64,
    y: f64,
    heading: f64,
}

#[derive(Debug)]
struct State2 {
    x: f64,
    y: f64,
    wx: f64,
    wy: f64,
}

#[derive(Debug)]
enum ActionEnum {
    North{value:f64},
    South{value:f64},
    East{value:f64},
    West{value:f64},
    Left{value:f64},
    Right{value:f64},
    Forward{value:f64},
}

fn update1(state: &State1, action: &ActionEnum) -> State1 {

    match action {
        ActionEnum::North{value} => State1{ x:state.x, 
                                            y:state.y + value, 
                                            heading:state.heading},

        ActionEnum::South{value} => State1{ x:state.x, 
                                            y:state.y - value, 
                                            heading:state.heading},

        ActionEnum::East{value}  => State1{ x:state.x + value, 
                                            y:state.y, 
                                            heading:state.heading},
                                           
        ActionEnum::West{value}  => State1{ x:state.x - value, 
                                            y:state.y, 
                                            heading:state.heading},

        ActionEnum::Left{value}  => State1{ x:state.x, 
                                            y:state.y, 
                                            heading:state.heading + value},

        ActionEnum::Right{value} => State1{ x:state.x, 
                                            y:state.y, 
                                            heading:state.heading - value},

        ActionEnum::Forward{value} => State1{ x:state.x + state.heading.to_radians().cos() * value, 
                                              y:state.y + state.heading.to_radians().sin() * value,  
                                              heading:state.heading},    
    }
}

fn parse_action(string: &String) -> ActionEnum {
    let (action, value_str) = string.split_at(1);
    let value = value_str.parse::<f64>().unwrap();

    match action {
        "N" => ActionEnum::North{value},
        "S" => ActionEnum::South{value},
        "E" => ActionEnum::East{value},
        "W" => ActionEnum::West{value},
        "L" => ActionEnum::Left{value},
        "R" => ActionEnum::Right{value},
        "F" => ActionEnum::Forward{value},
        _ => panic!("Unknown Action!"),
    }
}

fn part_1(lines: &Vec<String>) {

    let end_point = lines.iter()
        .map(|x| parse_action(x))
        .fold(
            State1{x:0.0,y:0.0,heading:0.0}, 
            |state, action| update1(&state, &action));

    println!("{}", end_point.x.abs() + end_point.y.abs())
}

fn rotate(state: &State2, angle_degrees:f64) -> State2 {
    let r = (state.wx.powf(2.0) + state.wy.powf(2.0)).sqrt();
    let theta = state.wy.atan2(state.wx) + angle_degrees.to_radians();

    State2{ x:state.x,
            y:state.y,
            wx:theta.cos() * r,
            wy:theta.sin() * r}
}

fn update2(state: &State2, action: &ActionEnum) -> State2 {

    match action {
        ActionEnum::North{value} => State2{ x:state.x, 
                                            y:state.y,
                                            wx:state.wx,
                                            wy:state.wy + value},

        ActionEnum::South{value} => State2{ x:state.x, 
                                            y:state.y,
                                            wx:state.wx,
                                            wy:state.wy - value},

        ActionEnum::East{value}  => State2{ x:state.x, 
                                            y:state.y,
                                            wx:state.wx + value,
                                            wy:state.wy},
                                           
        ActionEnum::West{value}  => State2{ x:state.x, 
                                            y:state.y,
                                            wx:state.wx - value,
                                            wy:state.wy},

        ActionEnum::Left{value}  => rotate(state, *value),

        ActionEnum::Right{value} => rotate(state, -(*value)),

        ActionEnum::Forward{value} => State2{ x:state.x + state.wx * value, 
                                              y:state.y + state.wy * value,
                                              wx:state.wx,
                                              wy:state.wy},  
    }
}

fn part_2(lines: &Vec<String>)
{
    let end_point = lines.iter()
        .map(|x| parse_action(x))
        .fold(
            State2{x:0.0,y:0.0,wx:10.0,wy:1.0},
            |state, action| update2(&state, &action));

    println!("{}", end_point.x.abs() + end_point.y.abs())
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