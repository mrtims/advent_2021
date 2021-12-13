use std::collections::{HashMap, HashSet};

use advent_2021::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Cave {
    Start,
    Big(String),
    Small(String),
    End,
}

fn parse_cave(input: &str) -> Cave {
    if input == "start" {
        Cave::Start
    } else if input == "end" {
        Cave::End
    } else if input.chars().all(|c| c.is_ascii_lowercase()) {
        Cave::Small(input.to_string())
    } else {
        Cave::Big(input.to_string())
    }
}

fn parse_join(input: &str) -> (Cave, Cave) {
    let caves: Vec<&str> = input.split('-').collect();
    if caves.len() == 2 {
        (parse_cave(caves[0]), parse_cave(caves[1]))
    } else {
        panic!();
    }
}

fn parse_layout(input: &[String]) -> HashMap<Cave, HashSet<Cave>> {
    let mut layout = HashMap::new();
    for line in input {
        let (from, to) = parse_join(line);
        layout
            .entry(from.clone())
            .or_insert_with(HashSet::new)
            .insert(to.clone());
        layout.entry(to).or_insert_with(HashSet::new).insert(from);
    }
    layout
}

#[test]
fn example_1() {
    let input = vec![
        "start-A".to_string(),
        "start-b".to_string(),
        "A-c".to_string(),
        "A-b".to_string(),
        "b-d".to_string(),
        "A-end".to_string(),
        "b-end".to_string(),
    ];
    let layout = parse_layout(&input);
    println!("{:?}", layout);
}

fn main() {
    let input = input::lines("./input/day_12.txt").unwrap();

    // part 1:
    {
        let layout = parse_layout(&input);
        println!("{:?}", layout);
        println!("#1: {:?}", "todo");
    }

    // part 2:
    {
        println!("#2: {:?}", "todo");
    }
}
