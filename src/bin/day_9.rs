use std::collections::HashSet;

use advent_2021::*;

#[test]
fn example_one() {
    let input: Vec<Vec<u32>> = vec![
        "2199943210".to_string(),
        "3987894921".to_string(),
        "9856789892".to_string(),
        "8767896789".to_string(),
        "9899965678".to_string(),
    ]
    .iter()
    .map(|x| {
        x.chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>()
    })
    .collect();

    assert_eq!(15, get_risk(&input));
}

fn get_risk(input: &[Vec<u32>]) -> u32 {
    let height = input.len();
    let width = input[0].len();

    let mut risk = 0;
    for x in 0..width {
        for y in 0..height {
            let mut neigh = Vec::new();
            if y > 0 {
                let up = input[y - 1][x];
                neigh.push(up);
            }
            if y < height - 1 {
                let down = input[y + 1][x];
                neigh.push(down);
            }
            if x > 0 {
                let left = input[y][x - 1];
                neigh.push(left);
            }
            if x < width - 1 {
                let right = input[y][x + 1];
                neigh.push(right);
            }
            if let Some(min) = neigh.iter().min() {
                if input[y][x] < *min {
                    risk += input[y][x] + 1;
                }
            }
        }
    }
    risk
}

#[test]
fn example_two() {
    let input: Vec<Vec<u32>> = vec![
        "2199943210".to_string(),
        "3987894921".to_string(),
        "9856789892".to_string(),
        "8767896789".to_string(),
        "9899965678".to_string(),
    ]
    .iter()
    .map(|x| {
        x.chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>()
    })
    .collect();

    let mut basin_sizes = get_basin_sizes(&input);

    basin_sizes.sort();
    basin_sizes.reverse();
    println!("{:?}", basin_sizes);
    let top_3 = basin_sizes.iter().take(3).fold(1, |f, x| f * x);
    assert_eq!(1134, top_3);
}

fn get_basin_sizes(input: &[Vec<u32>]) -> Vec<u32> {
    let height = input.len();
    let width = input[0].len();
    let mut result = Vec::new();
    for x in 0..width {
        for y in 0..height {
            let mut neigh = Vec::new();
            if y > 0 {
                let up = input[y - 1][x];
                neigh.push(up);
            }
            if y < height - 1 {
                let down = input[y + 1][x];
                neigh.push(down);
            }
            if x > 0 {
                let left = input[y][x - 1];
                neigh.push(left);
            }
            if x < width - 1 {
                let right = input[y][x + 1];
                neigh.push(right);
            }
            if let Some(min) = neigh.iter().min() {
                if input[y][x] < *min {
                    // Found low point
                    result.push(basin_size(input, x, y, width, height));
                }
            }
        }
    }
    result
}

fn basin_size(input: &[Vec<u32>], x: usize, y: usize, width: usize, height: usize) -> u32 {
    let bounds = find_bounding_box(input, x, y, width, height);
    (bounds.0.x..bounds.1.x)
        .map(|x| (bounds.0.y..bounds.1.y).map(move |y| input[y][x]))
        .flatten()
        .filter(|&x| x < 9)
        .count() as u32
}

struct Point {
    x: usize,
    y: usize,
}
type BoundingBox = (Point, Point);

fn find_bounding_box(
    input: &[Vec<u32>],
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> BoundingBox {
    let mut bounds: BoundingBox = (Point { x, y }, Point { x, y });
    loop {
        let vertical: Vec<_> = (bounds.0.x..bounds.1.x)
            .map(|x| (input[bounds.0.y][x], input[bounds.1.y][x]))
            .collect();
        let horizontal: Vec<_> = (bounds.0.y..bounds.1.y)
            .map(|y| (input[bounds.0.x][y], input[bounds.1.x][y]))
            .collect();
        let expand_up = bounds.0.y > 0 && vertical.iter().any(|(top, _)| *top < 9);
        let expand_down = bounds.1.y < height - 1 && vertical.iter().any(|(_, bottom)| *bottom < 9);
        let expand_left = bounds.0.x > 0 && horizontal.iter().any(|(left, _)| *left < 9);
        let expand_right = bounds.1.x < width - 1 && horizontal.iter().any(|(_, right)| *right < 9);

        if !expand_up && !expand_down && !expand_left && !expand_right {
            break;
        } else {
            if expand_up {
                bounds.0.y -= 1;
            }
            if expand_down {
                bounds.1.y += 1;
            }
            if expand_left {
                bounds.0.x -= 1;
            }
            if expand_right {
                bounds.1.x += 1;
            }
        }
    }

    bounds
}

fn explore_basin_size(input: &[Vec<u32>], x: usize, y: usize, width: usize, height: usize) -> u32 {
    println!("exploring @ {},{}", x, y);
    let mut points: HashSet<(usize, usize)> = HashSet::new();
    let mut horiz = points_left(input, x, y);
    horiz.extend(points_right(input, x, y, width));

    points.extend(horiz.iter());
    for (x, y) in horiz {
        let mut vert = points_up(input, x, y);
        vert.extend(points_down(input, x, y, height));
        for (x, y) in vert {
            points.extend(points_left(input, x, y).iter());
            points.extend(points_right(input, x, y, width).iter());
        }
    }
    points.len() as u32
}

fn points_left(input: &[Vec<u32>], mut x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    while input[y][x] < 9 {
        result.push((x, y));
        if x == 0 {
            break;
        }
        x -= 1;
    }
    result
}

fn points_right(input: &[Vec<u32>], mut x: usize, y: usize, width: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if x < width {
        while input[y][x] < 9 {
            result.push((x, y));
            if x == width - 1 {
                break;
            }
            x += 1;
        }
    }
    result
}

fn points_up(input: &[Vec<u32>], x: usize, mut y: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    while input[y][x] < 9 {
        result.push((x, y));
        if y == 0 {
            break;
        }
        y -= 1;
    }
    result
}

fn points_down(input: &[Vec<u32>], x: usize, mut y: usize, height: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if y < height {
        while input[y][x] < 9 {
            result.push((x, y));
            if y == height - 1 {
                break;
            }
            y += 1;
        }
    }
    result
}

fn main() {
    let input: Vec<Vec<u32>> = input::lines("./input/day_9.txt")
        .unwrap()
        .iter()
        .map(|x| {
            x.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect();

    // part 1:
    {
        println!("{}", get_risk(&input));
    }

    // part 2
    {
        let mut basin_sizes = get_basin_sizes(&input);

        // reverse sort
        basin_sizes.sort_unstable_by(|a, b| b.cmp(a));
        println!("{:?}", basin_sizes);
        let top_3: u32 = basin_sizes.iter().take(3).product();
        println!("{}", top_3);
    }
}
