#[macro_use] extern crate lazy_static;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;

struct Bag<'a> {
    bag_name: String,
    contains: Vec<(u32, &'a Bag<'a>)>,
    contained_in: Vec<&'a Bag<'a>>
}

impl<'a> Bag<'a> {
    fn new(bag_name: String ) -> Bag<'a> {
        Bag{ bag_name: bag_name,
            contains: Vec::new(),
            contained_in: Vec::new(),
        }
    }
}

struct BagTracker<'a> {
    bag_map: HashMap<String, Bag<'a>>
}

impl<'a> BagTracker<'a> {
    fn new() -> BagTracker<'a> {
        BagTracker{ bag_map: HashMap::new() }
    }

    fn add_bag(&mut self, bag_name: &String)
    {
        self.bag_map.insert(
            bag_name.to_string(),
            Bag::new(bag_name.to_string()));
    }

    fn get_bag_or_add<'b>(&'b mut self, bag_name: &String) -> &'b mut Bag<'a>
    {
        if !self.bag_map.contains_key(bag_name) {
            self.add_bag(bag_name);
        }

        self.bag_map.get_mut(bag_name).unwrap()
    }

    fn add_bag_relationship<'b>(&'b mut self, src_bag_name: &String, dst_bag_name: &String, quantity: u32)
    {
        let src_bag = self.get_bag_or_add(src_bag_name);
        let dst_bag = self.get_bag_or_add(dst_bag_name);

        src_bag.contains.push((quantity, dst_bag));
        dst_bag.contained_in.push(src_bag);
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

}

fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}