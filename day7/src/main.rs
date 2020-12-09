#[macro_use] extern crate lazy_static;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

struct Bag {
    bag_name: String,
    contains: Vec<(u32, String)>,
    contained_in: Vec<String>
}

impl Bag {
    fn new(bag_name: String ) -> Bag {
        Bag{ bag_name: bag_name,
            contains: Vec::new(),
            contained_in: Vec::new(),
        }
    }
}

struct BagTracker {
    bag_map: HashMap<String, Bag>,
}

impl BagTracker {
    fn new() -> BagTracker {
        BagTracker{ bag_map: HashMap::new() }
    }

    fn add_bag(&mut self, bag_name: &String)
    {
        if !self.bag_map.contains_key(bag_name) {
            self.bag_map.insert(
                bag_name.to_string(),
                Bag::new(bag_name.to_string()));
        }
    }

    fn add_bag_relationship(&mut self, src_bag_name: &String, dst_bag_name: &String, quantity: u32)
    {
        self.add_bag(src_bag_name);
        self.add_bag(dst_bag_name);

        self.bag_map.get_mut(src_bag_name).unwrap().contains.push((quantity, dst_bag_name.to_string()));
        self.bag_map.get_mut(dst_bag_name).unwrap().contained_in.push(src_bag_name.to_string());
    }

    fn get_bag(&self, bag_name: &String) -> &Bag
    {
        &self.bag_map[bag_name]
    }

    fn get_parents(&self, bag_name: &String) -> Vec<&Bag> {
        self.get_bag(bag_name).contained_in.iter().map(|x| self.get_bag(x)).collect()
    }

    fn get_all_parents(&self, bag_name: &String) -> Vec <&Bag> {
        self.get_parents(bag_name)
            .iter()
            .map(|x| *x )
            .chain(
                self.get_parents(bag_name)
                .iter()
                .map(|x|self.get_all_parents(&x.bag_name))
                .flatten())
            .collect()
    }
}

fn parse_relationships(src_bag_name: &String, relationships: &String, bags: &mut BagTracker)
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+) (\D+) bag[s]?").unwrap();
    }

    if *relationships == "no other bags" {
        return;
    }

    for cap in RE.captures_iter(relationships) {
        bags.add_bag_relationship(src_bag_name, &cap[2].to_string(), cap[1].parse::<u32>().unwrap() )
    }

}

fn parse_bag_rule(line: &String, bags: &mut BagTracker)
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([\w ]+) bags contain (.+)\.$").unwrap();
    }

    if let Some(cap) = RE.captures(line)
    {
        let bag_name = &cap[1].to_string();
        let relationships = &cap[2].to_string();
        bags.add_bag(bag_name);

        parse_relationships(bag_name, relationships, bags);
    }
}

fn main() {
    let lines = read_lines("./input.txt");
    let mut bags = BagTracker::new();

    for line in lines {
        parse_bag_rule(&line, &mut bags);
    }

    let parents : HashSet<_> = bags.get_all_parents(&"shiny gold".to_string()).into_iter().map(|x| x.bag_name.to_string()).collect();

    println!("{}", parents.len());

}

fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}