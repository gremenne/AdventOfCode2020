use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum GridPosition {
    Seat { occupied: bool },
    Floor,
}

#[derive(Eq, PartialEq)]
struct GridTracker1 {
    grid: Vec<Vec<GridPosition>>,
}

impl GridTracker1{

    fn new() -> GridTracker1 {
        GridTracker1{grid: Vec::new()}
    }

    fn add_row(&mut self, row:Vec<GridPosition>) {
        self.grid.push(row);
    }

    fn get(&self, x: isize, y: isize) -> std::option::Option<&GridPosition>  {
        if x < 0 || y < 0 {
            return None;
        } 

        match self.grid.get(x as usize) {
            Some(row) => row.get(y as usize),
            None => None,
        }
    }

    fn get_adjacent(&self, x:isize, y:isize) -> Vec<std::option::Option<&GridPosition>> { 
        [self.get(x+1,y+1), self.get(x+1,y), self.get(x+1,y-1), 
         self.get(x,y+1), self.get(x,y-1), 
         self.get(x-1,y+1), self.get(x-1,y), self.get(x-1,y-1)].to_vec()
    }

    fn get_num_occupied_adjacent(&self, x:usize, y:usize) -> u64 {
        self.get_adjacent(x as isize, y as isize).iter().map(|adjacent|{
            match adjacent {
                Some(GridPosition::Seat{occupied:true}) => 1,
                _ => 0,
            }}).sum()
    }

    fn get_next_generation_for_position(&self, x:usize, y:usize, position: &GridPosition) -> GridPosition{
            
        let num_adjacent_occupied = self.get_num_occupied_adjacent(x,y);

        match position {
            GridPosition::Seat{occupied:true} => GridPosition::Seat{occupied:num_adjacent_occupied < 4},
            GridPosition::Seat{occupied:false} => GridPosition::Seat{occupied:num_adjacent_occupied == 0},
            GridPosition::Floor=> GridPosition::Floor,
        }
    }

    fn get_next_generation(&self) -> GridTracker1 {
        let mut next_generation = GridTracker1::new();

        for (x,row) in self.grid.iter().enumerate() {
            next_generation.add_row(row.iter().cloned().enumerate().map(|(y,position)|self.get_next_generation_for_position(x,y,&position)).collect());
        }

        next_generation
    }
}

#[derive(Eq, PartialEq)]
struct GridTracker2 {
    grid: Vec<Vec<GridPosition>>,
}

impl GridTracker2{

    fn new() -> GridTracker2 {
        GridTracker2{grid: Vec::new()}
    }

    fn add_row(&mut self, row:Vec<GridPosition>) {
        self.grid.push(row);
    }

    fn get(&self, x: isize, y: isize) -> std::option::Option<&GridPosition>  {
        if x < 0 || y < 0 {
            return None;
        } 

        match self.grid.get(x as usize) {
            Some(row) => row.get(y as usize),
            None => None,
        }
    }

    fn get_visible(&self, x:isize, y:isize, dx:isize, dy:isize) -> std::option::Option<&GridPosition> {
        let (mut cx, mut cy) = (x+dx,y+dy);

        loop{
            match self.get(cx, cy) {
                Some(GridPosition::Seat{..}) => return self.get(cx, cy),
                Some(_) => (),
                None => return None,
            };

            cx = cx+dx;
            cy = cy+dy;
        }
    }

    fn get_all_visible(&self, x:isize, y:isize) -> Vec<std::option::Option<&GridPosition>> { 

        [self.get_visible(x, y,  1,  1),
         self.get_visible(x, y,  1,  0),
         self.get_visible(x, y,  1, -1),
         self.get_visible(x, y,  0,  1),
         self.get_visible(x, y,  0, -1),
         self.get_visible(x, y, -1,  1),
         self.get_visible(x, y, -1,  0),
         self.get_visible(x, y, -1, -1)].to_vec()
    }

    fn get_num_occupied_visible(&self, x:usize, y:usize) -> u64 {
        self.get_all_visible(x as isize, y as isize).iter().map(|adjacent|{
            match adjacent {
                Some(GridPosition::Seat{occupied:true}) => 1,
                _ => 0,
            }}).sum()
    }

    fn get_next_generation_for_position(&self, x:usize, y:usize, position: &GridPosition) -> GridPosition{
            
        let num_adjacent_visible = self.get_num_occupied_visible(x,y);

        match position {
            GridPosition::Seat{occupied:true} => GridPosition::Seat{occupied:num_adjacent_visible < 5},
            GridPosition::Seat{occupied:false} => GridPosition::Seat{occupied:num_adjacent_visible == 0},
            GridPosition::Floor=> GridPosition::Floor,
        }
    }

    fn get_next_generation(&self) -> GridTracker2 {
        let mut next_generation = GridTracker2::new();

        for (x,row) in self.grid.iter().enumerate() {
            next_generation.add_row(row.iter().cloned().enumerate().map(|(y,position)|self.get_next_generation_for_position(x,y,&position)).collect());
        }

        next_generation
    }
}

fn parse_row(line: &String) -> Vec<GridPosition>
{
    line.chars().map(|c|{
        match c {
            '.' => GridPosition::Floor,
            'L' => GridPosition::Seat{occupied:false},
            '#' => GridPosition::Seat{occupied:true},
            _ => panic!("Unexpected Character")
        }}).collect()
}

fn make_initial_grid_tracker1(lines: &Vec<String>) -> GridTracker1 {
    let mut grid_tracker = GridTracker1::new();

    for line in lines {
        grid_tracker.add_row(parse_row(line));
    }
    grid_tracker
}

fn make_initial_grid_tracker2(lines: &Vec<String>) -> GridTracker2 {
    let mut grid_tracker = GridTracker2::new();

    for line in lines {
        grid_tracker.add_row(parse_row(line));
    }
    grid_tracker
}

fn count_row_occupied(row: &Vec<GridPosition>) -> u64 {
    row.iter().map(|pos|{
        match pos {
            GridPosition::Seat{occupied:true} => 1,
            _ => 0,
        }}).sum()
}

fn count_occupied1(grid_tracker: &GridTracker1) -> u64 {
    grid_tracker.grid.iter().map( |row| count_row_occupied(row)).sum()
}

fn count_occupied2(grid_tracker: &GridTracker2) -> u64 {
    grid_tracker.grid.iter().map( |row| count_row_occupied(row)).sum()
}

fn part1(lines: & Vec<String>)
{
    let mut current_state = make_initial_grid_tracker1(&lines);
    let mut next_state = current_state.get_next_generation();
    
    while current_state != next_state {

        current_state = next_state;
        next_state = current_state.get_next_generation();
    }

    println!("{}", count_occupied1(&current_state) );
}

fn part2(lines: & Vec<String>)
{
    let mut current_state = make_initial_grid_tracker2(&lines);
    let mut next_state = current_state.get_next_generation();
    
    while current_state != next_state {

        current_state = next_state;
        next_state = current_state.get_next_generation();
    }

    println!("{}", count_occupied2(&current_state) );
}

fn main() {
    let lines = read_lines("./input.txt");
   
    part1(&lines);
    part2(&lines);
    
}

fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}