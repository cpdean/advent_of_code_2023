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
    //println!("pt2: {}", pt2(&lines));
    Ok(())
}

pub fn pt1(lines: &Vec<String>) -> i32 {
    let scores = lines.iter();
    0
}

pub fn parse_almanac<'a>(s: impl Iterator<Item = &'a str>) -> () {}

struct AlmanacMap {
    source_name: String,
    destination_name: String,
    mapping: Vec<MappingLine>,
}

struct MappingLine {
    destination_start: i32,
    source_start: i32,
    length: i32,
}

impl FromStr for AlmanacMap {
    type Err = ();

    /// parse this
    //
    // ```
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48
    // ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_ex1() {
        let almanac = parse_almanac(EXAMPLE.split("\n"));
    }
}
