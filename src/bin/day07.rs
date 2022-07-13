use aoc2017::day07;
fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day07.txt");
    let program_tree = day07::ProgramTree::build_tree(input);

    println!("Part 1: {}", program_tree.root);

    let part_2 = program_tree
        .find_branch_weight(&program_tree.root)
        .unwrap_err();

    println!("Part 2: {}", part_2);

    Ok(())
}
