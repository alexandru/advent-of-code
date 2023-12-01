/*
 * <https://adventofcode.com/2023/day/1>
 */

use std::collections::HashMap;
use regex::Regex;

fn calculate(re: Regex) -> i32 {
    let digits = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let lines: Vec<&str> = include_str!("input.txt").lines().collect();
    let mut sum = 0;

    for mut line in lines {
        //line = &line[1..];
        let mut first: Option<i32> = Option::None;
        let mut last = first;
        loop {
            match re.find(line) {
                Some(m) => {
                    let ds = m.as_str();
                    let di = digits
                        .get(ds)
                        .map(|x| x.clone())
                        .or_else(|| ds.parse::<i32>().ok());
                    first = first.or(di);
                    last = di.or(last);
                    line = &line[m.start()+1..];
                }
                None => {
                    break;
                }
            }
        }
        sum += first.unwrap() * 10 + last.unwrap();
    }
    sum
}

fn main() {
    let re = Regex::new(r"\d").unwrap();
    println!("\nPart 1: {}", calculate(re));

    let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    println!("Part 2: {}\n", calculate(re));
}
