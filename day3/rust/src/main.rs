const INPUT: &'static str = include_str!("../../input");
// const INPUT: &'static str = include_str!("../../sample-input");

fn max_joltage<const DIGITS: usize>(batteries: &[u8]) -> u64 {
    let mut acc = 0u64;
    let mut start_index = 0;

    for i in 0..DIGITS {
        let end_index = batteries.len() - (DIGITS - i);
        let mut max_digit = 0;
        let mut max_index = start_index;

        for j in start_index..=end_index {
            if batteries[j] > max_digit {
                max_digit = batteries[j];
                max_index = j;
            }
        }

        acc = acc * 10 + (max_digit - b'0') as u64;
        start_index = max_index + 1;
    }

    acc
}

fn main() {
    let part1: u64 = INPUT.lines().map(|x| max_joltage::<2>(x.as_bytes())).sum();
    println!("Part 1: {}", part1);

    let part2: u64 = INPUT.lines().map(|x| max_joltage::<12>(x.as_bytes())).sum();
    println!("Part 2: {}", part2);
}
