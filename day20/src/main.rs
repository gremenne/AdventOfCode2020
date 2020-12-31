#[macro_use] extern crate lazy_static;
use std::fs::File;
use std::io::{self, BufRead,};
use std::path::Path;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::collections::HashMap;
use std::collections::VecDeque;
use regex::Regex;

#[derive(EnumIter, Clone, Debug, PartialEq)]
enum Rotation {
    NoRotation,
    Rotate90CW,
    Rotate180CW,
    Rotate270CW,
}

#[derive(EnumIter, Clone, Debug, PartialEq)]
enum Flip {
    NoFlip,
    HorizontalFlip,
    VerticalFlip,
    BothFlip,
}

#[derive(EnumIter, Clone, Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West
} 

type Edge = Vec<bool>;
type Image = Vec<Vec<bool>>;

#[derive(Clone, Debug, PartialEq)]
struct MapTile {
    id: u32,
    image: Image,
}

#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y:i32) -> Position {
        Position{x:x, y:y}
    }

    fn neighbors( &self ) -> Vec<Position> {
        vec![self.north(), self.east(), self.south(), self.west()]
    }

    fn get_ajacent_position(&self, direction: &Direction) -> Position {
        match direction {
            Direction::North => self.north(),
            Direction::East => self.east(),
            Direction::South => self.south(),
            Direction::West => self.west() }
    }

    fn north(&self) -> Position {
        Position::new(self.x, self.y-1)
    }

    fn east(&self) -> Position {
        Position::new(self.x+1, self.y)
    }

    fn south(&self) -> Position {
        Position::new(self.x, self.y+1)
    }

    fn west(&self) -> Position {
        Position::new(self.x-1, self.y)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct OrientedMapTile<'a> {
    rotation: Rotation,
    flip: Flip,
    tile: &'a MapTile,
}

impl<'a> OrientedMapTile<'a> {
    fn new(tile: &'a MapTile, rotation: Rotation, flip: Flip) -> OrientedMapTile<'a> {
        OrientedMapTile{
            rotation: rotation,
            flip: flip,
            tile: tile,
        }
    }

    fn get_edge(&self, direction: &Direction) -> Edge {
        let origional_edge = rotate(&direction, &self.rotation);
        self.tile.get_edge(&origional_edge)
        // TODO: Actually implement flipping, and make sure rotation logic is correct
    }
}

struct Map<'a> {
    positions: HashMap<Position, Option<OrientedMapTile<'a>>>,
}

impl<'a> Map<'a> {

    fn new() -> Map<'a> {
        Map{
            positions: HashMap::new(),
        }
    }

    fn add_tile(mut self, pos: &Position, orientation: Orientation, tile: &'a MapTile) -> Map<'a>  {
        self.positions.insert(*pos, Some(OrientedMapTile::new(tile, orientation)));
        self._spawn_neighbors(pos);
        self
    }
    
    fn _spawn_neighbors(&mut self, pos: &Position) {
        for neighbor_pos in pos.neighbors().iter() {
            if !self.positions.contains_key(&neighbor_pos) {
                self.positions.insert(*neighbor_pos, None);
            }
        }
    }
    
    fn get_tile_at_pos(&'a self, pos: &Position) -> Option<&OrientedMapTile<'a>> {
        match self.positions.get(&pos)
        {
            Some(tile_option) => tile_option.as_ref(),
            None => None
        }
    }
    
    fn get_empty_positions(&'a self) -> impl Iterator<Item=Position> + 'a {
        self.positions.keys().filter(move |key| self.positions[key].is_none()).cloned()
    }
    
}

fn rotate90cw (direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South }
}

fn rotate(direction: &Direction, rotation: &Rotation) -> Direction {

    match orientation {
        Rotation::NoRotation => direction.clone(),
        Rotation::Rotate90CW => rotate90cw(direction),
        Rotation::Rotate180CW => rotate90cw(&rotate90cw(direction)),
        Rotation::Rotate270CW => rotate90cw(&rotate90cw(&rotate90cw(direction))),
    }
}

impl MapTile {
    fn new( id: u32, image: Image ) -> MapTile {
        MapTile{
            id: id,
            image: image,
        }
    }

    fn get_edge(&self, direction: &Direction) -> Edge {
        match direction {
            Direction::North => self.image.first().unwrap().to_vec(),
            Direction::East => self.image.iter().map(|x|x.last().unwrap()).cloned().collect(),
            Direction::South => self.image.last().unwrap().to_vec(),
            Direction::West => self.image.iter().map(|x|x.first().unwrap()).cloned().collect(),
        }
    }
}

fn check_edge_match(base_edge: Edge, candidate: Option<Edge>) -> bool {
    println!("        {:?} + {:?}", base_edge, candidate);

    let result = match candidate {
        Some(edge) => base_edge == edge,
        None => true,
    };

    println!("          {}", result);

    result
}

fn get_ajacent_edge_in_direction<'a>(map: &'a Map, pos:&Position, direction: &Direction) -> Option<Edge> {
    let adj_pos = pos.get_ajacent_position(direction);
    let mirrored_edge = match direction {
        Direction::North => Direction::South,
        Direction::East => Direction::West,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
    };

    let adj_tile = map.get_tile_at_pos(&adj_pos);
    if adj_tile.is_some(){
        println!("      Looking at the {:?} Edge of {} ({:?})", mirrored_edge, adj_tile.unwrap().tile.id, adj_pos);
    }
    else {
        println!("      No tile currently at {:?}", adj_pos);
    }


    match map.get_tile_at_pos(&adj_pos) {
        Some(tile) => Some(tile.get_edge(&mirrored_edge)),
        None => None
    }
}

fn check_match_in_direction(new_tile: &OrientedMapTile, map: &Map, pos: &Position, direction: &Direction ) -> bool {
    println!("      Checking Edge {:?} of {:?}", direction, pos);
    check_edge_match(new_tile.get_edge(direction), get_ajacent_edge_in_direction(map, pos, direction))
}

fn check_match(new_tile: &OrientedMapTile, map: &Map, pos: &Position ) -> bool {
    Direction::iter()
        .map(|dir| check_match_in_direction(new_tile, map, pos, &dir))
        .all(|x| x==true)
}

fn try_rotate_and_match<'a>(new_tile: &'a MapTile, map: &'a Map<'a>,) -> Option<(Position, Orientation)> {

    println!("Attempting to fit {}", new_tile.id);
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();

    let empty_positions : Vec<Position> = map.get_empty_positions().collect();

    for pos in empty_positions.iter() {
        println!("  Trying Position {:?}", pos);
        for orientation in Orientation::iter() {
            println!("    Trying Orientation {:?}", orientation);
            let oriented_tile = OrientedMapTile::new(new_tile, orientation.clone());
    
            if check_match(&oriented_tile, map, &pos) {
                return Some((*pos, orientation));
            }
        }
    }

    println!("  Unable to fit at this time");
    None
}

fn id_at_position(position: &Position, map: &Map) -> u32 {
    map.get_tile_at_pos(position).unwrap().tile.id
}

fn part_1(tiles: &Vec<MapTile>) {
    let mut map = Map::new();

    let mut tile_refs : VecDeque<&MapTile> = tiles.iter().collect();

    map = map.add_tile(&Position::new(0,0), Orientation::NoChange, tile_refs.pop_front().unwrap());

    while !tile_refs.is_empty() {
        let tile : &MapTile = tile_refs.pop_front().unwrap();

        if let Some((pos, orientation)) = try_rotate_and_match(&tile, &map) {
            println!("Fitting {} to {:?} with {:?}", tile.id, pos, orientation);
            map = map.add_tile(&pos, orientation, tile);
        }
        else {
            tile_refs.push_back(tile);
        }
    }

    let x_max = map.positions.keys().map(|pos| pos.x).max().unwrap();
    let x_min = map.positions.keys().map(|pos| pos.x).min().unwrap();
    let y_max = map.positions.keys().map(|pos| pos.y).max().unwrap();
    let y_min = map.positions.keys().map(|pos| pos.y).min().unwrap();

    let corners = [id_at_position(&Position::new(x_min, y_min), &map),
                   id_at_position(&Position::new(x_max, y_min), &map),
                   id_at_position(&Position::new(x_max, y_max), &map),
                   id_at_position(&Position::new(x_min, y_max), &map)];

    println!("{} * {} * {} * {} = {}", corners[0], corners[1], corners[2], corners[3], corners[0] * corners[1] * corners[2] * corners[3]);

}

fn parse_tile_id(line: &String) -> u32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Tile (\d+):").unwrap();
    }

    let cap = RE.captures(line).unwrap();

    cap[1].parse::<u32>().unwrap()
}

fn parse_image_row(line: &String) -> Vec<bool> {
    line.chars().map( |c| match c {
        '.' => false,
        '#' => true,
        _ => panic!("Unknown Pixle!")
    }).collect()
} 

fn parse_tiles(lines: &Vec<String>) -> Vec<MapTile> {
    let mut tiles = Vec::new();
    let mut iter = lines.iter().filter(|l| !l.is_empty() );

    while let Some(line) = iter.next() {
        let id = parse_tile_id(&line);
        let image = vec![
            parse_image_row(&iter.next().unwrap()),
            parse_image_row(&iter.next().unwrap()),
            parse_image_row(&iter.next().unwrap()),
            parse_image_row(&iter.next().unwrap()),
            parse_image_row(&iter.next().unwrap()),
            parse_image_row(&iter.next().unwrap()),
            parse_image_row(&iter.next().unwrap()),
            parse_image_row(&iter.next().unwrap()),
            parse_image_row(&iter.next().unwrap()),
            parse_image_row(&iter.next().unwrap())];
        
        tiles.push(MapTile::new(id, image));
    }
    tiles

}

fn main() {
    let lines = read_lines("./example.txt");
    let tiles = parse_tiles(&lines);

    part_1(&tiles);
}

fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}