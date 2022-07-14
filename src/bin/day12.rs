use std::collections::HashSet;

use aoc2017::day12;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day12.txt").trim();
    let input = day12::parse_input(input);

    let graph = day12::Graph::build_graph(input);
    let groups = graph.find_groups();

    let part_1 = groups.values().filter(|&&group| group == 0).count();
    println!("Part 1: {}", part_1);

    let part_2 = groups.values().collect::<HashSet<_>>().len();
    println!("Part 2: {}", part_2);

    Ok(())
}
