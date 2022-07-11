use itertools::Itertools;

pub fn part_1(input: &str) -> u32 {
    input
        .chars()
        .chain(input.chars().take(1))
        .filter_map(|c| c.to_digit(10))
        .tuple_windows()
        .fold(0, |acc, (a, b)| if a == b { acc + a } else { acc })
}

pub fn part_2(input: &str) -> u32 {
    input
        .chars()
        .zip(input.chars().cycle().skip(input.len() / 2))
        .filter_map(|(a, b)| Some((a.to_digit(10)?, b.to_digit(10)?)))
        .fold(0, |acc, (a, b)| if a == b { acc + a } else { acc })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let sum = part_1("1122");
        assert_eq!(3, sum);

        let sum = part_1("91212129");
        assert_eq!(9, sum);
    }

    #[test]
    fn part_2_works() {
        let sum = part_2("12131415");
        assert_eq!(4, sum);

        let sum = part_2("1212");
        assert_eq!(6, sum);

        let sum = part_2("123425");
        assert_eq!(4, sum);

        let sum = part_2("123123");
        assert_eq!(12, sum);

        let sum = part_2("1221");
        assert_eq!(0, sum);
    }
}
