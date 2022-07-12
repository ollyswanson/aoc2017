use aoc2017::day05;
fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day05.txt");
    let mut input = day05::parse_input(input);

    let part_1 = day05::part_1(&mut input.clone());
    println!("Part 1: {}", part_1);

    let part_2 = day05::part_2(&mut input);
    println!("Part 2: {}", part_2);

    Ok(())
}
