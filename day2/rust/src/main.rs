use std::ops::Range;

const INPUT: &'static str = include_str!("../../input");
// const INPUT: &'static str = include_str!("../../sample-input");

fn is_valid_part1(s: String) -> bool {
    let mid = s.len() / 2;
    s[..mid] == s[mid..]
}

fn is_valid_part2(s: String) -> bool {
    let n = s.len();
    for len in 1..=n / 2 {
        if n % len == 0 {
            let part = &s[..len];
            if part.repeat(n / len) == s {
                return true;
            }
        }
    }

    false
}

fn parse_range<S: AsRef<str>>(range: S) -> Range<u64> {
    let (start, end) = range
        .as_ref()
        .split_once('-')
        .expect("range formatted as <start>-<end>");
    let start = start.parse::<u64>().expect("start is a u32");
    let end = end.parse::<u64>().expect("start is a u32");
    start..end + 1
}

fn main() {
    let part1 = INPUT
        .trim()
        .split(',')
        .map(|range| {
            parse_range(range)
                .filter(|x| is_valid_part1(format!("{}", x)))
                .sum::<u64>()
        })
        .sum::<u64>();

    println!("Part 1: {}", part1);

    let part2 = INPUT
        .trim()
        .split(',')
        .map(|range| {
            parse_range(range)
                .filter(|x| is_valid_part2(format!("{}", x)))
                .sum::<u64>()
        })
        .sum::<u64>();

    println!("Part 2: {}", part2);
}
