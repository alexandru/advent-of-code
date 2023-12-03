/*
 * <https://adventofcode.com/2023/day/3>
 */

use std::collections::{HashMap, HashSet};

fn is_number(ch: char) -> bool {
    ch >= '0' && ch <= '9'
}

fn is_special_char(ch: char) -> bool {
    (ch < '0' || ch > '9') && ch != '.'
}

fn adjacent_coordonates(i: usize, j: usize) -> Vec<(usize, usize)> {
    let (i, j) = (i as i32, j as i32);
    vec![
        (i - 1, j - 1),
        (i - 1, j),
        (i - 1, j + 1),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
    ]
    .iter()
    .filter_map(|(i, j)| {
        if *i < 0 || *j < 0 {
            None
        } else {
            Some((*i as usize, *j as usize))
        }
    })
    .collect()
}

fn main() {
    let matrix = include_str!("input.txt")
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut all_numbers: Vec<i32> = vec![];
    let mut all_gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for (i, line) in matrix.iter().enumerate() {
        let mut is_in_number = false;
        let mut is_adjacent_special = false;
        let mut detected_gears: HashSet<(usize, usize)> = HashSet::new();
        let mut partial_number = 0;

        for j in 0..=line.len() {
            match line.get(j) {
                Some(ch) if is_number(*ch) => {
                    is_in_number = true;
                    partial_number = partial_number * 10 + (ch.to_digit(10).unwrap() as i32);

                    for (x, y) in adjacent_coordonates(i, j) {
                        let Some(ch) = matrix.get(x).and_then(|l| l.get(y)) else {
                            continue;
                        };
                        if is_special_char(*ch) {
                            is_adjacent_special = true;
                        }
                        if *ch == '*' {
                            detected_gears.insert((x, y));
                        }
                    }
                }
                _ if is_in_number => {
                    // Updating global state
                    if is_adjacent_special {
                        all_numbers.push(partial_number);
                    }
                    for gear in detected_gears.iter() {
                        match all_gears.get_mut(gear) {
                            Some(list) => {
                                list.push(partial_number);
                            }
                            None => {
                                all_gears.insert(*gear, vec![partial_number]);
                            }
                        }
                    }
                    // Reset local state, preparing for next number
                    is_in_number = false;
                    is_adjacent_special = false;
                    detected_gears.clear();
                    partial_number = 0;
                }
                _ => {}
            }
        }
    }

    let part1 = all_numbers.iter().sum::<i32>();
    let part2 = all_gears
        .values()
        .filter(|n| n.len() > 1)
        .map(|v| v.iter().product::<i32>())
        .sum::<i32>();

    println!("\nDay 3\n------------");
    println!("Part 1: {}", part1);
    println!("Part 2: {}\n", part2);
}
