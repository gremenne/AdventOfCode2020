use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum GridPosition {
    Seat { x: usize,y: usize, occupied: bool },
    Floor{ x: usize, y: usize },
}

struct GridTracker {
    grid: Vec<Vec<GridPosition>>,
}

fn GetXY(position: &GridPosition) -> (usize, usize) {
    match position {
        GridPosition::Seat{x, y, occupied} => (*x,*y),
        GridPosition::Floor{x,y} => (*x,*y),
    }
}

impl GridTracker{

    fn new() -> GridTracker {
        GridTracker{grid: Vec::new()}
    }

    fn get(&self, x: usize, y: usize) -> std::option::Option<&GridPosition>  {
        match self.grid.get(x) {
            Some(row) => row.get(y),
            None => None,
        }
    }

    fn getAjacent(&mut self, position: &GridPosition) -> Vec<std::option::Option<&GridPosition>> { 
        let (x,y) = GetXY(position);

        [self.get(x+1,y+1), self.get(x+1,y), self.get(x+1,y-1), 
         self.get(x,y+1), self.get(x,y), self.get(x,y-1), 
         self.get(x-1,y+1), self.get(x-1,y), self.get(x-1,y-1)].to_vec()
    }

    fn evaluatePosition(&mut self, position: &mut GridPosition) {
            
        let ajacent = self.getAjacent(position);
        let num_ajacent_occupied : u32 = ajacent.iter().map(|adj|{
            match adj {
                Some(GridPosition::Seat{x:_, y:_, occupied:true}) => 1,
                _ => 0
            }}).sum();

        match position {
            GridPosition::Seat{x, y, ref @occupied:true} => *occupied = (num_ajacent_occupied < 4),
            GridPosition::Seat{x, y, ref @occupied:false} => *occupied = (num_ajacent_occupied == 0),
            GridPosition::Floor{x,y} => (),
        };

    }
}



fn main() {
    let lines = read_lines("./input.txt");
    println!("Hello, world!");
}

fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}