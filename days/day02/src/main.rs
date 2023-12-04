use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn main() -> std::io::Result<()> {
    let f = File::open("data/2.1.txt")?;
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().flatten().collect::<Vec<_>>();
    println!("pt1: {}", pt1(&lines));
    //println!("pt2: {}", pt2(&lines));
    Ok(())
}

// The Elf would first like to know which games would have been possible if the bag contained only
// 12 red cubes, 13 green cubes, and 14 blue cubes?

type RedCount = i32;
type BlueCount = i32;
type GreenCount = i32;

#[derive(Debug, PartialEq)]
struct Game {
    id: i32,
    rounds: Vec<(RedCount, GreenCount, BlueCount)>,
}

impl Game {
    pub fn max(&self) -> (RedCount, GreenCount, BlueCount) {
        let mut out = (0, 0, 0);
        for (r, g, b) in &self.rounds {
            let (ar, ag, ab) = out;
            out = (*r.max(&ar), *g.max(&ag), *b.max(&ab));
        }
        out
    }

    pub fn can_be_playable_with(&self, (r, g, b): (i32, i32, i32)) -> bool {
        let (mr, mg, mb) = self.max();
        r >= mr && g >= mg && b >= mb
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let game = s.split(":").next().unwrap();
        let id: i32 = game.split(" ").nth(1).map(|n| n.parse().unwrap()).unwrap();
        let rounds: Vec<(i32, i32, i32)> = s
            .split(":")
            .nth(1)
            .map(|rs| {
                let g: Vec<(i32, i32, i32)> = rs
                    .split(";")
                    .map(|round| {
                        round
                            .split(",")
                            .map(|color| {
                                let mut cs = color.strip_prefix(" ").unwrap().split(" ");
                                let count = cs.next().unwrap().parse().unwrap();
                                let name = cs.next().unwrap();
                                let color_tuple: (i32, i32, i32) = match name {
                                    "red" => (count, 0, 0),
                                    "green" => (0, count, 0),
                                    "blue" => (0, 0, count),
                                    what => panic!("don't know this color: {}", what),
                                };
                                color_tuple
                            })
                            .reduce(|(red, green, blue), (acc_r, acc_g, acc_b)| {
                                (red + acc_r, green + acc_g, blue + acc_b)
                            })
                            .unwrap()
                    })
                    .collect();
                g
            })
            .unwrap();
        Ok(Game { id, rounds })
    }
}

// 12 red cubes, 13 green cubes, and 14 blue cubes
pub fn pt1(lines: &Vec<String>) -> i32 {
    let games = lines.iter().map(|g| g.parse::<Game>().unwrap());
    let can_fit: Vec<_> = games
        .filter(|g| g.can_be_playable_with((12, 13, 14)))
        .collect();
    can_fit.iter().map(|g| g.id).reduce(|a, b| a + b).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_parse() {
        let raw = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let p: Game = raw.parse().unwrap();
        assert_eq!(
            Game {
                id: 3,
                rounds: vec![(20, 8, 6), (4, 13, 5), (1, 5, 0)],
            },
            p
        )
    }

    #[test]
    fn test_pt1_example() {
        let lines: Vec<String> = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let answer = pt1(&lines);
        assert_eq!(8, answer);
    }
}
