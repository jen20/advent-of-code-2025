use itertools::Itertools;
use std::ops::Range;

const INPUT: &'static str = include_str!("../../input");
// const INPUT: &'static str = include_str!("../../sample-input");

fn main() {
    let puzzle = Puzzle::from(INPUT);
    println!("Part 1: {}", puzzle.part1());
    println!("Part 2: {}", puzzle.part2());
}

#[derive(Debug)]
struct Puzzle {
    ranges: Vec<Range<u64>>,
    available: Vec<u64>,
}

impl Puzzle {
    fn part1(&self) -> usize {
        self.available
            .iter()
            .filter(|ingredient| self.ranges.iter().any(|r| r.contains(ingredient)))
            .count()
    }

    fn part2(&self) -> u64 {
        self.ranges
            .clone()
            .into_iter()
            .sorted_by_key(|r| r.start)
            .coalesce(|a, b| {
                if b.start <= a.end {
                    Ok(a.start..a.end.max(b.end))
                } else {
                    Err((a, b))
                }
            })
            .map(|r| r.end - r.start)
            .sum()
    }
}

impl From<&str> for Puzzle {
    fn from(input: &str) -> Puzzle {
        let mut lines = input.lines();

        let ranges = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| {
                let (start, end) = l.split_once('-').expect("range formatted as <start>-<end>");
                let start = start.parse::<u64>().expect("start is a u64");
                let end = end.parse::<u64>().expect("start is a u64");
                start..end + 1
            })
            .collect::<Vec<_>>();

        let available = lines
            .take_while(|l| !l.is_empty())
            .map(|l| l.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        Puzzle { ranges, available }
    }
}
