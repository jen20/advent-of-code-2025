use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use std::cmp::Ordering;

const INPUT: &'static str = include_str!("../../input");
const NUM_CONNECTIONS: usize = 1000;
// const INPUT: &'static str = include_str!("../../sample-input");
// const NUM_CONNECTIONS: usize = 10;

fn main() {
    let boxes = INPUT
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.splitn(3, ",")
                .map(|c| c.parse::<i64>().unwrap())
                .collect_tuple::<(_, _, _)>()
                .unwrap()
        })
        .collect::<Vec<_>>();
    let n = boxes.len();

    let distances = (0..n)
        .tuple_combinations()
        .map(|(i, j)| (euclidean_distance(boxes[i], boxes[j]), i, j))
        .sorted_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal))
        .collect::<Vec<_>>();

    // Part 1
    let mut uf = UnionFind::<usize>::new(n);
    distances
        .iter()
        .take(NUM_CONNECTIONS)
        .for_each(|(_, i, j)| {
            uf.union(*i, *j);
        });

    let circuit_sizes = (0..n)
        .map(|i| uf.find(i))
        .counts()
        .values()
        .copied()
        .sorted()
        .rev()
        .collect::<Vec<_>>();

    println!(
        "Part 1: {}",
        circuit_sizes.iter().take(3).product::<usize>()
    );

    // Part 2
    let mut uf = UnionFind::<usize>::new(n);
    let mut n = n;
    let mut last_connection = (0, 0);

    for (_, i, j) in distances.iter() {
        if uf.find(*i) != uf.find(*j) {
            uf.union(*i, *j);
            n -= 1;
            last_connection = (*i, *j);

            if n == 1 {
                break;
            }
        }
    }

    println!(
        "Part 2: {}",
        boxes[last_connection.0].0 * boxes[last_connection.1].0
    );
}

fn euclidean_distance(a: (i64, i64, i64), b: (i64, i64, i64)) -> f64 {
    let dx = (a.0 - b.0) as f64;
    let dy = (a.1 - b.1) as f64;
    let dz = (a.2 - b.2) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}
