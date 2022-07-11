use itertools::Itertools;

pub fn part_1(input: &str) -> u32 {
    use itertools::MinMaxResult::*;
    let mut checksum = 0;

    for line in input.lines() {
        match line
            .split_whitespace()
            .filter_map(|c| c.parse::<u32>().ok())
            .minmax()
        {
            NoElements => {}
            OneElement(_) => {}
            MinMax(min, max) => checksum += max - min,
        }
    }

    checksum
}

pub fn part_2(input: &str) -> u32 {
    let mut checksum = 0;

    for line in input.lines() {
        checksum += line
            .split_whitespace()
            .filter_map(|c| c.parse::<u32>().ok())
            .sorted()
            .rev()
            .tuple_combinations()
            .find(|(a, b)| a % b == 0)
            .map(|(a, b)| a / b)
            .unwrap();
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let input = "\
            5 1 9 5
            7 5 3
            2 4 6 8";

        let checksum = part_1(input);

        assert_eq!(18, checksum);
    }

    #[test]
    fn part_2_works() {
        let input = "\
            5 9 2 8
            9 4 7 3
            3 8 6 5";

        let checksum = part_2(input);

        assert_eq!(9, checksum);
    }
}
