use advent_2021::*;

#[test]
fn example() {
    let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(count(&data), 7)
}

#[test]
fn example_2() {
    let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(count_2(&data), 5)
}

fn count(points: &[i32]) -> usize {
    points.windows(2).filter(|x| x[0] < x[1]).count()
}

fn count_2(points: &[i32]) -> usize {
    points
        .windows(4)
        .filter(|x| (x[0] + x[1] + x[2]) < (x[1] + x[2] + x[3]))
        .count()
}

fn main() {
    let data = input::as_1d("./input/day_1.txt");
    println!("#1 Count is {}", count(&data));
    println!("#2 Count is {}", count_2(&data));
}
