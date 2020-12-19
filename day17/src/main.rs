use std::collections::HashMap;
use itertools::multizip;


struct ConwayCube {
    space: HashMap<(u32,u32,u32), bool>
}

impl ConwayCube {
    fn is_active(&self, pos:(u32, u32, u32)) -> bool {
        match self.space.get(&pos) {
            None => false,
            Some(x) => *x,
        }
    }

    fn get_neighbor_positions(&self, pos:(u32, u32, u32) ) -> impl Iterator<Item=(u32,u32,u32)> {
        let (x,y,z) = pos;
        multizip((x-1..x+1, y-1..y+1, z-1..z+1)).filter(move |neighbor| *neighbor != pos)
    }

    fn spawn_neighbors(&mut self, pos:(u32, u32, u32)) {
        for neighbor_pos in self.get_neighbor_positions(pos) {
            if !self.space.contains_key(&neighbor_pos) {
                self.space.insert(neighbor_pos, false);
            }
        }
    }

    fn set_state(&mut self, pos:(u32, u32, u32), active:bool) {
        self.space.insert(pos, active);

        if active {
            self.spawn_neighbors(pos);
        }
    }

    fn all_defined_cells(&self) -> impl Iterator<Item=(&(u32,u32,u32), &bool)> {
        self.space.iter()
    }

    fn count_active_neighbors(&self, pos:(u32, u32, u32) ) -> u32 {
        self.get_neighbor_positions(pos)
            .map(|neighbor_pos| {
                match self.is_active(neighbor_pos) {
                    true => 1,
                    false => 0,
                }})
            .sum()
    }
}

fn run_one_cycle(mut cube: ConwayCube ) -> ConwayCube {
    let all_cells : Vec<((u32,u32,u32), bool)> = cube.all_defined_cells().map(|(pos, active)| (*pos, *active)).collect();
    
    for (pos, active) in all_cells {
        let active_neighbors = cube.count_active_neighbors(pos);

        if !active && active_neighbors == 3 {
            cube.set_state(pos, true);
        }
        else if active_neighbors > 2 || active_neighbors < 3 {
            cube.set_state(pos, false);
        }
     
    }
    cube
}

fn main() {
    let input = vec![
        ".##.####",
        ".#.....#",
        "#.###.##",
        "#####.##",
        "#...##.#",
        "#######.",
        "##.#####",
        ".##...#.",
    ];

    //part_1(&lines);
    //part_2(&lines);
}