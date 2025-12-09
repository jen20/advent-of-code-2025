use itertools::Itertools;
use std::cmp::min;
use std::iter::once;

const INPUT: &'static str = include_str!("../../input");
// const INPUT: &'static str = include_str!("../../sample-input");

fn main() {
    let char_lines = INPUT
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let max_line_length = char_lines.iter().map(|l| l.len()).max().unwrap();

    let boundaries: Vec<usize> = (0..max_line_length)
        .filter(|&i| char_lines.iter().all(|line| line.get(i) == Some(&' ')))
        .collect();
    let boundaries = once(0).chain(boundaries).chain(once(max_line_length));
    let col_ranges = boundaries.tuple_windows::<(_, _)>();

    let columns = col_ranges
        .map(|(start, end)| {
            char_lines
                .iter()
                .map(|line| {
                    let end = min(end, line.len());
                    line[start..end].iter().collect::<String>()
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let part1 = columns
        .clone()
        .iter()
        .map(|col| {
            let operands = col
                .iter()
                .map_while(|row| row.trim().parse::<u64>().ok())
                .collect_vec();

            match col.last().unwrap().trim() {
                "*" => operands.iter().fold(1, |acc, next| acc * next),
                "+" => operands.iter().fold(0, |acc, next| acc + next),
                _ => unreachable!(),
            }
        })
        .sum::<u64>();
    println!("Part 1: {}", part1);

    let part2 = columns
        .iter()
        .map(|col| {
            let max_len = col.iter().map(|l| l.len()).max().unwrap();
            let rows = col
                .iter()
                .map(|row| {
                    if row.len() == max_len {
                        row.to_string()
                    } else {
                        row.to_string() + &" ".repeat(max_len - row.len())
                    }
                })
                .collect_vec();

            let operands = (0..=max_len)
                .map(|i| {
                    rows[0..rows.len() - 1]
                        .iter()
                        .filter_map(|r| r.chars().nth(i))
                        .collect::<String>()
                        .trim()
                        .to_string()
                })
                .filter(|x| !x.trim().is_empty())
                .map(|x| x.parse::<u64>().unwrap())
                .collect_vec();

            match col.last().unwrap().trim() {
                "*" => operands.iter().fold(1, |acc, next| acc * next),
                "+" => operands.iter().fold(0, |acc, next| acc + next),
                _ => unreachable!(),
            }
        })
        .sum::<u64>();

    println!("Part 2: {}", part2);
}
