use itertools::Itertools;

struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn possibilities(&self) -> i64 {
        let mut count: i64 = 0;
        for speed in 0..self.time {
            if (self.time - speed) * speed > self.distance {
                count += 1;
            }
        }
        count
    }
}

fn main() {
    let input_part1 = {
        let input = include_str!("input.txt").lines().collect_vec();
        let time = input
            .iter()
            .find(|it| it.starts_with("Time:"))
            .map(|it| it.replace("Time:", ""))
            .map(|it| {
                it.split_whitespace()
                    .map(|it| it.parse::<i64>().unwrap())
                    .collect_vec()
            })
            .unwrap();
        let distance = input
            .iter()
            .find(|it| it.starts_with("Distance:"))
            .map(|it| it.replace("Distance:", ""))
            .map(|it| {
                it.split_whitespace()
                    .map(|it| it.parse::<i64>().unwrap())
                    .collect_vec()
            })
            .unwrap();
        time.iter()
            .zip(distance.iter())
            .map(|(t, d)| Race {
                time: *t,
                distance: *d,
            })
            .collect_vec()
    };

    let input_part2 = {
        let time = input_part1
            .iter()
            .map(|it| it.time.to_string())
            .join("")
            .parse::<i64>()
            .unwrap();
        let distance = input_part1
            .iter()
            .map(|it| it.distance.to_string())
            .join("")
            .parse::<i64>()
            .unwrap();
        Race { time, distance }
    };

    let part1 = input_part1
        .iter()
        .map(|it| it.possibilities())
        .product::<i64>();
    let part2 = input_part2.possibilities();

    println!("\nDay6");
    println!("---------");
    println!("Part 1: {}", part1);
    println!("Part 2: {}\n", part2);
}
