#[macro_use] extern crate lazy_static;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

#[derive(Clone, Debug)]
struct MaskInstruction1 {
    and_mask : u64,
    or_mask : u64,
    mask_str: String,
}

impl MaskInstruction1 {
    fn new(mask_str: &String) -> MaskInstruction1 {

    let (and_mask, or_mask) = mask_str.chars().fold((0,0), |(and_mask, or_mask), c|{
        match c {
            '0' => ((and_mask<<1),     (or_mask<<1)),
            '1' => ((and_mask<<1) + 1, (or_mask<<1) + 1),
            'X' => ((and_mask<<1) + 1, (or_mask<<1)),
            _ => panic!("Unkown Character!")
        }
    });

    //println!("Mask:     {}", mask_str);
    //println!("and_mask: {:036b}", and_mask);
    //println!("or_mask:  {:036b}", or_mask);

    MaskInstruction1{and_mask:and_mask, or_mask:or_mask, mask_str:mask_str.to_string()}
    }

    fn apply(&self, value: u64) -> u64 {
        ( value & self.and_mask ) | self.or_mask
    }
}

#[derive(Copy, Clone, Debug)]
struct MemInstruction {
    address : u64,
    value : u64,
}

enum Instruction1 {
    Mask(MaskInstruction1),
    Mem(MemInstruction),
}

struct State1 {
    current_mask : MaskInstruction1,
    memory : HashMap<u64, u64>,
}

impl State1 {
    fn new() -> State1 {
        State1{
            current_mask: MaskInstruction1::new(&"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string()),
            memory : HashMap::new(),
        }
    }

    fn execute(mut self, instruction: Instruction1) -> State1 {
        match instruction {
            Instruction1::Mask(mask) => {
                //println!("Updated Mask: {}", mask.mask_str);
                self.current_mask = mask;},
            Instruction1::Mem(mem) => {
                //println!("Wrote to Memory[{}]: {} masked with {} = {}", mem.address, mem.value, self.current_mask.mask_str, self.current_mask.apply(mem.value));
                self.memory.insert(mem.address, self.current_mask.apply(mem.value)); },
        };

        self
    }
}

fn parse_instruction1(line: &String) -> Instruction1 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<instruction>mask|mem)(\[(?P<address>\d+)\])? = (?P<value>[0-9X]+)").unwrap();
    }

    let cap = RE.captures(line).unwrap();

    match &cap["instruction"] {
        "mask" => Instruction1::Mask(MaskInstruction1::new(&cap["value"].to_string())),

        "mem" => Instruction1::Mem(MemInstruction{ address:cap["address"].parse::<u64>().unwrap(), 
                                                  value:cap["value"].parse::<u64>().unwrap()}),

        _ => panic!("Unkown instruction")
    }
}

fn parse_instructions1<'a>(lines: &'a Vec<String>) -> impl Iterator<Item=Instruction1> +'a {
    lines.iter().map(|x| parse_instruction1(x))
}

fn part_1(lines: &Vec<String>) {
    let instructions = parse_instructions1(lines);

    let final_state = instructions.fold(State1::new(), |state, instr|state.execute(instr));

    println!("{}", final_state.memory.values().sum::<u64>())
}

#[derive(Clone, Debug)]
struct MaskInstruction2 {
    or_mask : u64,
    floatings : Vec<usize>,
    mask_str: String,
}

impl MaskInstruction2 {
    fn new(mask_str: &String) -> MaskInstruction2 {

    let mut floatings = Vec::new();

    let (or_mask, _) = mask_str.chars().fold((0,0), |(or_mask, index), c|{
        match c {
            '0' => ((or_mask<<1), index +1 ),
            '1' => ((or_mask<<1) + 1, index +1),
            'X' => { floatings.push(35-index);
                   ((or_mask<<1), index +1 )},
            _ => panic!("Unkown Character!")
        }
    });

    MaskInstruction2{or_mask:or_mask, floatings:floatings, mask_str:mask_str.to_string()}
    }

    fn apply(&self, value: u64) -> Vec<u64>{
        let masked_value = value | self.or_mask;

        self.floatings.iter()
            .fold(vec![masked_value], |values, index| {
                values.iter()
                    .map(|x| x | (1<<index))
                    .chain(
                        values.iter()
                        .map(|x| x & !(1<<index)))
                        .collect()})
    }
}

enum Instruction2 {
    Mask(MaskInstruction2),
    Mem(MemInstruction),
}

struct State2 {
    current_mask : MaskInstruction2,
    memory : HashMap<u64, u64>,
}

impl State2 {
    fn new() -> State2 {
        State2{
            current_mask: MaskInstruction2::new(&"000000000000000000000000000000000000".to_string()),
            memory : HashMap::new(),
        }
    }

    fn execute(mut self, instruction: Instruction2) -> State2 {
        match instruction {
            Instruction2::Mask(mask) => {
                //println!("Updated Mask: {}", mask.mask_str);
                self.current_mask = mask;},
            Instruction2::Mem(mem) => {
                for addr in self.current_mask.apply(mem.address).iter() {
                    //println!("Writing {} to {:036b}", mem.value, addr);
                    self.memory.insert(*addr, mem.value);
                }
            },
        };

        self
    }
}

fn parse_instruction2(line: &String) -> Instruction2 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<instruction>mask|mem)(\[(?P<address>\d+)\])? = (?P<value>[0-9X]+)").unwrap();
    }

    let cap = RE.captures(line).unwrap();

    match &cap["instruction"] {
        "mask" => Instruction2::Mask(MaskInstruction2::new(&cap["value"].to_string())),

        "mem" => Instruction2::Mem(MemInstruction{ address:cap["address"].parse::<u64>().unwrap(), 
                                                  value:cap["value"].parse::<u64>().unwrap()}),

        _ => panic!("Unkown instruction")
    }
}

fn parse_instructions2<'a>(lines: &'a Vec<String>) -> impl Iterator<Item=Instruction2> +'a {
    lines.iter().map(|x| parse_instruction2(x))
}

fn part_2(lines: &Vec<String>) {
    let instructions = parse_instructions2(lines);

    let final_state = instructions.fold(State2::new(), |state, instr|state.execute(instr));

    println!("{}", final_state.memory.values().sum::<u64>())
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