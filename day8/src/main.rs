#[macro_use] extern crate lazy_static;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

#[derive(Copy, Clone)]
struct MachineState {
    acc: i32,
    pc: i32,
}

#[derive(Copy, Clone)]
enum Operation {
    ACC{ value: i32},
    JMP{ offset: i32},
    NOP{ value: i32},
}

fn parse_line(line: &String) -> Operation {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(nop|acc|jmp) ([-+]\d+)").unwrap();
    }

    let cap = RE.captures(line).unwrap();

    let value = cap[2].parse::<i32>().unwrap();

    match &cap[1] {
        "nop" => Operation::NOP{value:value},
        "acc" => Operation::ACC{value:value},
        "jmp" => Operation::JMP{offset:value},
        _ => panic!("Unknown Opcode"),
    }
}

fn parse_program(lines: Vec<String>) -> Vec<Operation> {
    lines.iter().map(|x|parse_line(x)).collect()
}

fn run_cycle(state: &MachineState, opcode: Operation) -> MachineState {
    match opcode {
        Operation::ACC{value} => MachineState{     acc: state.acc + value,
                                                   pc: state.pc +1},
        Operation::JMP{offset} => MachineState{    acc: state.acc,
                                                   pc: state.pc +offset},
        Operation::NOP{value:_}  => MachineState{  acc: state.acc,
                                                   pc: state.pc +1},                                               
    }
}

fn simulate( program: &Vec<Operation> ) -> Result<i32, i32>{
    let mut trace : Vec<(Operation, bool)> = program.iter().map(|x| (*x, false)).collect();
    let mut state = MachineState{acc:0,pc:0};

    loop{
        if state.pc as usize == trace.len() {
            return Ok(state.acc);
        }
        let (opcode, hit) = trace[state.pc as usize];
        if hit {
            return Err(state.acc);
        }
        trace[state.pc as usize] = (opcode, true);
        state=run_cycle(&state,opcode);
    }
}

fn main() {
    let lines = read_lines("./input.txt");

    let program = parse_program(lines);

    match simulate(&program) {
        Ok(_) => println!("This should not have happenend..."),
        Err(acc) => println!("Broke! {}", acc) 
    }

    for i in 0..program.len(){
        let mut copy = program.to_vec();
        match program[i] {
            Operation::ACC{value: _} => continue,
            Operation::JMP{offset} => copy[i] = Operation::NOP{value:offset},
            Operation::NOP{value}  => copy[i] = Operation::JMP{offset:value},
        }

        match simulate(&copy) {
            Ok(acc) => println!("Fixed! {}", acc),
            Err(_) => continue
        }
        break;
    }
}

fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}