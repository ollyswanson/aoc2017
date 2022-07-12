use aoc2017::day06;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day06.txt");

    let part_1 = day06::part_1::<16>(input);
    println!("Part 1: {}", part_1.unwrap());

    let part_2 = day06::part_2::<16>(input);
    println!("Part 2: {}", part_2.unwrap());

    Ok(())
}
