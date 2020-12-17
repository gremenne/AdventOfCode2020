#[macro_use] extern crate lazy_static;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
struct Field {
    name: String,
    range1: std::ops::Range<u32>,
    range2: std::ops::Range<u32>,
}

impl Field {
    fn is_valid(&self, value:u32) -> bool {
        self.range1.contains(&value) || self.range2.contains(&value)
    }
}

fn parse_field(line: &String) -> Field {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    }

    let cap = RE.captures(line).unwrap();
    let range1_start = cap[2].parse::<u32>().unwrap();
    let range1_end   = cap[3].parse::<u32>().unwrap() + 1;
    let range2_start = cap[4].parse::<u32>().unwrap();
    let range2_end   = cap[5].parse::<u32>().unwrap() + 1;

    Field{
        name:cap[1].to_string(),
        range1: range1_start..range1_end,
        range2: range2_start..range2_end,
    }
}

fn parse_ticket(line: &String) -> Vec<u32> {
    line.split(",").map(|line|line.parse::<u32>().unwrap()).collect()
}

fn part_1(fields: &Vec<Field>, tickets: &Vec<Vec<u32>>) {

    let mut invalid_numbers : Vec<u32> = Vec::new();

    for ticket in tickets.iter() {
        for value in ticket.iter() {
            if !fields.iter().any(|field|field.is_valid(*value)) {
                invalid_numbers.push(*value);
            }
        }
    }

    println!("{}", invalid_numbers.iter().sum::<u32>());
}

fn value_is_possibly_valid(fields: &Vec<Field>, value:u32) -> bool {
    fields.iter().any( |field| field.is_valid(value))
}

fn ticket_is_valid(fields: &Vec<Field>, ticket: &Vec<u32> ) -> bool {
    ticket.iter().all(|value| value_is_possibly_valid(fields, *value))
}

fn is_one_to_one(field_map: &Vec<Vec<Field>>) -> bool {
    field_map.iter().all(|fields| fields.len() == 1)
}

fn count_instances(field_map: &Vec<Vec<Field>>, field: &Field) -> u32 {
    field_map.iter().map(|fields|{
        if fields.contains(field) {
            1
        } else {
            0
        }
    }).sum()
}

fn part_2(fields: &Vec<Field>, tickets: &Vec<Vec<u32>>) {
    let valid_tickets : Vec<Vec<u32>> = tickets.iter().filter(|ticket|ticket_is_valid(fields, ticket)).cloned().collect();

    let mut field_map : Vec<Vec<Field>> = (0..20).map(|_| fields.clone()).collect();

    for ticket in valid_tickets.iter() {
        for (i, value) in ticket.iter().enumerate() {
            field_map[i] = field_map[i].iter().filter(|field|field.is_valid(*value)).cloned().collect();
        }
    }

    while !is_one_to_one(&field_map) {
        println!("Attempting to Reduce Field Map");
        let fixed_fields : Vec<Field>= fields.iter().filter(|field|count_instances(&field_map, field)==1).cloned().collect();

        for field in fixed_fields {
            for i in 0..20 {
                if  field_map[i].contains(&field) {
                    field_map[i] = vec![field.clone()];
                }
            }
        }

        for i in 0..20 {
            if field_map[i].len() == 1 {
                let pinned_field = field_map[i][0].clone();
                for j in 0..i {
                    field_map[j] = field_map[j].iter().filter(|x| **x != pinned_field).cloned().collect();
                }
                for j in i+1..20 {
                    field_map[j] = field_map[j].iter().filter(|x| **x != pinned_field).cloned().collect();
                }
            }
        } 

        for i in 0..20 {
            println!("  Position {} could be:", i);
            for field in field_map[i].iter() {
                println!("    {}", field.name);
            }
        }
    }

    let final_map : Vec<(usize,Field)> = field_map.iter().enumerate().map(|(i,x)|(i, x[0].clone())).collect();
    let dest_fields : Vec<(usize,Field)> = final_map.iter().filter(|(_,x)| x.name.starts_with("departure")).cloned().collect();

    let my_ticket : Vec<u64> = vec![191,139,59,79,149,83,67,73,167,181,173,61,53,137,71,163,179,193,107,197];
    let dest_vales : Vec<u64> = dest_fields.iter().map(|(i,_)|my_ticket[*i]).collect();

    println!("{:?}", dest_vales);
    println!("{:?}", dest_vales.iter().fold(1, |x,y|x*y));

}

fn main() {
    let legal_value_lines = read_lines("./legal_values.txt");
    let nearby_ticket_lines = read_lines("./nearby_tickets.txt");

    let fields : Vec<Field> = legal_value_lines.iter().map(|line|parse_field(line)).collect();
    let tickets : Vec<Vec<u32>> = nearby_ticket_lines.iter().map(|line|parse_ticket(line)).collect();


    part_1(&fields, &tickets);
    part_2(&fields, &tickets);
}

fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}