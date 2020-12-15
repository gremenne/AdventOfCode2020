#[macro_use] extern crate lazy_static;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn parse_ids1<'a>(line: &'a String) -> impl Iterator<Item=u32> +'a {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([\d]+)").unwrap();
    }

    RE.captures_iter(line).map(|x|x[1].parse::<u32>().unwrap())
}

fn part_1(lines: &Vec<String>)
{
    let earliest_departure = lines[0].parse::<u32>().unwrap();     // 1001798
    let ids = parse_ids1(&lines[1]);

    let mut closest_departures : Vec<(u32, u32)> = ids.map(|x|{
        (x, ((earliest_departure/x)+1) *x)
    }).collect();

    closest_departures.sort_by(|(_,x), (_,y)| x.cmp(y) );
    let (id, timestamp) = closest_departures[0];
    println!("{}", (timestamp- earliest_departure) * id);
}

fn parse_ids2<'a>(line: &'a String) -> impl Iterator<Item=(usize,u64)> +'a {

    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<padding>(x,)*)(?P<value>[\d]+)").unwrap();
    }

    RE.captures_iter(line).map(|x|{
        (x["padding"].split(',').count(), x["value"].parse::<u64>().unwrap())})
}

fn find_base((offset1, value1) : (u64,u64), (offset2, value2) : (u64,u64) ) -> (u64,u64){
    let mut base = offset1;

    loop{
        base += value1;
        if (base + (offset2 as u64))%value2 == 0
        {
            break;
        }
    }

    (base, value1*value2)
}


fn part_2(lines: &Vec<String>)
{
    let offsets_and_ids : Vec<(usize,u64)> = parse_ids2(&lines[1]).collect();

    let (_, base_id) : (usize,u64) = offsets_and_ids[0];

    let mut cumulitive_offsets_and_ids = Vec::<(usize,u64)>::new();

    let _ = offsets_and_ids[1..].iter().fold((0, &mut cumulitive_offsets_and_ids ), |(acc, vec), (offset, value)|{
        vec.push((offset+acc,*value));
        (offset+acc, vec)
    });

    let (final_offset, _) = cumulitive_offsets_and_ids.iter().fold((0, base_id), |base, (offset, id)| find_base(base, ((*offset) as u64,* id)));

    println!("{}", final_offset);

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