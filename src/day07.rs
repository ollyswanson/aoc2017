use chumsky::prelude::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn part_1(input: &str) -> String {
    let input = Input::from(input);
    let tree = ProgramTree::build_tree(input);

    tree.find_root().to_owned()
}

#[derive(Debug, Clone, PartialEq)]
struct ProgramTree {
    /// The index of an element in `program_names` is equal to the index of its entry in `tree`.
    program_names: Vec<String>,
    /// Each element in the vec holds the index of its parent, at the root of the tree the index
    /// points to itself.
    tree: Vec<usize>,
}

impl ProgramTree {
    fn build_tree(input: Input) -> Self {
        let len = input.0.len();
        let mut program_tree = ProgramTree {
            program_names: Vec::with_capacity(len),
            tree: Vec::with_capacity(len),
        };
        let mut seen_at: HashMap<&str, usize> = HashMap::new();
        let mut index = 0;

        for InputRow {
            parent, children, ..
        } in input.0.iter()
        {
            let parent_pos = match seen_at.entry(parent) {
                Entry::Occupied(e) => *e.get(),
                Entry::Vacant(e) => {
                    e.insert(index); // Populate seen_at with the index of the name
                    program_tree.program_names.push(parent.to_owned());
                    program_tree.tree.push(index); // Insert into program tree setting the node's parent to self
                    let tmp = index;
                    index += 1;
                    tmp
                }
            };

            for child in children.iter() {
                match seen_at.entry(child) {
                    Entry::Occupied(e) => program_tree.tree[*e.get()] = parent_pos,
                    Entry::Vacant(e) => {
                        e.insert(index);
                        program_tree.program_names.push(child.to_owned());
                        program_tree.tree.push(parent_pos);
                        index += 1;
                    }
                };
            }
        }

        program_tree
    }

    fn find_root(&self) -> &str {
        let index = self
            .tree
            .iter()
            .enumerate()
            .find(|(i, parent)| i == *parent)
            .unwrap();

        &self.program_names[index.0]
    }
}

#[derive(Debug, PartialEq)]
struct InputRow {
    parent: String,
    weight: u32,
    children: Vec<String>,
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
            children: children.unwrap_or_default(),
        });

    row.parse(input).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_tree_works() {
        let input: Input = Input(vec![
            InputRow {
                parent: "a".to_owned(),
                weight: 2,
                children: vec![],
            },
            InputRow {
                parent: "b".to_owned(),
                weight: 2,
                children: vec![],
            },
            InputRow {
                parent: "c".to_owned(),
                weight: 2,
                children: vec![],
            },
            InputRow {
                parent: "d".to_owned(),
                weight: 6,
                children: vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
            },
        ]);

        let expected = ProgramTree {
            program_names: vec![
                "a".to_owned(),
                "b".to_owned(),
                "c".to_owned(),
                "d".to_owned(),
            ],
            tree: vec![3, 3, 3, 3],
        };

        let program_tree = ProgramTree::build_tree(input);

        assert_eq!(expected, program_tree);
    }

    #[test]
    fn parse_row_works() {
        let input = "a (1) -> b, c";
        let expected = InputRow {
            parent: "a".to_owned(),
            weight: 1,
            children: vec!["b".to_owned(), "c".to_owned()],
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

        let input = Input::from(input);
        let tree = ProgramTree::build_tree(input);

        assert_eq!(expected, tree.find_root());
    }
}
