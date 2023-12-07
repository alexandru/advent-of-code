use itertools::Itertools;
use regex::Regex;
use std::{cmp, collections::BTreeSet, fmt::Debug};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Range {
    start_in: i64,
    end_ex: i64,
}

impl Range {
    fn validated(s: i64, e: i64) -> Option<Range> {
        if s < e {
            Some(Range {
                start_in: s,
                end_ex: e,
            })
        } else {
            None
        }
    }

    fn cut_and_translate(&self, mask: &Range, delta: i64) -> (Option<Range>, BTreeSet<Range>) {
        let s = cmp::max(self.start_in, mask.start_in);
        let e = cmp::min(mask.end_ex, self.end_ex);
        let mut set: BTreeSet<Range> = BTreeSet::new();
        match Range::validated(s, e) {
            Some(i) => {
                let newr = Range {
                    start_in: i.start_in + delta,
                    end_ex: i.end_ex + delta,
                };
                if let Some(r) = Range::validated(mask.start_in, self.start_in) {
                    set.insert(r);
                }
                if let Some(r) = Range::validated(i.end_ex, mask.end_ex) {
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
    map_start: i64,
}

impl RangeMapping {
    fn delta(&self) -> i64 {
        self.map_start - self.range.start_in
    }

    fn cut_and_translate(&self, mask: &Range) -> (Option<Range>, BTreeSet<Range>) {
        self.range.cut_and_translate(mask, self.delta())
    }
}

#[derive(Debug)]
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
                // .map(|r| r.cut_and_translate(range))
                .filter_map(|r| match r.cut_and_translate(&current_source) {
                    (Some(r2), set) => Some((r2, set)),
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
    state.iter().map(|i| i.start_in).min()
}

fn main() {
    let paragraphs_re = Regex::new(r"\r?\n\s*\r?\n").unwrap();
    let paragraphs = paragraphs_re.split(include_str!("input.txt")).collect_vec();

    let (seeds_part1, seeds_part2) = {
        let seeds_raw = paragraphs
            .first()
            .unwrap()
            .replace("seeds: ", "")
            .split_whitespace()
            .map(|nr| nr.parse::<i64>().unwrap())
            .collect_vec();
        let p1 = seeds_raw
            .iter()
            .map(|s| Range {
                start_in: *s,
                end_ex: s + 1,
            })
            .collect_vec();
        let p2 = seeds_raw
            .chunks(2)
            .map(|c| Range {
                start_in: c[0],
                end_ex: c[0] + c[1],
            })
            .collect_vec();
        (p1, p2)
    };

    let layers = paragraphs
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
                                start_in: src_str,
                                end_ex: src_str + len_str,
                            },
                            map_start: dest_str,
                        }
                    } else {
                        panic!("Invalid line: {:?}", range_str)
                    }
                })
                .sorted_by_key(|it| it.range.clone())
                .collect_vec();
            Layer { ranges }
        })
        .collect_vec();

    println!("\nDay5");
    println!("---------");
    println!("Part 1: {}", calculate(&layers, &seeds_part1).unwrap());
    println!("Part 2: {}\n", calculate(&layers, &seeds_part2).unwrap());
}
