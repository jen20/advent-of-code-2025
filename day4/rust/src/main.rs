use std::collections::HashSet;
use std::iter::successors;

const INPUT: &'static str = include_str!("../../input");
// const INPUT: &'static str = include_str!("../../sample-input");

fn main() {
    let grid = Grid::from(INPUT);
    println!("Part 1: {}", &grid.accessible_cells().iter().count());

    let part2: usize = successors(Some(grid), |g| g.next_grid())
        .map(|g| g.accessible_cells().len())
        .sum();
    println!("Part 2: {}", part2)
}

struct Grid(HashSet<(i32, i32)>);

impl Grid {
    fn next_grid(&self) -> Option<Self> {
        let accessible = self.accessible_cells();
        let next: HashSet<_> = self.0.difference(&accessible).copied().collect();

        if next.is_empty() || next == self.0 {
            None
        } else {
            Some(Self(next))
        }
    }

    fn accessible_cells(&self) -> HashSet<(i32, i32)> {
        self.0
            .iter()
            .cloned()
            .filter(|cell| self.cell_accessible(cell))
            .collect()
    }

    fn cell_accessible(&self, cell: &(i32, i32)) -> bool {
        const DIRS: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        DIRS.iter()
            .filter(|(dx, dy)| self.0.contains(&(cell.0 + dx, cell.1 + dy)))
            .count()
            < 4
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        Self(
            value
                .lines()
                .enumerate()
                .flat_map(|(m, line)| {
                    line.chars().enumerate().filter_map(move |(n, c)| match c {
                        '@' => Some((m as i32, n as i32)),
                        _ => None,
                    })
                })
                .collect::<HashSet<_>>(),
        )
    }
}
