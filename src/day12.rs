use chumsky::prelude::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Graph {
    inner: HashMap<u32, Vec<u32>>,
}

impl Graph {
    pub fn build_graph(input: Vec<InputRow>) -> Self {
        let inner = input
            .into_iter()
            .map(|row| (row.node, row.adjacent))
            .collect::<HashMap<_, _>>();

        Self { inner }
    }

    pub fn find_groups(&self) -> HashMap<u32, u32> {
        let mut groups = self.inner.keys().map(|&k| (k, k)).collect();
        let mut seen: HashSet<u32> = HashSet::new();

        for &node in self.inner.keys().sorted() {
            if seen.contains(&node) {
                continue;
            }

            let mut queue = VecDeque::new();
            queue.push_back(node);

            while let Some(node) = queue.pop_front() {
                if seen.contains(&node) {
                    continue;
                }

                seen.insert(node);

                for &adj in self.inner.get(&node).unwrap().iter() {
                    self.union(node, adj, &mut groups);
                    queue.push_back(adj);
                }
            }
        }

        groups
    }

    fn union(&self, p: u32, q: u32, groups: &mut HashMap<u32, u32>) {
        let p = groups.get(&p).copied().unwrap();
        // set q's group to group p
        *groups.get_mut(&q).unwrap() = p;
        // set everything in group q to group p
        for value in groups.values_mut().filter(|group| **group == q) {
            *value = p;
        }
    }
}

#[derive(Debug, Clone)]
pub struct InputRow {
    node: u32,
    adjacent: Vec<u32>,
}

pub fn parse_input(input: &str) -> Vec<InputRow> {
    let parser = row_parser();

    input
        .trim()
        .lines()
        .filter_map(|line| parser.parse(line).ok())
        .collect()
}

// Sample input:
// 11 <-> 98, 172, 859, 1303
fn row_parser() -> impl Parser<char, InputRow, Error = Simple<char>> {
    let node = text::int::<_, Simple<char>>(10)
        .padded()
        .try_map(|num, span| {
            num.parse::<u32>()
                .map_err(|e| Simple::custom(span, format!("{}", e)))
        });

    let adjacent = node
        .chain(just(',').padded().ignore_then(node).repeated())
        .collect::<Vec<_>>();

    node.then_ignore(just("<->").padded())
        .then(adjacent)
        .map(|(node, adjacent)| InputRow { node, adjacent })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connected_works() {
        let input = "\
            0 <-> 2
            1 <-> 1
            2 <-> 0, 3, 4
            3 <-> 2, 4
            4 <-> 2, 3, 6
            5 <-> 6
            6 <-> 4, 5";

        let input = parse_input(input);
        let graph = Graph::build_graph(input);

        let groups = graph.find_groups();
        let group_zero_size = groups.values().filter(|&&group| group == 0).count();

        assert_eq!(6, group_zero_size);
    }
}
