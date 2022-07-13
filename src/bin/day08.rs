use aoc2017::day08;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day08.txt");
    let program = day08::parse_program(input);

    let (registers, max_held) = day08::run_program(&program);
    let max = registers.into_iter().map(|(_, v)| v).max().unwrap();

    println!("Part 1: {}", max);
    println!("Part 2: {}", max_held);

    Ok(())
}
