use aoc2017::day07;
fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day07.txt");

    let part_1 = day07::part_1(input);
    println!("Part 1: {}", part_1);

    Ok(())
}
