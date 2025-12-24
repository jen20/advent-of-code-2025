use itertools::Itertools;
use good_lp::{constraint, variable, variables, Expression, Solution, SolverModel, highs};
use regex::Regex;

const INPUT: &'static str = include_str!("../../input");
// const INPUT: &'static str = include_str!("../../sample-input");

#[derive(Clone, Debug, PartialEq)]
enum Status {
    On,
    Off,
}

#[derive(Clone, Debug)]
struct Machine {
    required: Vec<Status>,
    switches: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

fn main() {
    let machine_re = Regex::new(r#"\[(.*)] (\(.*\)) \{(.*)}"#).unwrap();

    let machines = INPUT
        .lines()
        .map(|l| {
            let (_, [required, switches, joltage]) = machine_re.captures(l).unwrap().extract();

            let required = required
                .chars()
                .map(|c| match c {
                    '.' => Status::Off,
                    '#' => Status::On,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();

            let switches = switches
                .split(" ")
                .map(|s| {
                    s[1..s.len() - 1]
                        .split(",")
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let joltage = joltage
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            Machine {
                required,
                switches,
                joltage,
            }
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&machines));
    println!("Part 2: {}", part2(&machines));
}

fn part1(machines: &Vec<Machine>) -> u64 {
    machines
        .iter()
        .map(|machine| {
            (1..=machine.switches.len())
                .find_map(|count| {
                    machine
                        .switches
                        .iter()
                        .combinations(count)
                        .find(|switches| {
                            let state = (0..machine.required.len())
                                .map(|pos| {
                                    let toggles =
                                        switches.iter().filter(|s| s.contains(&pos)).count();
                                    if toggles % 2 == 0 {
                                        Status::Off
                                    } else {
                                        Status::On
                                    }
                                })
                                .collect::<Vec<_>>();
                            state == machine.required
                        })
                        .map(|_| count as u64)
                })
                .unwrap()
        })
        .sum::<u64>()
}

fn part2(machines: &Vec<Machine>) -> i32 {
    machines
        .iter()
        .map(|m| {
            let num_switches = m.switches.len();
            let num_counters = m.joltage.len();

            variables! {
                vars:
            }

            let switch_vars: Vec<_> = (0..num_switches)
                .map(|_| vars.add(variable().integer().min(0)))
                .collect();

            let objective: Expression = switch_vars.iter().sum();

            let mut problem = vars.minimise(&objective).using(highs);

            for i in 0..num_counters {
                let lhs: Expression = m.switches
                    .iter()
                    .enumerate()
                    .filter(|(_, switch)| switch.contains(&i))
                    .map(|(j, _)| switch_vars[j])
                    .sum();

                problem = problem.with(constraint!(lhs == m.joltage[i] as i32));
            }

            let solution = problem.solve().expect("solves");
            switch_vars.iter().map(|&v| solution.value(v).round() as i32).sum::<i32>()
        })
        .sum()
}
