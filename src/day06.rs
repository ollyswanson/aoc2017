use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

pub fn part_1<const N: usize>(input: &str) -> Option<usize> {
    let mut memory_bank = MemoryBank::<N>::from(input);
    let mut history = History::<N>::new();
    history.insert(memory_bank.clone());

    for i in 1.. {
        memory_bank = memory_bank.redistribute();
        if !history.insert(memory_bank.clone()) {
            return Some(i);
        }
    }

    None
}

pub fn part_2<const N: usize>(input: &str) -> Option<usize> {
    let mut memory_bank = MemoryBank::<N>::from(input);
    let mut history = LoopHistory::<N>::new();
    history.insert(memory_bank.clone(), 0);

    for i in 1.. {
        memory_bank = memory_bank.redistribute();
        match history.entry(memory_bank.clone()) {
            Entry::Occupied(e) => return Some(i - *e.get()),
            Entry::Vacant(e) => {
                e.insert(i);
            }
        }
    }

    None
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MemoryBank<const N: usize> {
    buckets: [u8; N],
}

type History<const N: usize> = HashSet<MemoryBank<N>>;

type LoopHistory<const N: usize> = HashMap<MemoryBank<N>, usize>;

impl<const N: usize> MemoryBank<N> {
    const MASK: usize = N - 1;

    fn new() -> Self {
        // Find a way to replace this with a static assertion :)
        assert!(N > 1 && N & (N - 1) == 0);
        Self { buckets: [0; N] }
    }

    pub fn redistribute(&self) -> Self {
        let mut next = self.clone();

        let (mut i, mut max) = next
            .buckets
            .iter()
            .copied()
            .enumerate()
            .rev()
            .max_by_key(|(_, n)| *n)
            .expect("MemoryBank has at least 2 buckets");

        next.buckets[i] = 0;

        while max > 0 {
            i += 1;
            i &= Self::MASK;
            next.buckets[i] += 1;
            max -= 1;
        }

        next
    }
}

impl<const N: usize> From<&str> for MemoryBank<N> {
    fn from(input: &str) -> Self {
        let mut memory_bank = MemoryBank::new();

        input
            .split_whitespace()
            .filter_map(|tok| tok.parse::<u8>().ok())
            .enumerate()
            .take(N)
            .for_each(|(i, n)| {
                memory_bank.buckets[i] = n;
            });

        memory_bank
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redistribute_works() {
        let input = "0 2 7 0";
        let expected = MemoryBank {
            buckets: [2, 4, 1, 2],
        };

        let memory_bank = MemoryBank::<4>::from(input);
        let next = memory_bank.redistribute();

        assert_eq!(expected, next);
    }

    #[test]
    fn part_1_works() {
        let input = "0 2 7 0";
        let num_cycles = part_1::<4>(input);

        assert_eq!(Some(5), num_cycles);
    }

    #[test]
    fn part_2_works() {
        let input = "0 2 7 0";
        let loop_length = part_2::<4>(input);

        assert_eq!(Some(4), loop_length);
    }
}
