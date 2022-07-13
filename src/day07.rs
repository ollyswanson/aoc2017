use chumsky::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct ProgramTree {
    pub root: String,
    tree: HashMap<String, Vec<String>>,
    weights: HashMap<String, u32>,
}

impl ProgramTree {
    pub fn build_tree(input: &str) -> Self {
        let input = Input::from(input);
        let mut has_parent: HashSet<String> = HashSet::new();
        let mut tree = HashMap::with_capacity(input.0.len());
        let mut weights = HashMap::with_capacity(input.0.len());

        for InputRow {
            parent,
            weight,
            children,
        } in input.0
        {
            weights.insert(parent.clone(), weight);
            tree.insert(parent.clone(), Vec::new());

            if let Some(children) = children {
                let c = tree.get_mut(&parent).unwrap();
                for child in children {
                    has_parent.insert(child.clone());
                    c.push(child);
                }
            }
        }

        let all_nodes: HashSet<String> = tree.keys().cloned().collect();
        let root = all_nodes.difference(&has_parent).next().unwrap().clone();

        ProgramTree {
            root,
            tree,
            weights,
        }
    }

    /// Returns Ok(branch_weight) or Err(required_node_weight) if the children are unbalanced.
    pub fn find_branch_weight(&self, node: &str) -> Result<u32, u32> {
        let children_weights: Vec<u32> = self.tree[node]
            .iter()
            .map(|child| self.find_branch_weight(child))
            .collect::<Result<_, _>>()?;

        let mut occurences: HashMap<u32, u32> = HashMap::new();

        for weight in children_weights.iter() {
            let count = occurences.entry(*weight).or_insert(0);
            *count += 1;
        }

        if occurences.len() > 1 {
            // There will only be 1 unbalanced weight, as specified by the problem
            let unbalanced_weight = occurences.into_iter().find(|(_, v)| *v == 1).unwrap().0;
            let desired_weight = *children_weights
                .iter()
                .find(|&&w| w != unbalanced_weight)
                .unwrap();

            let delta = desired_weight as i32 - unbalanced_weight as i32;

            let index = children_weights
                .iter()
                .enumerate()
                .find(|&(_, &w)| w == unbalanced_weight)
                .unwrap()
                .0;

            let child = &self.tree[node][index];
            let child_weight = self.weights[child];

            return Err((child_weight as i32 + delta as i32) as u32);
        }

        Ok(self.weights[node] + children_weights.iter().sum::<u32>())
    }
}

#[derive(Debug, PartialEq)]
struct InputRow {
    parent: String,
    weight: u32,
    children: Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
struct Input(Vec<InputRow>);

impl From<&str> for Input {
    fn from(input: &str) -> Self {
        Input(input.lines().map(InputRow::from).collect())
    }
}

impl From<&str> for InputRow {
    fn from(input: &str) -> Self {
        parse_row(input)
    }
}

fn parse_row(input: &str) -> InputRow {
    let name = text::ident::<_, Simple<char>>();
    let weight = text::int(10)
        .delimited_by(just('('), just(')'))
        .map(|n| n.parse::<u32>().unwrap());

    let children = name
        .chain(just(',').padded().ignore_then(name).repeated())
        .or_not()
        .flatten()
        .collect::<Vec<String>>();

    let row = name
        .padded()
        .then(weight.padded())
        .then(just("->").padded().ignore_then(children).or_not())
        .map(|((parent, weight), children)| InputRow {
            parent,
            weight,
            children,
        });

    row.parse(input).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_row_works() {
        let input = "a (1) -> b, c";
        let expected = InputRow {
            parent: "a".to_owned(),
            weight: 1,
            children: Some(vec!["b".to_owned(), "c".to_owned()]),
        };

        let row = parse_row(input);
        assert_eq!(expected, row);
    }

    #[test]
    fn find_root_works() {
        let input = "\
            pbga (66)
            xhth (57)
            ebii (61)
            havc (66)
            ktlj (57)
            fwft (72) -> ktlj, cntj, xhth
            qoyq (66)
            padx (45) -> pbga, havc, qoyq
            tknk (41) -> ugml, padx, fwft
            jptl (61)
            ugml (68) -> gyxo, ebii, jptl
            gyxo (61)
            cntj (57)";
        let expected = "tknk";

        let tree = ProgramTree::build_tree(input);

        assert_eq!(expected, &tree.root);
    }

    #[test]
    fn find_unbalanced_node_works() {
        let input = "\
            pbga (66)
            xhth (57)
            ebii (61)
            havc (66)
            ktlj (57)
            fwft (72) -> ktlj, cntj, xhth
            qoyq (66)
            padx (45) -> pbga, havc, qoyq
            tknk (41) -> ugml, padx, fwft
            jptl (61)
            ugml (68) -> gyxo, ebii, jptl
            gyxo (61)
            cntj (57)";

        let tree = ProgramTree::build_tree(input);
        let needed_weight = tree.find_branch_weight(&tree.root).unwrap_err();

        assert_eq!(60, needed_weight);
    }
}
