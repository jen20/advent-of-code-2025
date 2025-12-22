use itertools::Itertools;
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
    // joltage: Vec<usize>,
}

fn main() {
    let machine_re = Regex::new(r#"\[(.*)] (\(.*\)) \{(.*)}"#).unwrap();

    let machines = INPUT
        .lines()
        .map(|l| {
            let (_, [required, switches, _joltage]) = machine_re.captures(l).unwrap().extract();

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

            // let joltage = joltage
            //     .split(",")
            //     .map(|n| n.parse::<usize>().unwrap())
            //     .collect::<Vec<_>>();

            Machine {
                required,
                switches,
                // joltage,
            }
        })
        .collect::<Vec<_>>();

    let part1 = machines
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
        .sum::<u64>();

    println!("Part 1: {}", part1);
}
