use aoc2017::day09;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day09.txt").trim();

    let (score, garbage_chars) = day09::process_program(input);

    println!("Part 1: {}", score);
    println!("Part 2: {}", garbage_chars);
    Ok(())
}
