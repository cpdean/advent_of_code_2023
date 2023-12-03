use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn extract_digits(s: &String) -> i32 {
    let digits: Vec<i32> = s
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_string().parse().unwrap())
        .collect::<Vec<_>>();
    let first = digits[0];
    let last = digits[digits.len() - 1];
    (first * 10) + last
}

pub fn main() -> std::io::Result<()> {
    let f = File::open("data/1.1.txt")?;
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().flatten().collect::<Vec<_>>();
    println!("pt1: {}", pt1(&lines));
    println!("pt2: {}", pt2(&lines));
    Ok(())
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
fn number_word_positions(s: &String) -> Vec<(usize, usize)> {
    let mut o = vec![];
    for (i, word) in NUMBERS.iter().enumerate() {
        // slide
        if s.len() < word.len() {
            break;
        }
        for position in 0..1 + s.len() - word.len() {
            if position + word.len() > s.len() {
                break;
            }
            let substr = &s[position..(position + word.len())];
            if substr == *word {
                o.push((position, i + 1));
            }
        }
    }
    o
}

fn number_digit_positions(s: &String) -> Vec<(usize, usize)> {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            if c.is_digit(10) {
                Some((i, c.to_string().parse().unwrap()))
            } else {
                None
            }
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn part2_extract(s: &String) -> i32 {
    let by_digit = number_digit_positions(s);
    let by_word = number_word_positions(s);
    let mut positions = by_digit.iter().chain(by_word.iter()).collect::<Vec<_>>();
    positions.sort();
    let first = positions[0].1 as i32;
    let last = positions[positions.len() - 1].1 as i32;
    (first * 10) + last
}

pub fn pt1(lines: &Vec<String>) -> i32 {
    let mut total = 0;
    for line in lines {
        total += extract_digits(&line);
    }
    total
}

pub fn pt2(lines: &Vec<String>) -> i32 {
    let mut total = 0;
    for line in lines {
        total += part2_extract(&line);
    }
    total
}

#[cfg(test)]
mod tests {

    use super::*;

    fn part1_example<'a>() -> Vec<(&'a str, i32)> {
        vec![
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ]
    }

    fn part2_example<'a>() -> Vec<(&'a str, i32)> {
        vec![
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
        ]
    }

    #[test]
    fn test_basic_extract() {
        let example = part1_example();
        for (test, expected) in example {
            let result = extract_digits(&test.to_string());
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn basic_pt2_extract() {
        let example = part2_example();
        for (test, expected) in example {
            let result = part2_extract(&test.to_string());
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_word_extract() {
        assert_eq!(
            number_word_positions(&"two1nine".to_string()),
            vec![(0, 2), (4, 9)]
        )
    }

    #[test]
    fn test_pos_digit_extract() {
        assert_eq!(
            number_digit_positions(&"two1nine".to_string()),
            vec![(3, 1)]
        )
    }
}
