use advent_2021::*;

enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
    None,
}

#[test]
fn example() {
    let data = vec![
        Direction::Forward(5),
        Direction::Down(5),
        Direction::Forward(8),
        Direction::Up(3),
        Direction::Down(8),
        Direction::Forward(2),
    ];
    let result = follow_path_1(&data);
    assert_eq!(result, Position(15, 10, 0));
}

#[derive(Debug, PartialEq)]
struct Position(i32, i32, i32);

fn follow_path_1(directions: &[Direction]) -> Position {
    directions
        .iter()
        .fold(Position(0, 0, 0), |pos, dir| match dir {
            Direction::Forward(n) => Position(pos.0 + n, pos.1, 0),
            Direction::Down(n) => Position(pos.0, pos.1 + n, 0),
            Direction::Up(n) => Position(pos.0, pos.1 - n, 0),
            Direction::None => pos,
        })
}

fn follow_path_2(directions: &[Direction]) -> Position {
    directions
        .iter()
        .fold(Position(0, 0, 0), |pos, dir| match dir {
            Direction::Forward(n) => Position(pos.0 + n, pos.1 + (pos.2 * n), pos.2),
            Direction::Down(n) => Position(pos.0, pos.1, pos.2 + n),
            Direction::Up(n) => Position(pos.0, pos.1, pos.2 - n),
            Direction::None => pos,
        })
}

fn parse(s: String) -> Direction {
    let n = s
        .chars()
        .skip_while(|c| !c.is_digit(10))
        .collect::<String>();
    if let Ok(n) = str::parse::<i32>(&n) {
        if s.starts_with("forward") {
            Direction::Forward(n)
        } else if s.starts_with("down") {
            Direction::Down(n)
        } else if s.starts_with("up") {
            Direction::Up(n)
        } else {
            Direction::None
        }
    } else {
        Direction::None
    }
}

fn input_as_direction(filename: &str) -> Vec<Direction> {
    if let Ok(lines) = input_lines(filename) {
        lines.into_iter().map(parse).collect()
    } else {
        Vec::new()
    }
}

fn main() {
    let directions = input_as_direction("./input/day_2.txt");
    let position = follow_path_1(&directions);
    println!(
        "Final #1:{:?}, multiple:{}",
        position,
        position.0 * position.1
    );
    let position = follow_path_2(&directions);
    println!(
        "Final #2:{:?}, multiple:{}",
        position,
        position.0 * position.1
    );
}
