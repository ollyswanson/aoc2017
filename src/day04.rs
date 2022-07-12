use std::collections::HashSet;

use itertools::Itertools;

struct Passphrase {
    inner: HashSet<String>,
}

enum ChunkRule {
    None,
    Sorted,
}

impl Passphrase {
    fn construct(input: &str, rule: ChunkRule) -> Result<Self, ()> {
        let mut passphrase = HashSet::new();

        for chunk in input.trim().split_whitespace() {
            let chunk = match rule {
                ChunkRule::None => chunk.to_owned(),
                ChunkRule::Sorted => chunk.chars().sorted().collect(),
            };

            let already_present = !passphrase.insert(chunk);
            if already_present {
                return Err(());
            }
        }

        Ok(Self { inner: passphrase })
    }
}

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| Passphrase::construct(line, ChunkRule::None).ok())
        .count()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| Passphrase::construct(line, ChunkRule::Sorted).ok())
        .count()
}
