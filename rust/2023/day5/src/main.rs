use std::{collections::BTreeSet, cmp};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Range {
    start_inclusive: i64,
    end_exclusive: i64,
}

impl Range {
    fn validated(s: i64, e: i64) -> Option<Range> {
        if s < e { Some(Range { start_inclusive: s, end_exclusive: e }) }
        else { None }
    }

    fn cut_and_translate(&self, mask: Range, delta: i64) -> (Option<Range>, BTreeSet<Range>) {
        let s = cmp::max(self.start_inclusive, mask.start_inclusive);
        let e = cmp::min(mask.end_exclusive, self.end_exclusive);
        let mut set: BTreeSet<Range> = BTreeSet::new();
        match Range::validated(s, e) {
            Some(i) => {
                let newr = Range {
                    start_inclusive: i.start_inclusive + delta,
                    end_exclusive: i.end_exclusive + delta
                };
                if let Some(r) = Range::validated(mask.start_inclusive, self.start_inclusive) {
                    set.insert(r);
                }
                if let Some(r) = Range::validated(i.end_exclusive, mask.end_exclusive) {
                    set.insert(r);
                }
                (Some(newr), set)
            },
            None => {
                set.insert(self.clone());
                (None, set)
            }
        }
    }
}

fn main() {
    let input = include_str!("input.txt").lines().collect::<Vec<&str>>();
}

#[cfg(test)]
mod tests {
    use super::Range;

    #[test]
    fn ranges() {
        let r1 = Range { start_inclusive: 10, end_exclusive: 20 };
        let r2 = Range { start_inclusive: 5, end_exclusive: 30 };

        let (newr, rest) = r1.cut_and_translate(r2, 2);
        assert_eq!(newr, Some(Range {
            start_inclusive: 12,
            end_exclusive: 22
        }));

        let rest = rest.iter().collect::<Vec<&Range>>();
        assert_eq!(rest, vec![
            &Range { start_inclusive: 5, end_exclusive: 10 },
            &Range { start_inclusive: 20, end_exclusive: 30 }
        ]);
    }
}