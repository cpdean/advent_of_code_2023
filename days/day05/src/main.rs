use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn main() -> std::io::Result<()> {
    let f = File::open("data/5.1.txt")?;
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().flatten().collect::<Vec<_>>();
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
    //pt2_naiive(&almanac)
    pt2_reverse_strategy(&almanac)
}

/// this strategy will cut down the number of lookups by doing it backwards.
/// the problem only wants the very lowest location id, so we can just check
/// possible location ids in order and stop early instead of going through the
/// whole set of lookups and see which one was smallest.
///
/// this approach should also account for the issue of a seed mapping to multiple outputs, which
/// broke the naiive strategy
/// pt2: 44642725  "thats too high"
fn pt2_reverse_strategy(almanac: &Almanac) -> u64 {
    let location_map = &almanac.maps[&almanac.maps.len() - 1];
    // i am an idiot. of course you can still have a location outside of the
    // location mapping that will hit a seed.
    //let locations = OrderedLocations::new(&location_map.mapping);
    for loc in 0..44_642_725 {
        if almanac.pt2_contains_location(&loc) {
            return loc;
        }
    }
    0
}

/// naiive implementation, just do the work of implementing the seed explosion, then redo
/// the problem as normal.
//  the following showed that there are over 2b seeds for pt2. maybe that's not
//  big enough to worry about optimization
//
//      let almanac = parse_almanac(lines.iter().map(|s| s.as_str()));
//      let lengths = almanac
//          .seeds
//          .iter()
//          .enumerate()
//          .filter(|(i, _e)| i % 2 == 1);
//      lengths.map(|(_i, e)| e).sum()
//
//  2_037_733_040
/// this was going to take 3 hr on debug, few minutes on release build
/// broke on my assert where a seed must map to only one mappingline
#[allow(dead_code)]
fn pt2_naiive(almanac: &Almanac) -> u64 {
    let seed_list = SeedList::new(&almanac.seeds);
    let locations = seed_list.enumerate().map(|(i, s)| {
        // print about every 0.01% increment to track progress
        if i % 200_000 == 0 {
            let percent = i as f64 / 2_037_733_040.0;
            println!("progress: {:.4}%", percent * 100.0);
        }
        almanac.seed_to_location(&s)
    });
    locations.min().unwrap()
}

struct SeedList {
    seed_ranges: Vec<(u64, u64)>,
    // current range the iterator is sourcing from
    range_position: usize,
    // position in current range
    cursor_position: u64,
}

impl SeedList {
    fn new(seeds: &Vec<u64>) -> Self {
        let mut seed_ranges = vec![];
        for i in 0..(seeds.len() / 2) {
            seed_ranges.push((seeds[i], seeds[i + 1]));
        }
        Self {
            seed_ranges,
            range_position: 0,
            cursor_position: 0,
        }
    }

    fn contains_seed(&self, seed: &u64) -> bool {
        for (start, length) in &self.seed_ranges {
            if start <= seed && (seed <= &(start + length)) {
                return true;
            }
        }
        false
    }
}

impl Iterator for SeedList {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let (range_start, range_length) = self.seed_ranges[self.range_position];
        if self.cursor_position <= range_length {
            let ret = range_start + self.cursor_position;
            self.cursor_position += 1;
            Some(ret)
        } else {
            self.range_position += 1;
            if self.range_position >= self.seed_ranges.len() {
                None
            } else {
                let ret = self.seed_ranges[self.range_position].0;
                self.cursor_position = 1;
                Some(ret)
            }
        }
    }
}

struct OrderedLocations {
    source_location_map: Vec<MappingLine>,
    current_section: usize,
    current_seed: u64,
}

impl OrderedLocations {
    fn new(source_location_map: &Vec<MappingLine>) -> Self {
        let mut source_location_map: Vec<_> = source_location_map.to_vec();
        source_location_map.sort_by(|a, b| {
            a.destination_start
                .partial_cmp(&b.destination_start)
                .unwrap()
        });
        let current_section = 0;
        // Iterator::next impl assumes current_seed was the previously returned value.
        // start this at start - 1
        let current_seed = source_location_map[current_section].destination_start - 1;
        Self {
            source_location_map,
            current_section,
            current_seed,
        }
    }
}

impl Iterator for OrderedLocations {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let MappingLine {
            destination_start,
            length,
            ..
        } = self.source_location_map[self.current_section];
        if self.current_seed >= destination_start + length {
            if self.current_section + 1 >= self.source_location_map.len() {
                return None;
            }
            self.current_section += 1;
            self.current_seed = self.source_location_map[self.current_section].destination_start;
        } else {
            self.current_seed += 1;
        }
        Some(self.current_seed)
    }
}

fn parse_almanac<'a>(mut s: impl Iterator<Item = &'a str>) -> Almanac {
    // seeds: 79 14 55 13
    //
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48
    let seeds: Vec<_> = s
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
    Almanac {
        seeds: seeds.clone(),
        maps,
        seed_list: SeedList::new(&seeds),
    }
}

struct Almanac {
    seeds: Vec<u64>,
    // assume order is the seed -> ... -> location path
    maps: Vec<AlmanacMap>,
    seed_list: SeedList,
}

impl Almanac {
    fn new(
        seeds: Vec<u64>,
        // assume order is the seed -> ... -> location path
        maps: Vec<AlmanacMap>,
    ) -> Self {
        let seed_list = SeedList::new(&seeds);
        Self {
            seeds,
            maps,
            seed_list,
        }
    }

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

    /// pt 1 would be a Vec::contains, but pt2 changed
    /// the rules such that a the seed list denotes ranges of seeds
    fn pt2_contains_seed(&self, seed: &u64) -> bool {
        self.seed_list.contains_seed(seed)
    }

    /// instead of following the instructions of seed -> soil -> ... -> location
    /// do the lookup in reverse and see if the almanac has that corresponding seed
    fn pt2_contains_location(&self, location: &u64) -> bool {
        let mut loc = vec![location.clone()];
        let mut maps = self.maps.clone();
        maps.reverse();
        //let mut path = vec![];
        for map in maps {
            let mut next_locations = vec![];
            for found_location in loc {
                let newly_found_locations = map.dest_to_source(&found_location);
                for n in newly_found_locations {
                    next_locations.push(n);
                }
            }
            loc = next_locations;
        }

        for potential_seeds in loc {
            if self.pt2_contains_seed(&potential_seeds) {
                return true;
            }
        }
        false
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct AlmanacMap {
    source_name: String,
    destination_name: String,
    mapping: Vec<MappingLine>,
}

#[derive(Clone, Debug)]
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

        if matching_lines.len() == 0 {
            return *n;
        }
        assert!(matching_lines.len() == 1, "{n} is in {matching_lines:?}");

        let mapping = matching_lines[0];

        let delta = n - mapping.source_start;
        mapping.destination_start + delta
    }

    fn dest_to_source(&self, n: &u64) -> Vec<u64> {
        // return a vec because multiple mappinglines can overlap, as per the example
        //let mut output = vec![];

        let matching_lines = self
            .mapping
            .iter()
            .filter(|line| {
                &line.destination_start <= n && n <= &(line.destination_start + line.length)
            })
            .collect::<Vec<_>>();

        if matching_lines.len() == 0 {
            return vec![*n];
        }
        let mut found_mappings = vec![];
        // assuming no overlaps actually just to see what happens
        //assert!(matching_lines.len() == 1, "{n} is in {matching_lines:?}");

        for mapping in matching_lines {
            let delta = n - mapping.destination_start;
            found_mappings.push(mapping.source_start + delta);
        }

        found_mappings
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

    #[test]
    fn pt2_contains() {
        let almanac = parse_almanac(EXAMPLE.split("\n"));
        assert!(almanac.pt2_contains_seed(&80));
        assert!(!almanac.pt2_contains_seed(&10));
    }

    #[test]
    fn pt2_ordered_locations() {
        let almanac = parse_almanac(EXAMPLE.split("\n"));
        let location_map = &almanac.maps[almanac.maps.len() - 1];
        let locations = OrderedLocations::new(&location_map.mapping);
        let sorted: Vec<_> = locations.take(200).collect();
        assert_eq!(sorted[0], 56);
        assert_eq!(sorted[sorted.len() - 1], 97, "{:?}", sorted);
        assert!(sorted.len() != 200, "{:?}", sorted);
    }

    #[test]
    fn pt2_check_if_has_locations() {
        let almanac = parse_almanac(EXAMPLE.split("\n"));
        let loc_map = &almanac.maps[almanac.maps.len() - 1];
        assert_eq!(almanac.pt2_contains_seed(&79), true);
        assert_eq!(loc_map.source_to_dest(&79), 83);
        assert_eq!(loc_map.dest_to_source(&83)[0], 79);
        assert!(almanac.pt2_contains_location(&83));
    }
}
