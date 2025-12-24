use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

// const INPUT_PART1: &'static str = include_str!("../../sample-input-part1");
// const INPUT_PART2: &'static str = include_str!("../../sample-input-part2");
const INPUT_PART1: &'static str = include_str!("../../input");
const INPUT_PART2: &'static str = include_str!("../../input");

fn main() {
    let puzzle = Puzzle::new(INPUT_PART1);
    println!("Part 1: {}", puzzle.part1());

    let puzzle = Puzzle::new(INPUT_PART2);
    println!("Part 2: {}", puzzle.part2());
}

struct Puzzle<'a> {
    graph: DiGraph<&'a str, ()>,
    nodes: HashMap<&'a str, NodeIndex>,
}

impl<'a> Puzzle<'a> {
    fn new(input: &'a str) -> Puzzle<'a> {
        let mut graph = DiGraph::<&str, ()>::new();
        let mut nodes = HashMap::<&str, NodeIndex>::new();

        for (a, b) in input.lines().flat_map(|l| {
            let (source, dests) = l.split_once(":").unwrap();
            dests.split_whitespace().map(move |d| (source, d))
        }) {
            let a_idx = *nodes.entry(a).or_insert_with(|| graph.add_node(a));
            let b_idx = *nodes.entry(b).or_insert_with(|| graph.add_node(b));
            graph.add_edge(a_idx, b_idx, ());
        }

        Self { graph, nodes }
    }

    fn part1(&self) -> usize {
        let start = self.nodes.get("you").unwrap().clone();
        let end = self.nodes.get("out").unwrap().clone();

        let mut memo = HashMap::new();
        self.count_paths(start, end, &mut memo)
    }

    fn part2(&self) -> usize {
        let start = self.nodes.get("svr").unwrap().clone();
        let end = self.nodes.get("out").unwrap().clone();
        let through1 = *self.nodes.get("fft").unwrap();
        let through2 = *self.nodes.get("dac").unwrap();

        let mut memo = HashMap::new();

        let paths1 = self.count_paths(start, through1, &mut memo)
            * self.count_paths(through1, through2, &mut memo)
            * self.count_paths(through2, end, &mut memo);

        let paths2 = self.count_paths(start, through2, &mut memo)
            * self.count_paths(through2, through1, &mut memo)
            * self.count_paths(through1, end, &mut memo);

        paths1 + paths2
    }

    fn count_paths(
        &self,
        start: NodeIndex,
        end: NodeIndex,
        memo: &mut HashMap<(NodeIndex, NodeIndex), usize>,
    ) -> usize {
        if start == end {
            return 1;
        }

        if let Some(&count) = memo.get(&(start, end)) {
            return count;
        }

        let mut count = 0;
        for neighbor in self.graph.neighbors(start) {
            count += self.count_paths(neighbor, end, memo);
        }

        memo.insert((start, end), count);
        count
    }
}
