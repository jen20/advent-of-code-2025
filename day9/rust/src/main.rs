use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &'static str = include_str!("../../input");
// const INPUT: &'static str = include_str!("../../sample-input");

fn main() {
    let vertices = INPUT
        .lines()
        .map(|l| l.split(",").collect_tuple::<(_, _)>().unwrap())
        .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
        .collect::<HashSet<_>>();

    let part1 = vertices
        .iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1))
        .max()
        .unwrap();

    println!("Part 1: {}", part1)
}
