const INPUT: &'static str = include_str!("../../input");
// const INPUT: &'static str = include_str!("../../sample-input");

const STARTING_POSITION: i32 = 50;
const TOTAL_POSITIONS: i32 = 100;

fn parse_instruction(instruction: &str) -> i32 {
    let (direction, clicks) = instruction.split_at(1);
    let clicks = clicks.parse::<i32>().expect("unexpected click count");

    match direction {
        "L" => -clicks,
        "R" => clicks,
        _ => panic!("unexpected direction"),
    }
}

fn part1() {
    let result =
        INPUT
            .lines()
            .map(|x| parse_instruction(x))
            .fold((STARTING_POSITION, 0), |acc, clicks| {
                let next = (acc.0 + clicks).rem_euclid(TOTAL_POSITIONS);
                if next == 0 {
                    (next, acc.1 + 1)
                } else {
                    (next, acc.1)
                }
            });
    println!("Part 1: {}", result.1);
}

fn part2() {
    let result = INPUT
        .lines()
        .map(|x| parse_instruction(x))
        .fold((STARTING_POSITION, 0), |acc, clicks| {
            let total = acc.0 + clicks;

            let complete_revs = (total / TOTAL_POSITIONS).unsigned_abs();
            let crossed_zero = total <= 0 && acc.0 > 0;
            let compensation = if crossed_zero { 1 } else { 0 };

            (total.rem_euclid(TOTAL_POSITIONS), acc.1 + complete_revs + compensation)
        });
    println!("Part 2: {}", result.1);
}

fn main() {
    part1();
    part2();
}
