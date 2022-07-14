use aoc2017::day11;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day11.txt").trim();

    let journey = day11::parse_journey(input)?;
    let (distance, furthest) = day11::distance(&journey);

    println!("Part 1: {}", distance);
    println!("Part 2: {}", furthest);

    Ok(())
}
