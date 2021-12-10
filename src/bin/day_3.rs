use advent_2021::*;

#[test]
fn example_one() {
    let input: Vec<String> = vec![
        "00100".to_string(),
        "11110".to_string(),
        "10110".to_string(),
        "10111".to_string(),
        "10101".to_string(),
        "01111".to_string(),
        "00111".to_string(),
        "11100".to_string(),
        "10000".to_string(),
        "11001".to_string(),
        "00010".to_string(),
        "01010".to_string(),
    ];
    let most_common: Vec<u8> = (0..5).map(|i| most_common(&input, i)).collect();
    assert_eq!(most_common[0], 1);
    assert_eq!(most_common[1], 0);
    assert_eq!(most_common[2], 1);
    assert_eq!(most_common[3], 1);
    assert_eq!(most_common[4], 0);
}

fn most_common(input: &[String], pos: usize) -> u8 {
    let (zero, one): (Vec<&String>, Vec<&String>) =
        input.iter().partition(|x| x.chars().nth(pos) == Some('0'));
    if zero.len() > one.len() {
        0
    } else {
        1
    }
}

fn least_common(input: &[String], pos: usize) -> u8 {
    let (zero, one): (Vec<&String>, Vec<&String>) =
        input.iter().partition(|x| x.chars().nth(pos) == Some('0'));
    if zero.len() > one.len() {
        1
    } else {
        0
    }
}

fn main() {
    let input = input_lines("./input/day_3.txt").unwrap();
    // part 1
    {
        let most_common_1: Vec<u8> = (0..12).map(|i| most_common(&input, i)).collect();
        let least_common_1: Vec<u8> = (0..12).map(|i| least_common(&input, i)).collect();
        println!("Most:{:?}, Least:{:?}", most_common_1, least_common_1);
    }

    // part 2
    {
        let mut oxy = input.clone();
        let mut pos = 0;
        while oxy.len() > 1 {
            let most_common_1: Vec<u8> = (0..12).map(|i| most_common(&oxy, i)).collect();
            oxy.retain(|x| {
                x.chars().nth(pos).map(|x| x.to_digit(10)).flatten()
                    == Some(most_common_1[pos] as u32)
            });
            pos += 1;
        }
        println!("oxy: {:?}", oxy);
    }
    {
        let mut co2 = input;
        let mut pos = 0;
        while co2.len() > 1 {
            let least_common_1: Vec<u8> = (0..12).map(|i| least_common(&co2, i)).collect();
            co2.retain(|x| {
                x.chars().nth(pos).map(|x| x.to_digit(10)).flatten()
                    == Some(least_common_1[pos] as u32)
            });
            pos += 1;
        }
        println!("CO2: {:?}", co2);
    }
}
