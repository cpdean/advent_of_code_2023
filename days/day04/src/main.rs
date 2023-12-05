use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn main() -> std::io::Result<()> {
    let f = File::open("data/4.1.txt")?;
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().flatten().collect::<Vec<_>>();
    println!("pt1: {}", pt1(&lines));
    println!("pt2: {}", pt2(&lines));
    Ok(())
}

pub fn pt1(lines: &Vec<String>) -> i32 {
    let scores = lines
        .iter()
        .map(|e| e.parse::<Card>().unwrap())
        .map(|c| c.score());
    scores.sum()
}

pub fn pt2(lines: &Vec<String>) -> i32 {
    let cards = lines
        .iter()
        .map(|e| e.parse::<Card>().unwrap())
        .collect::<Vec<Card>>();
    let mut pool = Vec::new();
    pool.resize(cards.len(), 1);
    for i in 0..cards.len() {
        let matches = cards[i].matches();
        for m in 1..=matches {
            let count = pool[i];
            if i + m < pool.len() {
                pool[i + m] += count;
            }
        }
    }
    pool.iter().sum()
}

#[derive(Debug, PartialEq)]
struct Card {
    id: i32,
    numbers: Vec<i32>,
    winning_numbers: Vec<i32>,
}

impl Card {
    pub fn score(&self) -> i32 {
        let mut acc = 0;
        for n in &self.numbers {
            if self.winning_numbers.contains(&n) {
                acc = if acc == 0 { 1 } else { acc * 2 };
            }
        }
        acc
    }

    pub fn matches(&self) -> usize {
        self.numbers
            .iter()
            .filter(|e| self.winning_numbers.contains(&e))
            .count()
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let header = s.split(":").next().unwrap();
        let id = header
            .split(" ")
            .filter(|e| e.len() > 0)
            .nth(1)
            .and_then(|e| e.parse().ok())
            .unwrap();
        let numbers_part = s.split(": ").nth(1);
        // " 1 21 53 59 44 | 69 82 63 72 16 21 14  1"
        let my_raw = numbers_part.as_ref().and_then(|r| r.split(" | ").nth(0));
        let winning_raw = numbers_part.as_ref().and_then(|r| r.split(" | ").nth(1));

        let numbers = my_raw.map(parse_nums).unwrap();
        let winning_numbers = winning_raw.map(parse_nums).unwrap();

        Ok(Card {
            id,
            numbers,
            winning_numbers,
        })
    }
}

fn parse_nums(line: &str) -> Vec<i32> {
    let mut start = 0;
    let mut end = 0;
    let mut in_number = false;
    let line = line.chars().collect::<Vec<_>>();
    let mut acc = vec![];
    while end < line.len() {
        if line[end].is_digit(10) {
            if !in_number {
                start = end;
            }
            in_number = true;
        } else {
            if in_number {
                let num = line[start..end]
                    .iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                acc.push(num);
                start = end;
                in_number = false;
            }
        }
        end += 1;
    }
    if in_number {
        let num = line[start..end]
            .iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        acc.push(num);
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1() {
        let input: Vec<_> = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let c: Card = input[0].parse().unwrap();

        assert_eq!(
            Card {
                id: 1,
                numbers: vec![41, 48, 83, 86, 17],
                winning_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            },
            c,
        );
        /*
            In the above example, card 1 has five winning numbers (41, 48, 83, 86, and 17) and
            eight numbers you have (83, 86, 6, 31, 17, 9, 48, and 53). Of the numbers you have,
            four of them (48, 83, 17, and 86) are winning numbers! That means card 1 is worth 8
            points (1 for the first match, then doubled three times for each of the three matches
            after the first).

            Card 2 has two winning numbers (32 and 61), so it is worth 2 points.
            Card 3 has two winning numbers (1 and 21), so it is worth 2 points.
            Card 4 has one winning number (84), so it is worth 1 point.
            Card 5 has no winning numbers, so it is worth no points.
            Card 6 has no winning numbers, so it is worth no points.
        */

        let scores = input
            .iter()
            .map(|e| e.parse::<Card>().unwrap())
            .map(|c| c.score())
            .collect::<Vec<_>>();
        assert_eq!(scores, vec![8, 2, 2, 1, 0, 0]);
    }

    #[test]
    fn test_ex2() {
        let input: Vec<_> = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let answer = pt2(&input);
        assert_eq!(30, answer);
    }

    #[test]
    fn test_parse_num_col() {
        let input = " 1 21 53 59 44";
        assert_eq!(parse_nums(input), vec![1, 21, 53, 59, 44]);
    }
}
