use std::{cmp, collections::BTreeSet};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Range {
    start_inclusive: i64,
    end_exclusive: i64,
}

impl Range {
    fn validated(s: i64, e: i64) -> Option<Range> {
        if s < e {
            Some(Range {
                start_inclusive: s,
                end_exclusive: e,
            })
        } else {
            None
        }
    }

    fn cut_and_translate(&self, mask: &Range, delta: i64) -> (Option<Range>, BTreeSet<Range>) {
        let s = cmp::max(self.start_inclusive, mask.start_inclusive);
        let e = cmp::min(mask.end_exclusive, self.end_exclusive);
        let mut set: BTreeSet<Range> = BTreeSet::new();
        match Range::validated(s, e) {
            Some(i) => {
                let newr = Range {
                    start_inclusive: i.start_inclusive + delta,
                    end_exclusive: i.end_exclusive + delta,
                };
                if let Some(r) = Range::validated(mask.start_inclusive, self.start_inclusive) {
                    set.insert(r);
                }
                if let Some(r) = Range::validated(i.end_exclusive, mask.end_exclusive) {
                    set.insert(r);
                }
                (Some(newr), set)
            }
            None => {
                set.insert(self.clone());
                (None, set)
            }
        }
    }
}

#[derive(Debug)]
struct RangeMapping {
    range: Range,
    map_to_start_inclusive: i64,
}

impl RangeMapping {
    fn delta(&self) -> i64 {
        self.map_to_start_inclusive - self.range.start_inclusive
    }

    fn cut_and_translate(&self, mask: &Range) -> (Option<Range>, BTreeSet<Range>) {
        self.range.cut_and_translate(mask, self.delta())
    }
}

struct Layer {
    ranges: Vec<RangeMapping>,
}

impl Layer {
    fn translate(&self, range: &Range) -> BTreeSet<Range> {
        let mut remaining_source = BTreeSet::new();
        remaining_source.insert(range.clone());
        let mut translated: BTreeSet<Range> = BTreeSet::new();

        while let Some(current_source) = remaining_source.first().cloned() {
            let found = self
                .ranges
                .iter()
                .map(|r| r.cut_and_translate(range))
                .filter_map(|r| match r {
                    (Some(r), set) => Some((r, set)),
                    (None, _) => None,
                })
                .next();

            remaining_source.remove(&current_source);
            match found {
                None => {
                    translated.insert(current_source);
                }
                Some((tr, mut rest)) => {
                    translated.insert(tr);
                    remaining_source.append(&mut rest);
                }
            }
        }

        translated
    }
}

fn calculate(layers: &Vec<Layer>, seeds: &Vec<Range>) -> Option<i64> {
    let mut state = BTreeSet::new();
    for s in seeds {
        state.insert(s.clone());
    }
    for layer in layers {
        let vec = state.iter().map(|r| layer.translate(r)).collect_vec();
        state.clear();
        for mut set in vec {
            state.append(&mut set);
        }
    }
    state.iter().map(|i| i.start_inclusive).min()
}

fn main() {
    let input = include_str!("input.txt").lines().collect::<Vec<&str>>();

    let (seeds_part1, seeds_part2) = {
        let seeds_raw = input
            .first()
            .unwrap()
            .replace("seeds: ", "")
            .split_whitespace()
            .map(|nr| nr.parse::<i64>().unwrap())
            .collect_vec();
        let p1 = seeds_raw
            .iter()
            .map(|s| Range {
                start_inclusive: *s,
                end_exclusive: s + 1,
            })
            .collect_vec();
        let p2 = seeds_raw
            .chunks(2)
            .map(|c| Range {
                start_inclusive: c[0],
                end_exclusive: c[1],
            })
            .collect_vec();
        (p1, p2)
    };

    let layers = input
        .iter()
        .skip(1)
        .map(|layer_str| {
            let lines = layer_str.lines().collect_vec();
            let ranges = lines
                .iter()
                .skip(1)
                .map(|range_str| {
                    let range_vec = range_str
                        .split_whitespace()
                        .map(|nr| nr.parse::<i64>().unwrap())
                        .collect_vec();
                    if let &[dest_str, src_str, len_str] = &range_vec[..3] {
                        RangeMapping {
                            range: Range {
                                start_inclusive: src_str,
                                end_exclusive: src_str + len_str,
                            },
                            map_to_start_inclusive: dest_str,
                        }
                    } else {
                        panic!("Invalid line: {:?}", range_str)
                    }
                })
                .collect_vec();
            Layer { ranges }
        })
        .collect_vec();

    println!("\nDay5");
    println!("---------");
    println!("Part 1: {}", calculate(&layers, &seeds_part1).unwrap());
    println!("Part 2: {}\n", calculate(&layers, &seeds_part2).unwrap());
}

#[cfg(test)]
mod tests {
    use super::Range;

    #[test]
    fn ranges() {
        let r1 = Range {
            start_inclusive: 10,
            end_exclusive: 20,
        };
        let r2 = Range {
            start_inclusive: 5,
            end_exclusive: 30,
        };

        let (newr, rest) = r1.cut_and_translate(&r2, 2);
        assert_eq!(
            newr,
            Some(Range {
                start_inclusive: 12,
                end_exclusive: 22
            })
        );

        let rest = rest.iter().collect::<Vec<&Range>>();
        assert_eq!(
            rest,
            vec![
                &Range {
                    start_inclusive: 5,
                    end_exclusive: 10
                },
                &Range {
                    start_inclusive: 20,
                    end_exclusive: 30
                }
            ]
        );
    }

    #[test]
    fn ranges_cmp() {
        let r1 = Range {
            start_inclusive: 10,
            end_exclusive: 20,
        };
        let r2 = Range {
            start_inclusive: 5,
            end_exclusive: 30,
        };
        let r3 = Range {
            start_inclusive: 10,
            end_exclusive: 30,
        };
        let r4 = Range {
            start_inclusive: 10,
            end_exclusive: 30,
        };

        assert!(r1 < r3);
        assert!(r2 < r1);
        assert!(r2 < r3);
        assert!(r3 <= r4);
        assert!(r3 == r4);
    }
}
