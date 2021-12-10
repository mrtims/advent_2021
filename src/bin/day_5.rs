use advent_2021::*;
use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line(Point, Point);

type Map = HashMap<Point, u32>;

fn add_line(map: &mut Map, line: &Line) {
    if line.0.x == line.1.x {
        // Vertical
        let y1 = min(line.0.y, line.1.y);
        let y2 = max(line.0.y, line.1.y);
        for y in y1..=y2 {
            let point = Point { x: line.0.x, y };
            *map.entry(point).or_insert(0) += 1;
        }
    } else if line.0.y == line.1.y {
        // Horizontal
        let x1 = min(line.0.x, line.1.x);
        let x2 = max(line.0.x, line.1.x);
        for x in x1..=x2 {
            let point = Point { x, y: line.0.y };
            *map.entry(point).or_insert(0) += 1;
        }
    } else {
        // skip
    }
}

fn add_line_including_diagonal(map: &mut Map, line: &Line) {
    if line.0.x == line.1.x {
        // Vertical
        let y1 = min(line.0.y, line.1.y);
        let y2 = max(line.0.y, line.1.y);
        for y in y1..=y2 {
            let point = Point { x: line.0.x, y };
            *map.entry(point).or_insert(0) += 1;
        }
    } else if line.0.y == line.1.y {
        // Horizontal
        let x1 = min(line.0.x, line.1.x);
        let x2 = max(line.0.x, line.1.x);
        for x in x1..=x2 {
            let point = Point { x, y: line.0.y };
            *map.entry(point).or_insert(0) += 1;
        }
    } else {
        // Diagonal
        let mut x = line.0.x;
        let mut y = line.0.y;
        if line.0.x < line.1.x {
            // X increasing
            if line.0.y < line.1.y {
                // Y increasing
                while x <= line.1.x && y <= line.1.y {
                    let point = Point { x, y };
                    *map.entry(point).or_insert(0) += 1;
                    x += 1;
                    y += 1;
                }
            } else {
                // Y decreasing
                while x <= line.1.x && y >= line.1.y {
                    let point = Point { x, y };
                    *map.entry(point).or_insert(0) += 1;
                    x += 1;
                    y -= 1;
                }
            }
        } else {
            // X decreasing
            if line.0.y < line.1.y {
                // Y increasing
                while x >= line.1.x && y <= line.1.y {
                    let point = Point { x, y };
                    *map.entry(point).or_insert(0) += 1;
                    x -= 1;
                    y += 1;
                }
            } else {
                // Y decreasing
                while x >= line.1.x && y >= line.1.y {
                    let point = Point { x, y };
                    *map.entry(point).or_insert(0) += 1;
                    x -= 1;
                    y -= 1;
                }
            }
        }
    }
}

fn point_from_str(x: &str, y: &str) -> Point {
    let x = str::parse::<i32>(x).unwrap();
    let y = str::parse::<i32>(y).unwrap();
    Point { x, y }
}

#[test]
fn example_one() {
    let input = vec![
        "0,9 -> 5,9".to_string(),
        "8,0 -> 0,8".to_string(),
        "9,4 -> 3,4".to_string(),
        "2,2 -> 2,1".to_string(),
        "7,0 -> 7,4".to_string(),
        "6,4 -> 2,0".to_string(),
        "0,9 -> 2,9".to_string(),
        "3,4 -> 1,4".to_string(),
        "0,0 -> 8,8".to_string(),
        "5,5 -> 8,2".to_string(),
    ];

    let mut lines: Vec<Line> = Vec::new();
    for line in input {
        if let Some((a, b)) = line.split_once(" -> ") {
            if let Some((x1, y1)) = a.split_once(',') {
                if let Some((x2, y2)) = b.split_once(',') {
                    lines.push(Line(point_from_str(x1, y1), point_from_str(x2, y2)));
                }
            }
        }
    }

    println!("{:?}", lines);

    let mut map = Map::new();
    for line in &lines {
        add_line(&mut map, line);
    }

    for item in &map {
        println!("{},{}: {}", item.0.x, item.0.y, item.1);
    }

    let two_or_more = map
        .into_values()
        .fold(0, |sum, n| if n >= 2 { sum + 1 } else { sum });
    assert_eq!(5, two_or_more);
}

#[test]
fn example_two() {
    let input = vec![
        "0,9 -> 5,9".to_string(),
        "8,0 -> 0,8".to_string(),
        "9,4 -> 3,4".to_string(),
        "2,2 -> 2,1".to_string(),
        "7,0 -> 7,4".to_string(),
        "6,4 -> 2,0".to_string(),
        "0,9 -> 2,9".to_string(),
        "3,4 -> 1,4".to_string(),
        "0,0 -> 8,8".to_string(),
        "5,5 -> 8,2".to_string(),
    ];

    let mut lines: Vec<Line> = Vec::new();
    for line in input {
        if let Some((a, b)) = line.split_once(" -> ") {
            if let Some((x1, y1)) = a.split_once(',') {
                if let Some((x2, y2)) = b.split_once(',') {
                    lines.push(Line(point_from_str(x1, y1), point_from_str(x2, y2)));
                }
            }
        }
    }

    println!("{:?}", lines);

    let mut map = Map::new();
    for line in &lines {
        add_line_including_diagonal(&mut map, line);
    }

    for item in &map {
        println!("{},{}: {}", item.0.x, item.0.y, item.1);
    }

    let two_or_more = map
        .into_values()
        .fold(0, |sum, n| if n >= 2 { sum + 1 } else { sum });
    assert_eq!(12, two_or_more);
}

fn main() {
    let input = input_lines("./input/day_5.txt").unwrap();
    let mut lines: Vec<Line> = Vec::new();
    for line in input {
        if let Some((a, b)) = line.split_once(" -> ") {
            if let Some((x1, y1)) = a.split_once(',') {
                if let Some((x2, y2)) = b.split_once(',') {
                    lines.push(Line(point_from_str(x1, y1), point_from_str(x2, y2)));
                }
            }
        }
    }
    // part 1
    {
        let mut map = Map::new();
        for line in &lines {
            add_line(&mut map, line);
        }

        let two_or_more = map.into_values().filter(|n| *n >= 2).count();
        println!("{}", two_or_more);
    }

    // part 2
    {
        let mut map = Map::new();
        for line in &lines {
            add_line_including_diagonal(&mut map, line);
        }

        let two_or_more = map.into_values().filter(|n| *n >= 2).count();
        println!("{}", two_or_more);
    }
}
