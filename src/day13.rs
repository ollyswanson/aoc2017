use std::collections::HashMap;

use chumsky::prelude::*;

#[derive(Debug)]
pub struct Firewall {
    // Pairs of depths and scanner range
    inner: Vec<(u32, u32)>,
}

impl Firewall {
    pub fn traverse(&self) -> u32 {
        self.inner
            .iter()
            .filter(|&(depth, range)| depth % (2 * (range - 1)) == 0)
            .map(|(depth, range)| depth * range)
            .sum()
    }

    pub fn traverse_without_detection(&self) -> Option<u32> {
        (0..).into_iter().find(|wait| {
            !self
                .inner
                .iter()
                .any(|&(depth, range)| (wait + depth) % (2 * (range - 1)) == 0)
        })
    }

    pub fn parse_firewall(input: &str) -> Firewall {
        let parser = scanner_parser();
        let inner = input
            .trim()
            .lines()
            .map(|line| parser.parse(line))
            .collect::<Result<_, _>>()
            .unwrap();

        Self { inner }
    }
}

fn scanner_parser() -> impl Parser<char, (u32, u32), Error = Simple<char>> {
    let num = text::int::<_, Simple<char>>(10)
        .padded()
        .try_map(|n, span| {
            n.parse::<u32>()
                .map_err(|e| Simple::custom(span, format!("{}", e)))
        });

    num.then_ignore(just(':').padded())
        .then(num)
        .map(|(depth, range)| (depth, range))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity() {
        let input = "\
            0: 3
            1: 2
            4: 4
            6: 4";
        let expected = 24;

        let firewall = Firewall::parse_firewall(input);
        let severity = firewall.traverse();

        assert_eq!(expected, severity);
    }

    #[test]
    fn test_wait() {
        let input = "\
            0: 3
            1: 2
            4: 4
            6: 4";
        let expected = 10;

        let firewall = Firewall::parse_firewall(input);
        let wait = firewall.traverse_without_detection().unwrap();

        assert_eq!(expected, wait);
    }
}
