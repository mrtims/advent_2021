use advent_2021::*;

#[test]
fn example_1() {
    let crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    let cost = (0..=16).map(|n| cost(n, &crabs)).min();
    assert_eq!(Some(37), cost);
}

fn cost(n: i32, crabs: &[i32]) -> i32 {
    crabs.iter().map(|crab| i32::abs(*crab - n)).sum()
}

#[test]
fn example_2() {
    let crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    let cost = (0..=16).map(|n| cost_2(n, &crabs)).min();
    assert_eq!(Some(168), cost);
}

fn cost_2(n: i32, crabs: &[i32]) -> i32 {
    crabs
        .iter()
        .map(|crab| (0..=i32::abs(*crab - n)).sum::<i32>())
        .sum()
}

fn main() {
    let crabs = input::as_csv("input/day_7.txt");
    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();
    // part 1:
    {
        let result = (*min..=*max).map(|n| cost(n, &crabs)).min().unwrap();
        println!("#1: {}", result);
    }

    // part 2:
    {
        let result = (*min..=*max).map(|n| cost_2(n, &crabs)).min().unwrap();
        println!("#2: {}", result);
    }
}
