use std::collections::HashMap;

use advent_2021::*;

const STEPS: isize = 2000;
const MIN_X_VEL: isize = 0;
const MAX_X_VEL: isize = 1000;

const MIN_Y_VEL: isize = -500;
const MAX_Y_VEL: isize = 500;

struct Probe {
    pos: (isize, isize),
    velocity: (isize, isize),
}

impl Probe {
    fn new(velocity: (isize, isize)) -> Self {
        Self {
            pos: (0, 0),
            velocity,
        }
    }

    fn step(&mut self) {
        self.pos.0 += self.velocity.0;
        self.pos.1 += self.velocity.1;
        if self.velocity.0 < 0 {
            self.velocity.0 += 1;
        } else if self.velocity.0 > 0 {
            self.velocity.0 -= 1;
        }
        // else nothing
        self.velocity.1 -= 1;
    }

    fn is_in_target(&self, x: (isize, isize), y: (isize, isize)) -> bool {
        self.pos.0 >= x.0 && self.pos.0 <= x.1 && self.pos.1 >= y.0 && self.pos.1 <= y.1
    }
}

fn parse_range(input: &str) -> (isize, isize) {
    let mut range = input.split("..");
    (
        range.next().unwrap().parse::<isize>().unwrap(),
        range.last().unwrap().parse::<isize>().unwrap(),
    )
}

type Paths = HashMap<(isize, isize), Vec<(isize, isize)>>;

fn find_all_intersecting_paths(target_x: (isize, isize), target_y: (isize, isize)) -> Paths {
    let mut paths = HashMap::new();
    for x_velocity in MIN_X_VEL..=MAX_X_VEL {
        for y_velocity in MIN_Y_VEL..=MAX_Y_VEL {
            let mut probe = Probe::new((x_velocity, y_velocity));
            let mut path = Vec::new();
            for _ in 0..STEPS {
                probe.step();
                path.push(probe.pos);
                if probe.is_in_target(target_x, target_y) {
                    paths.insert((x_velocity, y_velocity), path.clone());
                    break;
                }
            }
        }
    }
    paths
}

fn maximum_height(path: &[(isize, isize)]) -> isize {
    path.iter().max_by_key(|(_, y)| y).unwrap().1
}

#[test]
fn example_1() {
    let input = "target area: x=20..30, y=-10..-5";
    let mut target = input.split(": x=").last().unwrap().split(", y=");
    let target_x = parse_range(&target.next().unwrap());
    let target_y = parse_range(&target.last().unwrap());

    let paths = find_all_intersecting_paths(target_x, target_y);
    let max = paths
        .iter()
        .max_by_key(|(_, path)| maximum_height(path))
        .unwrap();

    println!("{:?}: {}", max.0, maximum_height(max.1));
}
fn main() {
    let file = input::lines("./input/day_17.txt").unwrap();
    let input = file.iter().next().unwrap();

    let mut target = input.split(": x=").last().unwrap().split(", y=");
    let target_x = parse_range(&target.next().unwrap());
    let target_y = parse_range(&target.last().unwrap());

    let paths = find_all_intersecting_paths(target_x, target_y);

    let max = paths
        .iter()
        .max_by_key(|(_, path)| maximum_height(path))
        .unwrap();
    // part 1:
    {
        println!("{:?}: {}", max.0, maximum_height(max.1));
    }
    // part 2:
    {
        println!("#2:{}", paths.len());
    }
}
