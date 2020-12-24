#![feature(str_split_once)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Debug)]
enum RuleType {
    CharRule{c:char},
    MetaRule{id_sequences:Vec<Vec<u32>>}
}

fn parse_rule(line: &String) -> RuleType {
    if line.starts_with('"') && line.ends_with('"') {
        RuleType::CharRule{c:line.chars().nth(1).unwrap()}
    } else {
        RuleType::MetaRule{
            id_sequences: line.split_terminator("| ")
                .map(|x|{
                    x.split_terminator(' ')
                        .map(|y| y.parse::<u32>().unwrap())
                        .collect()})
                .collect() }
    }
}

fn parse_rule_line(line: &String, rules: &mut HashMap<u32, RuleType>) {

    let (id, rule) = line.split_once(": ").unwrap();

    rules.insert((*id).parse::<u32>().unwrap(), parse_rule(&rule.to_string()));
}

fn evaluate_meta_rule_sequence(ids: &Vec<u32>, rules: &HashMap<u32, RuleType>, chars:&[char]) -> Option<usize> {
    if ids.len() > chars.len() {
        None
    } else {
        let mut index = 0;

        for id in ids {
            if let Some(consumed) = evaluate_rule(*id, rules, &chars[index..]) {
                index += consumed;
            } else {
                return None;
            }
        }
        Some(index)
    }
}

fn hungry_match(id:u32, rules: &HashMap<u32, RuleType>, chars:&[char]) -> Option<usize> {
    let mut index = 0;
    while let Some(consumed) = evaluate_rule(id, rules, &chars[index..]) {
        index += consumed;
    }
    //println!("Hungry Matching Rule {} consumed {} characters", id, index);
    if index > 0 {
        return Some(index);
    }
    else{
        return None;
    }
}


fn evaluate_rule(id:u32, rules: &HashMap<u32, RuleType>, chars:&[char]) -> Option<usize> {
    //println!("Applying {} {:?} against {:?}", id, rules[&id], chars);

    if id == 0 {
        if let Some(consumed_42) = hungry_match( 42, rules, chars ) {
            if let Some(consumed_31) = hungry_match( 31, rules, &chars[consumed_42..] ) {
                if consumed_42 > consumed_31 && consumed_42 >= 2 && consumed_31 >= 1{
                    return Some(consumed_42 + consumed_31);
                } else {
                }
            }
        }
        return None;

    } else {

        match &rules[&id] {
            RuleType::CharRule{c} => {
                if chars[0] == *c {
                    Some(1)}
                else{
                    None
                }},

            RuleType::MetaRule{id_sequences} => {
                for id_sequence in id_sequences {
                    if let Some(consumed) = evaluate_meta_rule_sequence(id_sequence, rules, chars) {
                        return Some(consumed)
                    }
                }
                None
            }
        }
    }
}

fn make_rules_dict(lines:&Vec<String>) -> HashMap::<u32, RuleType> {
    let mut rule_dict = HashMap::<u32, RuleType>::new();

    for line in lines {
        parse_rule_line(&line, &mut rule_dict);
    }

    rule_dict
}

fn check_perfect_match(id:u32, rule_dict: &HashMap<u32, RuleType>, message: &String) -> bool {

    //println!("matching {}", message);

    let chars : Vec<char> = message.chars().collect();
    if let Some(consumed) = evaluate_rule(id, &rule_dict, &chars) {
        if consumed == chars.len() {
            //println!("Match!");
            true
        } else {
            //println!("Incomplete Match");
            false
        }
    } else {
        //println!("No Match");
        false
    }
}

fn main() {
    let rule_lines = read_lines("./rules.txt");
    let message_lines = read_lines("./messages.txt");

    let rule_dict = make_rules_dict(&rule_lines);
    
    println!("{}", message_lines.iter().filter(|x| check_perfect_match(0, &rule_dict, x)).count());
}

fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}

#[test]
fn p1_example_1 () {
    let rules = vec![
        "0: 1 2".to_string(),
        "1: \"a\"".to_string(),
        "2: 1 3 | 3 1".to_string(),
        "3: \"b\"".to_string()];

    let rule_dict = make_rules_dict(&rules);

    assert_eq!(evaluate_rule(0, &rule_dict, &['a','a','b']), Some(3));
    assert_eq!(evaluate_rule(0, &rule_dict, &['a','b','a']), Some(3));
    assert_eq!(evaluate_rule(0, &rule_dict, &['a','a','a']), None);
    assert_eq!(evaluate_rule(0, &rule_dict, &['a','b']), None);
    assert_eq!(evaluate_rule(0, &rule_dict, &['a','b','a','b']), Some(3));
}

#[test]
fn p1_example_2 () {
    let rules = vec![
        "0: 4 1 5".to_string(),
        "1: 2 3 | 3 2".to_string(),
        "2: 4 4 | 5 5".to_string(),
        "3: 4 5 | 5 4".to_string(),
        "4: \"a\"".to_string(),
        "5: \"b\"".to_string()];

    let rule_dict = make_rules_dict(&rules);

    assert_eq!(evaluate_rule(0, &rule_dict, &['a','a','a','a','b','b']), Some(6));
    assert_eq!(evaluate_rule(0, &rule_dict, &['a','b','a','b','b','b']), Some(6));
    assert_eq!(evaluate_rule(0, &rule_dict, &['a','a','a','a','b','b','b']), Some(6));
    assert_eq!(evaluate_rule(0, &rule_dict, &['b','a','b','a','b','a']), None);
}

#[test]
fn p2_example_1 () {
    let rules = vec![
        "42: 9 14 | 10 1".to_string(),
        "9: 14 27 | 1 26".to_string(),
        "10: 23 14 | 28 1".to_string(),
        "1: \"a\"".to_string(),
        "11: 42 31".to_string(),
        "5: 1 14 | 15 1".to_string(),
        "19: 14 1 | 14 14".to_string(),
        "12: 24 14 | 19 1".to_string(),
        "16: 15 1 | 14 14".to_string(),
        "31: 14 17 | 1 13".to_string(),
        "6: 14 14 | 1 14".to_string(),
        "2: 1 24 | 14 4".to_string(),
        "0: 8 11".to_string(),
        "13: 14 3 | 1 12".to_string(),
        "15: 1 | 14".to_string(),
        "17: 14 2 | 1 7".to_string(),
        "23: 25 1 | 22 14".to_string(),
        "28: 16 1".to_string(),
        "4: 1 1".to_string(),
        "20: 14 14 | 1 15".to_string(),
        "3: 5 14 | 16 1".to_string(),
        "27: 1 6 | 14 18".to_string(),
        "14: \"b\"".to_string(),
        "21: 14 1 | 1 14".to_string(),
        "25: 1 1 | 1 14".to_string(),
        "22: 14 14".to_string(),
        "8: 42".to_string(),
        "26: 14 22 | 1 20".to_string(),
        "18: 15 15".to_string(),
        "7: 14 5 | 1 21".to_string(),
        "24: 14 1".to_string()];

    let rule_dict = make_rules_dict(&rules);

    assert_eq!(check_perfect_match(0, &rule_dict, &"bbabbbbaabaabba".to_string()), true);
}