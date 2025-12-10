const INPUT: &'static str = include_str!("../../input");
// const INPUT: &'static str = include_str!("../../sample-input");

fn process_row(line: &str, beams: &mut Vec<usize>) -> usize {
    let mut split_count = 0;
    for (i, c) in line.chars().enumerate() {
        if c == '^' && beams[i] >= 1 {
            beams[i - 1] += beams[i];
            beams[i + 1] += beams[i];
            beams[i] = 0;
            split_count += 1;
        }
    }
    split_count
}

fn main() {
    let mut lines = INPUT.lines();
    let mut split_count = 0;
    let mut beams = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'S' { 1 } else { 0 })
        .collect::<Vec<_>>();

    while let Some(line) = lines.next() {
        split_count += process_row(line, &mut beams);
    }

    println!("Part 1: {}", split_count);
    println!("Part 2: {}", beams.iter().sum::<usize>());
}
