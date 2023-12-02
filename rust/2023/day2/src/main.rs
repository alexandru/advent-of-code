/*
 * <https://adventofcode.com/2023/day/2>
 */

use regex::Regex;

struct GameSet {
    red: i32,
    green: i32,
    blue: i32,
}

impl GameSet {
    fn subset_of(&self, other: &GameSet) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

struct Game {
    id: i32,
    sets: Vec<GameSet>,
}

impl Game {
    fn minimum_bag(&self) -> GameSet {
        GameSet {
            red: self.sets.iter().map(|s| s.red).max().unwrap_or(0),
            green: self.sets.iter().map(|s| s.green).max().unwrap_or(0),
            blue: self.sets.iter().map(|s| s.blue).max().unwrap_or(0),
        }
    }
}

fn main() {
    let initial_bag = GameSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    let game_re = Regex::new(r"^Game\s+(?<id>\d+)[:](?<sets>[^$]+)$").unwrap();
    let count_re = Regex::new(r"^\s*(?<count>\d+)\s+(?<color>red|green|blue)\s*$").unwrap();

    let lines: Vec<&str> = include_str!("input.txt").lines().collect();
    let mut games = Vec::new();

    for game_raw in lines {
        let Some(m) = game_re.captures(game_raw) else {
            continue;
        };
        let id = m["id"].parse::<i32>().unwrap();
        let mut sets = Vec::new();

        for set_raw in m["sets"].split(";") {
            let mut set = GameSet {
                red: 0,
                green: 0,
                blue: 0,
            };
            for color_raw in set_raw.split(",") {
                let Some(m) = count_re.captures(color_raw) else {
                    continue;
                };
                let count = m["count"].parse::<i32>().unwrap();
                match &m["color"] {
                    "red" => set.red = count,
                    "green" => set.green = count,
                    "blue" => set.blue = count,
                    _ => continue,
                }
            }
            sets.push(set);
        }
        games.push(Game { id, sets });
    }

    let part1 = games.iter()
        .filter(|g| g.sets.iter().all(|s| s.subset_of(&initial_bag)))
        .map(|g| g.id)
        .sum::<i32>();

    let part2 = games.iter()
        .map(|g| g.minimum_bag().power())
        .sum::<i32>();

    println!("\nDay 2\n------------");
    println!("Part 1: {}", part1);
    println!("Part 2: {}\n", part2);
}
