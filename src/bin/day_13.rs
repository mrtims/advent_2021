use std::collections::HashMap;

use advent_2021::*;
use colored::*;
use itertools::Itertools;

type Paper = HashMap<(usize, usize), bool>;

#[test]
fn example_1() {
    let input = vec![
        "6,10".to_string(),
        "0,14".to_string(),
        "9,10".to_string(),
        "0,3".to_string(),
        "10,4".to_string(),
        "4,11".to_string(),
        "6,0".to_string(),
        "6,12".to_string(),
        "4,1".to_string(),
        "0,13".to_string(),
        "10,12".to_string(),
        "3,4".to_string(),
        "3,0".to_string(),
        "8,4".to_string(),
        "1,10".to_string(),
        "2,14".to_string(),
        "8,10".to_string(),
        "9,0".to_string(),
    ];

    let mut paper = parse_paper(&input);
    fold_y(&mut paper, 7);
    assert_eq!(paper.len(), 17);
    print(&paper, 11, 7);
}

#[test]
fn example_2() {
    let input = vec![
        "6,10".to_string(),
        "0,14".to_string(),
        "9,10".to_string(),
        "0,3".to_string(),
        "10,4".to_string(),
        "4,11".to_string(),
        "6,0".to_string(),
        "6,12".to_string(),
        "4,1".to_string(),
        "0,13".to_string(),
        "10,12".to_string(),
        "3,4".to_string(),
        "3,0".to_string(),
        "8,4".to_string(),
        "1,10".to_string(),
        "2,14".to_string(),
        "8,10".to_string(),
        "9,0".to_string(),
    ];

    let mut paper = parse_paper(&input);
    let folds = vec!["fold along y=7".to_string(), "fold along x=5".to_string()];
    for fold in folds.iter().map(|f| parse_fold(f)) {
        fold_paper(&mut paper, fold);
    }

    assert_eq!(paper.len(), 16);
    print(&paper, 6, 6);
}

enum Fold {
    X(usize),
    Y(usize),
}

fn fold_y(paper: &mut Paper, fold: usize) {
    let points: Vec<(usize, usize)> = paper.iter().filter(|(_, v)| **v).map(|(k, _)| *k).collect();
    for (x, y) in points {
        if y > fold {
            paper.insert((x, fold - (y - fold)), true);
            paper.remove(&(x, y));
        }
    }
}

fn fold_x(paper: &mut Paper, fold: usize) {
    let points: Vec<(usize, usize)> = paper.iter().filter(|(_, v)| **v).map(|(k, _)| *k).collect();
    for (x, y) in points {
        if x > fold {
            paper.insert((fold - (x - fold), y), true);
            paper.remove(&(x, y));
        }
    }
}

fn fold_paper(paper: &mut Paper, fold: Fold) {
    match fold {
        Fold::X(fold) => fold_x(paper, fold),
        Fold::Y(fold) => fold_y(paper, fold),
    }
}

fn parse_point(input: &str) -> (usize, usize) {
    input
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect_tuple()
        .unwrap()
}

fn parse_paper(input: &[String]) -> Paper {
    let mut paper: Paper = HashMap::new();
    for point in input.iter().map(|line| parse_point(line)) {
        paper.insert(point, true);
    }
    paper
}

fn parse_fold(input: &str) -> Fold {
    let fold = input
        .split('=')
        .last()
        .map(|s| str::parse::<usize>(s).ok())
        .flatten()
        .unwrap();
    if input.starts_with("fold along y=") {
        Fold::Y(fold)
    } else {
        Fold::X(fold)
    }
}

fn print(paper: &Paper, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            if paper.contains_key(&(x, y)) {
                print!("{}", "  ".on_white());
            } else {
                print!("  ");
            }
        }
        print!("\n");
    }
}

fn main() {
    let input = input::lines("./input/day_13_paper.txt").unwrap();
    let folds = input::lines("./input/day_13_folds.txt").unwrap();
    // part 1:
    {
        let mut paper = parse_paper(&input);
        fold_x(&mut paper, 655);
        println!("#1:{}", paper.len());
    }

    // part 2:
    {
        let mut paper = parse_paper(&input);
        for fold in folds.iter().map(|f| parse_fold(f)) {
            fold_paper(&mut paper, fold);
        }
        println!("#2:");
        print(&paper, 40, 6);
    }
}
