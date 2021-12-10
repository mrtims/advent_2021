use advent_2021::*;

const PAIRS: [(char, char); 4] = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];

#[test]
fn example_1() {
    let input = vec![
        "[({(<(())[]>[[{[]{<()<>>".to_string(),
        "[(()[<>])]({[<{<<[]>>(".to_string(),
        "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
        "(((({<>}<{<{<>}{[]{[]{}".to_string(),
        "[[<[([]))<([[{}[[()]]]".to_string(),
        "[{[{({}]{}}([{[{{{}}([]".to_string(),
        "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
        "[<(<(<(<{}))><([]([]()".to_string(),
        "<{([([[(<>()){}]>(<<{{".to_string(),
        "<{([{{}}[<[[[<>{}]]]>[]]]".to_string(),
    ];
    let x = get_illegal(&input);
    let points: u32 = x.into_iter().map(points).sum();
    assert_eq!(26397, points);
}

fn get_illegal(input: &[String]) -> Vec<char> {
    let mut illegal = Vec::new();
    for line in input {
        let mut owing = Vec::new();
        for c in line.chars() {
            if is_opening(c) {
                owing.push(c);
            } else {
                let opener = opener_for(c);
                if let Some(character) = owing.iter().last() {
                    if *character == opener {
                        owing.pop();
                    } else {
                        illegal.push(c);
                        println!("Expected {}, but found {} instead.", '.', c);
                        break;
                    }
                } else {
                    println!("Expected NOTHING, but found {} instead.", c);
                }
            }
        }
        // ignore incomplete lines
    }
    illegal
}

fn points(x: char) -> u32 {
    match x {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }
}

fn is_opening(c: char) -> bool {
    for (open, _) in PAIRS.iter() {
        if c == *open {
            return true;
        }
    }
    false
}

fn opener_for(c: char) -> char {
    for (open, close) in PAIRS.iter() {
        if c == *close {
            return *open;
        }
    }
    '-'
}

#[test]
fn example_2() {
    let input = vec![
        "[({(<(())[]>[[{[]{<()<>>".to_string(),
        "[(()[<>])]({[<{<<[]>>(".to_string(),
        "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
        "(((({<>}<{<{<>}{[]{[]{}".to_string(),
        "[[<[([]))<([[{}[[()]]]".to_string(),
        "[{[{({}]{}}([{[{{{}}([]".to_string(),
        "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
        "[<(<(<(<{}))><([]([]()".to_string(),
        "<{([([[(<>()){}]>(<<{{".to_string(),
        "<{([{{}}[<[[[<>{}]]]>[]]]".to_string(),
    ];
    let incomplete = incomplete_lines(&input);
    println!("{:?}", incomplete);
    assert_eq!(incomplete.len(), 5);

    let owing: Vec<Vec<char>> = incomplete.into_iter().map(|x| owing(&x)).collect();
    let mut points: Vec<_> = owing.iter().map(points_per_line).collect();
    points.sort();
    println!("{:?}", points);
}

fn points_2(x: char) -> u32 {
    match x {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!(),
    }
}

fn incomplete_lines(input: &[String]) -> Vec<String> {
    let mut incomplete = Vec::new();
    for line in input {
        let mut illegal = false;
        let mut owing = Vec::new();
        for c in line.chars() {
            if is_opening(c) {
                owing.push(c);
            } else {
                let opener = opener_for(c);
                if let Some(character) = owing.iter().last() {
                    if *character == opener {
                        owing.pop();
                    } else {
                        println!("Expected {}, but found {} instead.", '.', c);
                        illegal = true;
                        break;
                    }
                } else {
                    println!("Expected NOTHING, but found {} instead.", c);
                }
            }
        }
        if !illegal {
            incomplete.push(line.clone());
        }
    }
    incomplete
}

fn owing(line: &str) -> Vec<char> {
    let mut owing = Vec::new();
    for c in line.chars() {
        if is_opening(c) {
            owing.push(c);
        } else {
            let opener = opener_for(c);
            if let Some(character) = owing.iter().last() {
                if *character == opener {
                    owing.pop();
                } else {
                    println!("Expected {}, but found {} instead.", '.', c);
                    break;
                }
            } else {
                println!("Expected NOTHING, but found {} instead.", c);
            }
        }
    }
    owing.reverse();
    owing
}

fn points_per_line(line: &Vec<char>) -> u64 {
    line.iter()
        .fold(0u64, |acc, x| acc * 5 + points_2(*x) as u64)
}

fn main() {
    let input = input_lines("./input/day_10.txt").unwrap();
    // part 1:
    {
        let x = get_illegal(&input);
        let points: u32 = x.into_iter().map(points).sum();
        println!("#1: {}", points);
    }

    // part 2:
    {
        let incomplete = incomplete_lines(&input);
        let owing: Vec<Vec<char>> = incomplete.into_iter().map(|x| owing(&x)).collect();
        let mut points: Vec<_> = owing.iter().map(points_per_line).collect();
        points.sort();
        println!("#2: {:?}", points[(points.len() / 2)]);
    }
}
