use std::collections::HashSet;

const INPUT: &str = include_str!("../../sample-input");
// const INPUT: &str = include_str!("../../input");

#[derive(Debug, Clone)]
struct Shape(Vec<(i32, i32)>);

impl Shape {
    fn new(lines: &[&str]) -> Self {
        let cells = lines
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(col, ch)| (ch == '#').then_some((row as i32, col as i32)))
            })
            .collect();
        Shape(cells)
    }

    fn normalize(&self) -> Shape {
        if self.0.is_empty() {
            return Shape(vec![])
        }

        let min_row = self.0.iter().map(|(r, _)| r).min().unwrap();
        let min_col = self.0.iter().map(|(_, c)| c).min().unwrap();

        let cells = self
            .0
            .iter()
            .map(|(r, c)| (r - min_row, c - min_col))
            .collect();

        Shape(cells)
    }

    fn rotate(&self) -> Shape {
        let cells = self.0.iter().map(|(r, c)| (*c, -r)).collect();
        Shape(cells).normalize()
    }

    fn flip(&self) -> Shape {
        let cells = self.0.iter().map(|(r, c)| (*r, -c)).collect();
        Shape(cells).normalize()
    }

    fn all_orientations(&self) -> Vec<Shape> {
        let mut orientations = Vec::new();
        let mut seen = HashSet::new();

        let mut add_rotations = |mut current: Shape| {
            for _ in 0..4 {
                let mut sorted_cells = current.0.clone();
                sorted_cells.sort_unstable();
                if seen.insert(sorted_cells) {
                    orientations.push(current.clone());
                }
                current = current.rotate();
            }
        };

        add_rotations(self.normalize());
        add_rotations(self.flip());

        orientations
    }

    fn can_place(&self, grid: &[Vec<bool>], row: i32, col: i32) -> bool {
        let height = grid.len() as i32;
        let width = grid[0].len() as i32;

        self.0.iter().all(|(dr, dc)| {
            let r = row + dr;
            let c = col + dc;
            r >= 0 && r < height && c >= 0 && c < width && !grid[r as usize][c as usize]
        })
    }

    fn place(&self, grid: &mut [Vec<bool>], row: i32, col: i32) {
        self.0.iter().for_each(|(dr, dc)| {
            grid[(row + dr) as usize][(col + dc) as usize] = true;
        });
    }

    fn unplace(&self, grid: &mut [Vec<bool>], row: i32, col: i32) {
        self.0.iter().for_each(|(dr, dc)| {
            grid[(row + dr) as usize][(col + dc) as usize] = false;
        });
    }
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    required: Vec<usize>,
}

fn parse_input(content: &str) -> (Vec<Shape>, Vec<Region>) {
    let lines: Vec<&str> = content.lines().collect();
    let mut shapes = Vec::new();
    let mut regions = Vec::new();

    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.contains(':') && !line.contains('x') {
            i += 1;
            let mut shape_lines = Vec::new();

            while i < lines.len() && !lines[i].trim().is_empty() {
                shape_lines.push(lines[i]);
                i += 1;
            }

            shapes.push(Shape::new(&shape_lines));
        } else if line.contains('x') {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let dims: Vec<&str> = parts[0].trim().split('x').collect();
                let width = dims[0].parse().unwrap();
                let height = dims[1].parse().unwrap();

                let required: Vec<usize> = parts[1]
                    .trim()
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();

                regions.push(Region {
                    width,
                    height,
                    required,
                });
            }
            i += 1;
        } else {
            i += 1;
        }
    }

    (shapes, regions)
}

fn solve_region(
    shapes: &[Vec<Shape>],
    required: &[(usize, usize)],
    grid: &mut [Vec<bool>],
    idx: usize,
) -> bool {
    if idx == required.len() {
        return true;
    }

    let remaining_cells: usize = grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&cell| !cell)
        .count();

    let needed_cells: usize = required[idx..]
        .iter()
        .map(|(shape_idx, _)| shapes[*shape_idx][0].0.len())
        .sum();

    if needed_cells > remaining_cells {
        return false;
    }

    let (shape_type, _) = required[idx];
    let orientations = &shapes[shape_type];

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    for orientation in orientations {
        for row in 0..height {
            for col in 0..width {
                if orientation.can_place(grid, row, col) {
                    orientation.place(grid, row, col);

                    if solve_region(shapes, required, grid, idx + 1) {
                        orientation.unplace(grid, row, col);
                        return true;
                    }

                    orientation.unplace(grid, row, col);
                }
            }
        }
    }

    false
}

fn can_fit_all_presents(shapes: &[Vec<Shape>], region: &Region) -> bool {
    let mut grid = vec![vec![false; region.width]; region.height];

    let required: Vec<_> = region
        .required
        .iter()
        .enumerate()
        .flat_map(|(shape_idx, &count)| std::iter::repeat((shape_idx, 0)).take(count))
        .collect();

    let total_cells_needed: usize = required
        .iter()
        .map(|(shape_idx, _)| shapes[*shape_idx][0].0.len())
        .sum();

    let available_cells = region.width * region.height;

    if total_cells_needed > available_cells {
        return false;
    }

    solve_region(shapes, &required, &mut grid, 0)
}

fn main() {
    let (shapes, regions) = parse_input(INPUT);

    let shape_orientations: Vec<Vec<Shape>> = shapes.iter().map(|s| s.all_orientations()).collect();

    let count = regions
        .iter()
        .filter(|region| can_fit_all_presents(&shape_orientations, region))
        .count();

    println!("Part 1: {}", count);
}
