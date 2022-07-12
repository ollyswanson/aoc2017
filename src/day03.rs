use std::collections::HashMap;

use itertools::Itertools;

/// 17 16 15 14 13
/// 18  5  4  3 12
/// 19  6  1  2 11
/// 20  7  8  9 10
/// 21 22 23 24 25
pub fn part_1(input: &str) -> i32 {
    let value: i32 = input.parse().unwrap();

    let (ring, square) = get_nearest_square(value);

    // to generate arbitrary sequence 0,1,2..n-1,n,n-1,..2,1,0,1,2.. we can use the formula
    // y = -|x-n-2n.floor(x/2n)| + n
    let x = square - value;
    let n = ring;
    let adjustment = -(x - n - 2 * n * x.div_euclid(2 * n)).abs() + n;

    2 * ring - adjustment
}

// takes a value and calculates the nearest perfect square >= the value and the ring in which it
// can be found
fn get_nearest_square(value: i32) -> (i32, i32) {
    (1..)
        .step_by(2)
        .map(|n| n * n)
        .enumerate()
        .find(|&(_, sq)| sq >= value)
        .map(|(i, sq)| (i as i32, sq))
        .unwrap()
}

pub fn part_2(input: &str) -> Option<i32> {
    let value: i32 = input.parse().unwrap();

    let mut pos = (0, 0);
    let mut spiral: HashMap<(isize, isize), i32> = HashMap::new();

    spiral.insert(pos, 1);
    pos.0 += 1;

    for i in (3..).step_by(2) {
        for _ in 0..i - 2 {
            let new = set_value_from_surrounding(pos, &mut spiral);
            if new > value {
                return Some(new);
            }

            pos.1 += 1;
        }
        for _ in 0..i - 1 {
            let new = set_value_from_surrounding(pos, &mut spiral);
            if new > value {
                return Some(new);
            }

            pos.0 -= 1;
        }
        for _ in 0..i - 1 {
            let new = set_value_from_surrounding(pos, &mut spiral);
            if new > value {
                return Some(new);
            }

            pos.1 -= 1;
        }
        for _ in 0..i {
            let new = set_value_from_surrounding(pos, &mut spiral);
            if new > value {
                return Some(new);
            }

            pos.0 += 1;
        }
    }

    None
}

const ADJACENT: [(isize, isize); 8] = [
    (-1, 1),
    (0, 1),
    (1, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

fn set_value_from_surrounding(
    pos: (isize, isize),
    spiral: &mut HashMap<(isize, isize), i32>,
) -> i32 {
    let sum: i32 = ADJACENT
        .iter()
        .filter_map(|&adj| spiral.get(&(pos.0 + adj.0, pos.1 + adj.1)))
        .sum();

    spiral.insert(pos, sum);

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_ring_works() {
        assert_eq!((1, 9), get_nearest_square(8));
        assert_eq!((2, 25), get_nearest_square(10));
        assert_eq!((2, 25), get_nearest_square(25));
    }

    #[test]
    fn part_1_works() {
        assert_eq!(6, part_1("49"));
        assert_eq!(4, part_1("25"));
        assert_eq!(2, part_1("23"));
        assert_eq!(3, part_1("12"));
        assert_eq!(31, part_1("1024"));
    }
}
