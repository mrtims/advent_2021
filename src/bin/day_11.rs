use advent_2021::*;

#[test]
fn example_1() {
    let input = vec![
        "5483143223".to_string(),
        "2745854711".to_string(),
        "5264556173".to_string(),
        "6141336146".to_string(),
        "6357385478".to_string(),
        "4167524645".to_string(),
        "2176841721".to_string(),
        "6882881134".to_string(),
        "4846848554".to_string(),
        "5283751526".to_string(),
    ];

    let mut octo = parse(&input);
    let flashes: u32 = (0..100).map(|_| step(&mut octo)).sum();
    assert_eq!(1656, flashes);
}

fn step(octo: &mut Vec<Vec<u32>>) -> u32 {
    let mut flashes = 0;

    // Increment energy by one
    for line in octo.iter_mut() {
        for oc in line.iter_mut() {
            *oc += 1;
        }
    }

    // Check all positions for a flash (if it flashes, it will hit all neighbours recursively, so we only need to do one pass)
    let height = octo.len();
    let width = octo[0].len();
    for y in 0..height {
        for x in 0..width {
            flashes += flash_octo(octo, x, y);
        }
    }

    // Reset the ones that flashed
    for line in octo.iter_mut() {
        for oc in line.iter_mut() {
            if *oc == u32::MAX {
                *oc = 0;
            }
        }
    }

    flashes
}

fn flash_octo(octo: &mut Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut flashes = 0;

    let height = octo.len();
    let width = octo[0].len();
    // Use u32::MAX as a sentinel value for an already flashed octopus.
    // Saturating Add can then be used to add only to unflashed octopi.
    if octo[y][x] > 9 && octo[y][x] != u32::MAX {
        flashes += 1;
        octo[y][x] = u32::MAX;
        // Recursively flash neighbours
        if y > 0 {
            flashes += add_then_flash_octo(octo, x, y - 1);
            if x > 0 {
                flashes += add_then_flash_octo(octo, x - 1, y - 1);
            }
            if x < width - 1 {
                flashes += add_then_flash_octo(octo, x + 1, y - 1);
            }
        }
        if y < height - 1 {
            flashes += add_then_flash_octo(octo, x, y + 1);
            if x > 0 {
                flashes += add_then_flash_octo(octo, x - 1, y + 1);
            }
            if x < width - 1 {
                flashes += add_then_flash_octo(octo, x + 1, y + 1);
            }
        }
        if x > 0 {
            flashes += add_then_flash_octo(octo, x - 1, y);
        }
        if x < width - 1 {
            flashes += add_then_flash_octo(octo, x + 1, y);
        }
    }

    flashes
}

fn add_then_flash_octo(octo: &mut Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    octo[y][x] = octo[y][x].saturating_add(1);
    flash_octo(octo, x, y)
}

fn parse(input: &[String]) -> Vec<Vec<u32>> {
    input
        .iter()
        .map(|x| {
            x.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn main() {
    let input = input::lines("./input/day_11.txt").unwrap();

    // part 1:
    {
        let mut octo = parse(&input);
        let flashes: u32 = (0..100).map(|_| step(&mut octo)).sum();
        println!("#1: {:?}", flashes);
    }

    // part 2:
    {
        let mut octo = parse(&input);
        let mut i = 0;
        loop {
            i += 1;
            if step(&mut octo) == 100 {
                break;
            }
        }
        println!("#2: {:?}", i);
    }
}
