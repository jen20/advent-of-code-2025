use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &'static str = include_str!("../../input");
// const INPUT: &'static str = include_str!("../../sample-input");

fn part1(vertices: &[(i64, i64)]) -> i64 {
    vertices
        .iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1))
        .max()
        .unwrap()
}

fn part2(vertices: &[(i64, i64)]) -> i64 {
    let x_axis = CompressedAxis::new(vertices.iter().map(|v| v.0));
    let y_axis = CompressedAxis::new(vertices.iter().map(|v| v.1));

    let edges: Vec<_> = vertices.iter().copied().circular_tuple_windows().collect();

    let inside: Vec<Vec<bool>> = x_axis
        .values
        .windows(2)
        .map(|xw| {
            let cx = (xw[0] + xw[1]) as f64 / 2.0;
            y_axis
                .values
                .windows(2)
                .map(|yw| {
                    let cy = (yw[0] + yw[1]) as f64 / 2.0;
                    point_in_polygon((cx, cy), &edges)
                })
                .collect()
        })
        .collect();

    let mut prefix = vec![vec![0i64; y_axis.len()]; x_axis.len()];
    for i in 0..x_axis.len() - 1 {
        for j in 0..y_axis.len() - 1 {
            let val = if inside[i][j] { 1 } else { 0 };
            prefix[i + 1][j + 1] = val + prefix[i][j + 1] + prefix[i + 1][j] - prefix[i][j];
        }
    }

    vertices
        .iter()
        .tuple_combinations()
        .filter_map(|(&(x1, y1), &(x2, y2))| {
            let (min_x, max_x) = (x1.min(x2), x1.max(x2));
            let (min_y, max_y) = (y1.min(y2), y1.max(y2));

            let i1 = x_axis.idx(min_x);
            let i2 = x_axis.idx(max_x);
            let j1 = y_axis.idx(min_y);
            let j2 = y_axis.idx(max_y);

            let total_cells = ((i2 - i1) * (j2 - j1)) as i64;
            let inside_cells = prefix[i2][j2] - prefix[i1][j2] - prefix[i2][j1] + prefix[i1][j1];

            (inside_cells == total_cells).then(|| (max_x - min_x + 1) * (max_y - min_y + 1))
        })
        .max()
        .unwrap()
}

struct CompressedAxis {
    values: Vec<i64>,
    to_idx: HashMap<i64, usize>,
}

impl CompressedAxis {
    fn new(coords: impl Iterator<Item = i64>) -> Self {
        let mut values: Vec<i64> = coords.collect();
        values.sort();
        values.dedup();
        let to_idx = values.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        Self { values, to_idx }
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn idx(&self, value: i64) -> usize {
        self.to_idx[&value]
    }
}

fn point_in_polygon(point: (f64, f64), edges: &[((i64, i64), (i64, i64))]) -> bool {
    let (x, y) = point;
    let mut crossings = 0;

    for &((x1, y1), (x2, y2)) in edges {
        let (y1, y2) = (y1 as f64, y2 as f64);
        let (x1, x2) = (x1 as f64, x2 as f64);

        if (y1 <= y && y < y2) || (y2 <= y && y < y1) {
            let t = (y - y1) / (y2 - y1);
            let x_intersect = x1 + t * (x2 - x1);
            if x < x_intersect {
                crossings += 1;
            }
        }
    }

    crossings % 2 == 1
}

fn main() {
    let vertices = INPUT
        .lines()
        .map(|l| l.split(",").collect_tuple::<(_, _)>().unwrap())
        .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&vertices));
    println!("Part 2: {}", part2(&vertices));
}
