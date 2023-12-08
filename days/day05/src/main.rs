use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn main() -> std::io::Result<()> {
    let f = File::open("data/5.1.txt")?;
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().flatten().collect::<Vec<_>>();
    // pt1: 135086721  "your answer is too low"
    // i was repeatedly using 'seed' in the chained lookup. proper chaining:
    // pt1: 130120695  "your answer is too low"
    println!("pt1: {}", pt1(&lines));
    println!("pt2: {}", pt2(&lines));
    Ok(())
}

pub fn pt1(lines: &Vec<String>) -> u64 {
    let almanac = parse_almanac(lines.iter().map(|s| s.as_str()));
    let locations = almanac.seeds.iter().map(|s| almanac.seed_to_location(s));
    locations.min().unwrap()
}

// the input seeds list is not a list of seeds, but a list of ranges
//
// if the ranges were materialized to a list of seeds, what's the lowest loc number now?
pub fn pt2(lines: &Vec<String>) -> u64 {
    let almanac = parse_almanac(lines.iter().map(|s| s.as_str()));
    // TODO: over 2b seeds. do I need to optimize?
    let lengths = almanac.seeds.iter().enumerate().filter(|(i, e)| i % 2 == 1);
    lengths.map(|(i, e)| e).sum()
}

fn parse_almanac<'a>(mut s: impl Iterator<Item = &'a str>) -> Almanac {
    // seeds: 79 14 55 13
    //
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48
    let seeds = s
        .next()
        .unwrap()
        .split("seeds: ")
        .nth(1)
        .map(|line| line.split(" ").map(|n| n.parse::<u64>().unwrap()).collect())
        .unwrap();

    let mut tmp = vec![];
    let mut maps = vec![];

    while let Some(line) = s.next() {
        if line == "" {
            if tmp.len() > 0 {
                maps.push(AlmanacMap::try_from(&tmp).unwrap());
                tmp = vec![];
            }
        } else {
            tmp.push(line);
        }
    }
    if tmp.len() > 2 {
        maps.push(AlmanacMap::try_from(&tmp).unwrap());
    }
    Almanac { seeds, maps }
}

struct Almanac {
    seeds: Vec<u64>,
    // assume order is the seed -> ... -> location path
    maps: Vec<AlmanacMap>,
}

impl Almanac {
    fn seed_to_location(&self, seed: &u64) -> u64 {
        let mut loc = seed.clone();
        //let mut path = vec![];
        for map in &self.maps {
            //let o = loc;
            loc = map.source_to_dest(&loc);
            /*
            path.push(format!(
                "{o} : {0} -> {1} : {loc}",
                map.source_name, map.destination_name
            ));
            */
        }
        /*
        if loc == 130120695 {
            dbg!(seed);
            dbg!(path);
        }
        */
        loc
    }
}

#[allow(dead_code)]
struct AlmanacMap {
    source_name: String,
    destination_name: String,
    mapping: Vec<MappingLine>,
}

#[derive(Debug)]
struct MappingLine {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl TryFrom<&Vec<&str>> for AlmanacMap {
    type Error = ();

    /// parse this
    //
    // ```
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48
    // ```
    fn try_from(value: &Vec<&str>) -> Result<Self, Self::Error> {
        let name = value[0].split(" map:").next().unwrap();
        let parts = name.split("-to-").collect::<Vec<_>>();
        let source_name = parts[0].to_string();
        let destination_name = parts[1].to_string();
        let mut mapping = vec![];
        for line in &value[1..] {
            let nums: Vec<_> = line.split(" ").map(|n| n.parse().unwrap()).collect();
            let destination_start = nums[0];
            let source_start = nums[1];
            let length = nums[2];
            mapping.push(MappingLine {
                destination_start,
                source_start,
                length,
            });
        }
        Ok(AlmanacMap {
            source_name,
            destination_name,
            mapping,
        })
    }
}

impl AlmanacMap {
    /// following the rules of the problem
    ///
    /// if the input falls within a range covered by the map, return the translated location
    /// otherwise return the same input
    fn source_to_dest(&self, n: &u64) -> u64 {
        let matching_lines = self
            .mapping
            .iter()
            .filter(|line| &line.source_start <= n && n <= &(line.source_start + line.length))
            .collect::<Vec<_>>();
        // this isn't mentioned as a possibility, but i want to crash in case this results in a bug
        // later
        if matching_lines.len() == 0 {
            return *n;
        }
        assert!(matching_lines.len() == 1, "{n} is in {matching_lines:?}");

        let mapping = matching_lines[0];

        let delta = n - mapping.source_start;
        mapping.destination_start + delta
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
        let m = &almanac.maps[0];
        assert_eq!(m.source_name, "seed");
        assert_eq!(m.destination_name, "soil");
        assert_eq!(m.source_to_dest(&79), 81);
    }
}
