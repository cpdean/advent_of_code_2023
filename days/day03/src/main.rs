use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn main() -> std::io::Result<()> {
    let f = File::open("data/3.1.txt")?;
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().flatten().collect::<Vec<_>>();
    println!("pt1: {}", pt1(&lines));
    println!("pt2: {}", pt2(&lines));
    Ok(())
}

pub fn pt1(lines: &Vec<String>) -> i32 {
    let p = parsed_schematic(lines);
    let n = numbers_touching_a_symbol(p);
    n.iter().sum()
}

pub fn pt2(lines: &Vec<String>) -> i32 {
    let p = parsed_schematic(lines);
    let n = touching_gear(p);

    n.iter().map(|(a, b)| a * b).sum()
}

#[derive(Debug, PartialEq)]
enum Part {
    Num(String),
    Symbol(String),
}

fn parsed_schematic(lines: &Vec<String>) -> Vec<((i32, i32), Part)> {
    let mut out: Vec<((i32, i32), Part)> = vec![];
    for (y, line) in lines.iter().enumerate() {
        let line = line.chars().collect::<Vec<_>>();
        let mut c = 0;
        let mut in_number = false;
        for end in 0..line.len() {
            let current = line[end];
            if current.is_digit(10) {
                if in_number {
                    continue;
                } else {
                    in_number = true;
                    // trim the leading tile only when entering the number, otherwise leave cursor
                    // untouched
                    c = end;
                }
            } else {
                if in_number {
                    in_number = false;
                    out.push((
                        (c as i32, y as i32),
                        Part::Num(line[c..end].iter().collect()),
                    ));
                }
                if current == '.' {
                    // ...
                } else {
                    out.push(((end as i32, y as i32), Part::Symbol(current.to_string())));
                }
                c = end;
            }
        }
        // if you finish the line still in a number
        if in_number {
            out.push((
                (c as i32, y as i32),
                Part::Num(line[c..line.len()].iter().collect()),
            ));
        }
    }
    out
}

fn has_collision(
    to_check: &(&(i32, i32), &String),
    lookup: &HashSet<(i32, i32)>,
) -> Option<(i32, i32)> {
    let ((x, y), num) = to_check;
    // iterate over the bounding box around the number, checking our lookup
    // push and break when you find one
    let upper_left = (x - 1, y - 1);
    let bottom_right = (x + num.len() as i32, y + 1);

    for iy in upper_left.1..=bottom_right.1 {
        for ix in upper_left.0..=bottom_right.0 {
            if lookup.contains(&(ix, iy)) {
                return Some((ix, iy));
            }
        }
    }
    None
}

fn numbers_touching_a_symbol(parsed: Vec<((i32, i32), Part)>) -> Vec<i32> {
    let mut numbers = vec![];
    let lookup: HashSet<(i32, i32)> = parsed
        .iter()
        .filter(|(_coord, p)| match p {
            Part::Num(_) => false,
            _ => true,
        })
        .map(|((x, y), _p)| (*x as i32, *y as i32))
        .collect();
    let numbers_to_check = parsed.iter().flat_map(|(coord, p)| match p {
        Part::Num(s) => Some((coord, s)),
        _ => None,
    });
    for to_check in numbers_to_check {
        if let Some(_coord) = has_collision(&to_check, &lookup) {
            let (_, num) = to_check;
            numbers.push(num.parse::<i32>().unwrap());
        }
    }
    numbers
}

fn touching_gear(parsed: Vec<((i32, i32), Part)>) -> Vec<(i32, i32)> {
    let mut gear_pairs = HashMap::new();
    let gears: HashSet<(i32, i32)> = parsed
        .iter()
        .flat_map(|(coord, p)| match p {
            Part::Symbol(x) if x == &"*".to_string() => Some(coord.clone()),
            Part::Symbol(_) => None,
            Part::Num(_) => None,
        })
        .collect();

    let numbers_to_check = parsed.iter().flat_map(|(coord, p)| match p {
        Part::Num(s) => Some((coord, s)),
        _ => None,
    });
    for to_check in numbers_to_check {
        if let Some(coord) = has_collision(&to_check, &gears) {
            let (_, num) = to_check;
            let parsed_number = num.parse::<i32>().unwrap();
            if !gear_pairs.contains_key(&coord) {
                gear_pairs.insert(coord.clone(), vec![parsed_number]);
            } else {
                let pairs = gear_pairs.get_mut(&coord).unwrap();
                pairs.push(parsed_number);
            }
        }
    }
    gear_pairs
        .iter()
        .filter(|(_, pairs)| pairs.len() == 2)
        .map(|(_, pairs)| (pairs[0], pairs[1]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1_example() {
        let input: Vec<String> = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .iter()
        .map(|e| e.to_string())
        .collect();

        let p = parsed_schematic(&input);
        //assert_eq!(vec![((0, 0), Part::Num("1".to_string()))], p);
        let numbers = numbers_touching_a_symbol(p);
        assert_eq!(vec![467, 35, 633, 617, 592, 755, 664, 598], numbers);

        assert_eq!(pt1(&input), 4361);
    }

    #[test]
    fn test_pt1_more_bounds() {
        let input: Vec<String> = vec!["...*......", "....88....", "...89..777", "......-..."]
            .iter()
            .map(|e| e.to_string())
            .collect();

        let p = parsed_schematic(&input);
        //assert_eq!(vec![((0, 0), Part::Num("1".to_string()))], p);
        let numbers = numbers_touching_a_symbol(p);
        assert_eq!(vec![88, 777], numbers);
    }
}
