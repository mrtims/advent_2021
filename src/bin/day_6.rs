use advent_2021::*;

#[test]
fn example_1() {
    let mut fish = vec![3, 4, 3, 1, 2];
    for _ in 1..=18 {
        fish = step(fish);
    }
    assert_eq!(fish.len(), 26);
    for _ in 19..=80 {
        fish = step(fish);
    }
    assert_eq!(fish.len(), 5934);
}

fn step(fish: Vec<i64>) -> Vec<i64> {
    let (spawning, mut remaining): (Vec<i64>, Vec<i64>) =
        fish.iter().map(|f| *f - 1).partition(|f| *f < 0);
    for _ in spawning {
        remaining.push(6);
        remaining.push(8)
    }
    remaining
}

#[test]
fn example_2() {
    let fish = vec![3, 4, 3, 1, 2];
    let mut fish_count = count_fish(fish);
    for _ in 1..=256 {
        fish_count = step_2(fish_count);
    }
    assert_eq!(sum(&fish_count), 26984457539);
}

struct FishCount(i64, i64, i64, i64, i64, i64, i64, i64, i64);

fn count_fish(fish: Vec<i64>) -> FishCount {
    FishCount(
        fish.iter().filter(|f| **f == 0).count() as i64,
        fish.iter().filter(|f| **f == 1).count() as i64,
        fish.iter().filter(|f| **f == 2).count() as i64,
        fish.iter().filter(|f| **f == 3).count() as i64,
        fish.iter().filter(|f| **f == 4).count() as i64,
        fish.iter().filter(|f| **f == 5).count() as i64,
        fish.iter().filter(|f| **f == 6).count() as i64,
        fish.iter().filter(|f| **f == 7).count() as i64,
        fish.iter().filter(|f| **f == 8).count() as i64,
    )
}

fn sum(fish: &FishCount) -> i64 {
    fish.0 + fish.1 + fish.2 + fish.3 + fish.4 + fish.5 + fish.6 + fish.7 + fish.8
}

fn step_2(fish: FishCount) -> FishCount {
    FishCount(
        fish.1,
        fish.2,
        fish.3,
        fish.4,
        fish.5,
        fish.6,
        fish.7 + fish.0,
        fish.8,
        fish.0,
    )
}

fn read_input_file(filename: &str) -> Vec<i64> {
    if let Ok(lines) = input::lines(filename) {
        lines
            .first()
            .unwrap()
            .split(',')
            .filter_map(|n| str::parse::<i64>(n).ok())
            .collect()
    } else {
        Vec::new()
    }
}

fn main() {
    let mut fish = read_input_file("./input/day_6.txt");
    // part 1
    for _ in 1..=80 {
        fish = step(fish);
    }
    println!("{}", fish.len());

    // part 2
    let mut fish_count = count_fish(fish);
    for _ in 1..=256 {
        fish_count = step_2(fish_count);
    }
    println!("{}", sum(&fish_count));
}
