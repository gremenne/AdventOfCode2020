use std::collections::HashMap;
use itertools::Itertools;


struct ConwayCube3D {
    space: HashMap<(i32,i32,i32), bool>
}

impl ConwayCube3D {

    fn new() -> ConwayCube3D {
        ConwayCube3D{space: HashMap::new()}
    }

    fn is_active(&self, pos:(i32, i32, i32)) -> bool {
        match self.space.get(&pos) {
            None => false,
            Some(x) => *x,
        }
    }

    fn get_neighbor_positions(&self, pos:(i32, i32, i32) ) -> impl Iterator<Item=(i32,i32,i32)> {
        let (x,y,z) = pos;
        Itertools::multi_cartesian_product(vec![x-1..x+2, y-1..y+2, z-1..z+2].iter().cloned())
            .map(|arr| (arr[0], arr[1], arr[2]) )
            .filter(move |neighbor| *neighbor != pos)
    }

    fn spawn_neighbors(&mut self, pos:(i32, i32, i32)) {
        for neighbor_pos in self.get_neighbor_positions(pos) {
            if !self.space.contains_key(&neighbor_pos) {
                self.space.insert(neighbor_pos, false);
            }
        }
    }

    fn set_state(&mut self, pos:(i32, i32, i32), active:bool) {
        self.space.insert(pos, active);

        if active {
            self.spawn_neighbors(pos);
        }
    }

    fn all_defined_cells(&self) -> impl Iterator<Item=(&(i32,i32,i32), &bool)> {
        self.space.iter()
    }

    fn count_active_neighbors(&self, pos:(i32, i32, i32) ) -> i32 {
        self.get_neighbor_positions(pos)
            .map(|neighbor_pos| {
                self.is_active(neighbor_pos)})
            .filter(|active| *active == true).count() as i32
    }
}

fn run_one_cycle_3d(cube: ConwayCube3D ) -> ConwayCube3D {
    let mut new_cube = ConwayCube3D::new();
    let all_cells : Vec<((i32,i32,i32), bool)> = cube.all_defined_cells().map(|(pos, active)| (*pos, *active)).collect();
    
    for (pos, active) in all_cells {
        let active_neighbors = cube.count_active_neighbors(pos);

        //println!("Cell {:?} is {} has {} active neighbors", pos, active, active_neighbors );

        if !active && active_neighbors == 3 {
            new_cube.set_state(pos, true);
        }
        else if active && (active_neighbors == 2 || active_neighbors == 3) {
            new_cube.set_state(pos, true);
        }
        else {
            new_cube.set_state(pos, false);
        }     
    }
    new_cube
}

fn make_cube_3d(lines: &Vec<String>) -> ConwayCube3D {
    let mut cube = ConwayCube3D::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                '#' => cube.set_state((x as i32,y as i32, 0), true),
                _ => panic!("Unkown Character"),
            };
        }
    }

    cube
}

struct ConwayCube4D {
    space: HashMap<(i32,i32,i32,i32), bool>
}

impl ConwayCube4D {

    fn new() -> ConwayCube4D {
        ConwayCube4D{space: HashMap::new()}
    }

    fn is_active(&self, pos:(i32, i32, i32, i32)) -> bool {
        match self.space.get(&pos) {
            None => false,
            Some(x) => *x,
        }
    }

    fn get_neighbor_positions(&self, pos:(i32, i32, i32, i32) ) -> impl Iterator<Item=(i32,i32,i32,i32)> {
        let (x,y,z,w) = pos;
        Itertools::multi_cartesian_product(vec![x-1..x+2, y-1..y+2, z-1..z+2, w-1..w+2,].iter().cloned())
            .map(|arr| (arr[0], arr[1], arr[2], arr[3]) )
            .filter(move |neighbor| *neighbor != pos)
    }

    fn spawn_neighbors(&mut self, pos:(i32, i32, i32, i32)) {
        for neighbor_pos in self.get_neighbor_positions(pos) {
            if !self.space.contains_key(&neighbor_pos) {
                self.space.insert(neighbor_pos, false);
            }
        }
    }

    fn set_state(&mut self, pos:(i32, i32, i32, i32), active:bool) {
        self.space.insert(pos, active);

        if active {
            self.spawn_neighbors(pos);
        }
    }

    fn all_defined_cells(&self) -> impl Iterator<Item=(&(i32,i32,i32,i32), &bool)> {
        self.space.iter()
    }

    fn count_active_neighbors(&self, pos:(i32, i32, i32, i32) ) -> i32 {
        self.get_neighbor_positions(pos)
            .map(|neighbor_pos| {
                self.is_active(neighbor_pos)})
            .filter(|active| *active == true).count() as i32
    }
}

fn run_one_cycle_4d(cube: ConwayCube4D ) -> ConwayCube4D {
    let mut new_cube = ConwayCube4D::new();
    let all_cells : Vec<((i32,i32,i32,i32), bool)> = cube.all_defined_cells().map(|(pos, active)| (*pos, *active)).collect();
    
    for (pos, active) in all_cells {
        let active_neighbors = cube.count_active_neighbors(pos);

        //println!("Cell {:?} is {} has {} active neighbors", pos, active, active_neighbors );

        if !active && active_neighbors == 3 {
            new_cube.set_state(pos, true);
        }
        else if active && (active_neighbors == 2 || active_neighbors == 3) {
            new_cube.set_state(pos, true);
        }
        else {
            new_cube.set_state(pos, false);
        }     
    }
    new_cube
}

fn make_cube_4d(lines: &Vec<String>) -> ConwayCube4D {
    let mut cube = ConwayCube4D::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                '#' => cube.set_state((x as i32,y as i32, 0, 0), true),
                _ => panic!("Unkown Character"),
            };
        }
    }

    cube
}

fn part_1(lines: &Vec<String>) {
    let mut cube = make_cube_3d(lines);

    println!("{}", cube.all_defined_cells().filter(|(_, active)| **active == true).count());

    for i in 1..7 {
        cube = run_one_cycle_3d(cube);
        println!("Generation {}: {}", i, cube.all_defined_cells().filter(|(_, active)| **active == true).count());
    }
}

fn part_2(lines: &Vec<String>) {
    let mut cube = make_cube_4d(lines);

    println!("{}", cube.all_defined_cells().filter(|(_, active)| **active == true).count());

    for i in 1..7 {
        cube = run_one_cycle_4d(cube);
        println!("Generation {}: {}", i, cube.all_defined_cells().filter(|(_, active)| **active == true).count());
    }
}

fn main() {
    let input1 = vec![
        ".##.####".to_string(),
        ".#.....#".to_string(),
        "#.###.##".to_string(),
        "#####.##".to_string(),
        "#...##.#".to_string(),
        "#######.".to_string(),
        "##.#####".to_string(),
        ".##...#.".to_string(),
    ];

    let _test_input = vec![
        ".#.".to_string(),
        "..#".to_string(),
        "###".to_string(),
    ];

    part_1(&input1);
    part_2(&input1);
}