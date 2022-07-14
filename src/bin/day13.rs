use aoc2017::day13::Firewall;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day13.txt").trim();

    let firewall = Firewall::parse_firewall(input);
    let severity = firewall.traverse();
    let wait = firewall.traverse_without_detection().unwrap();

    println!("Part 1: {}", severity);
    println!("Part 2: {}", wait);

    Ok(())
}
