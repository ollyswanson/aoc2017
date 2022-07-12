pub fn parse_input(input: &str) -> Vec<isize> {
    input
        .lines()
        .filter_map(|line| line.trim().parse::<isize>().ok())
        .collect()
}

pub fn part_1(jumps: &mut [isize]) -> usize {
    let mut pos: isize = 0;
    let mut num_jumps = 0;

    loop {
        if pos < 0 {
            return num_jumps;
        }

        match jumps.get_mut(pos as usize) {
            None => return num_jumps,
            Some(jump) => {
                pos += *jump;
                *jump += 1;
            }
        }

        num_jumps += 1;
    }
}

pub fn part_2(jumps: &mut [isize]) -> usize {
    let mut pos: isize = 0;
    let mut num_jumps = 0;

    loop {
        if pos < 0 {
            return num_jumps;
        }

        match jumps.get_mut(pos as usize) {
            None => return num_jumps,
            Some(jump) => {
                pos += *jump;
                if *jump >= 3 {
                    *jump -= 1;
                } else {
                    *jump += 1;
                }
            }
        }

        num_jumps += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let input = "
            0
            3
            0
            1
           -3";

        let mut input = parse_input(input);
        let num_jumps = part_1(&mut input);

        assert_eq!(5, num_jumps);
    }

    #[test]
    fn part_2_works() {
        let input = "
            0
            3
            0
            1
           -3";

        let mut input = parse_input(input);
        let num_jumps = part_2(&mut input);

        assert_eq!(10, num_jumps);
    }
}
