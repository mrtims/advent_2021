use std::collections::HashMap;

use itertools::Itertools;

use advent_2021::*;

const ZERO: &str = "abcefg";
const ONE: &str = "cf";
const TWO: &str = "acdeg";
const THREE: &str = "acdfg";
const FOUR: &str = "bcdf";
const FIVE: &str = "abdfg";
const SIX: &str = "abdefg";
const SEVEN: &str = "acf";
const EIGHT: &str = "abcdefg";
const NINE: &str = "abcdfg";

#[test]
fn example_1() {
    let input = vec![
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"
            .to_string(),
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"
            .to_string(),
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"
            .to_string(),
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"
            .to_string(),
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"
            .to_string(),
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"
            .to_string(),
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"
            .to_string(),
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"
            .to_string(),
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            .to_string(),
    ];
    let sum = count_unique(&outputs(&input));
    assert_eq!(26, sum);
}

fn outputs(input: &[String]) -> Vec<String> {
    input
        .iter()
        .map(|x| x.split(" | ").last().unwrap().to_owned())
        .collect()
}

fn count_unique(outputs: &[String]) -> u32 {
    let mut sum = 0;
    for output in outputs {
        let values = output.split(' ');
        for value in values {
            let c = value.len();
            if c == ONE.len() || c == FOUR.len() || c == SEVEN.len() || c == EIGHT.len() {
                sum += 1;
            }
        }
    }
    sum
}

#[test]
fn example_2() {
    let input = vec![
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"
            .to_string(),
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"
            .to_string(),
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"
            .to_string(),
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"
            .to_string(),
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"
            .to_string(),
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"
            .to_string(),
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"
            .to_string(),
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"
            .to_string(),
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            .to_string(),
    ];

    assert_eq!(61229, sum_output(&input));
}

fn sum_output(input: &[String]) -> u32 {
    let mut sum = 0;
    for line in input {
        let whole_line = whole_line_space_sep(line);
        if let Some(wiring) = find_wiring(&whole_line) {
            let output = output_only(line);
            sum += get_value(&output, &wiring);
        } else {
            panic!();
        }
    }
    sum
}

fn whole_line_space_sep(line: &str) -> String {
    let segments = line.split(" | ").collect::<Vec<&str>>();
    segments.join(" ")
}

fn output_only(line: &str) -> String {
    line.split(" | ").last().unwrap().to_string()
}

// Map from to->from
type Wiring = HashMap<char, char>;

fn rewire(display: &str, wiring: &Wiring) -> String {
    display.chars().map(|c| wiring[&c]).sorted().collect()
}

fn find_wiring(output: &str) -> Option<Wiring> {
    let possible = possible_wirings();
    let best = possible
        .iter()
        .filter(|wiring| matches(wiring, output))
        .collect::<Vec<_>>();
    if best.len() == 1 {
        return Some(best[0].clone());
    } else {
        println!("{:?}", best);
    }
    None
}

fn possible_wirings() -> Vec<Wiring> {
    let mut result = Vec::new();
    for tos in "abcdefg".to_string().chars().permutations(7) {
        let mut wiring = HashMap::new();
        for (&to, from) in tos.iter().zip("abcdefg".to_string().chars()) {
            wiring.insert(to, from);
        }
        result.push(wiring);
    }
    result
}

fn matches(wiring: &Wiring, output: &str) -> bool {
    let values = output.split(' ');
    for value in values {
        // Compare with the known values:
        let hypothesis = rewire(value, wiring);
        if to_number(&hypothesis).is_none() {
            return false;
        }
    }
    true
}

fn get_value(line: &str, wiring: &Wiring) -> u32 {
    let digits: Vec<_> = line
        .split(' ')
        .map(|value| rewire(value, wiring))
        .map(|x| to_number(&x).unwrap())
        .collect();
    concatenate_digits(&digits)
}

fn to_number(value: &str) -> Option<u32> {
    let compare = |first: &str, second: &str| {
        if first.len() != second.len() {
            return false;
        }
        for (a, b) in first.chars().zip(second.chars()) {
            if a != b {
                return false;
            }
        }
        true
    };
    for (number, s) in (0..=9).zip([ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE]) {
        if compare(value, s) {
            return Some(number);
        }
    }
    None
}

fn concatenate_digits(digits: &[u32]) -> u32 {
    digits.iter().fold(0, |sum, n| sum * 10 + n)
}

fn main() {
    let input = input_lines("input/day_8.txt").unwrap();
    // part 1
    {
        let sum = count_unique(&outputs(&input));
        println!("#1: {}", sum);
    }

    // part 2
    {
        let sum = sum_output(&input);
        println!("#2: {}", sum);
    }
}
