use anyhow::anyhow;

pub type Journey = Vec<HexVector>;

pub fn parse_journey(input: &str) -> anyhow::Result<Journey> {
    input
        .trim()
        .split(',')
        .map(HexVector::try_from)
        .collect::<anyhow::Result<_>>()
}

// returns the final distance and the furthest distance
pub fn distance(journey: &Journey) -> (i32, i32) {
    let (final_pos, furthest) =
        journey
            .iter()
            .copied()
            .fold((HexVector::origin(), 0), |(total, furthest), cur| {
                let total = total + cur;
                let furthest = std::cmp::max(furthest, total.abs() / 2);
                (total, furthest)
            });

    (final_pos.abs() / 2, furthest)
}

// A hexagonal grid can be thought of as a 3d coordinate system projected onto 2d space, i.e. with
// a diagonal slice at x + y + z = 0
// The unit vectors are:
// N:  ( 0, -1,  1)
// S:  ( 0,  1, -1)
// NE: ( 1, -1,  0)
// SW: (-1,  1,  0)
// NW: ( 1,  0, -1)
// SE: (-1,  0,  1)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HexVector {
    q: i32,
    r: i32,
    s: i32,
}

impl HexVector {
    const ORIGIN: HexVector = HexVector { q: 0, r: 0, s: 0 };

    fn abs(&self) -> i32 {
        self.q.abs() + self.r.abs() + self.s.abs()
    }

    fn origin() -> Self {
        Self::ORIGIN
    }
}

impl std::ops::Add for HexVector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
            s: self.s + rhs.s,
        }
    }
}

impl TryFrom<&str> for HexVector {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> anyhow::Result<Self> {
        match s {
            "n" => Ok(Self { q: 0, r: -1, s: 1 }),
            "s" => Ok(Self { q: 0, r: 1, s: -1 }),
            "ne" => Ok(Self { q: 1, r: -1, s: 0 }),
            "sw" => Ok(Self { q: -1, r: 1, s: 0 }),
            "nw" => Ok(Self { q: -1, r: 0, s: 1 }),
            "se" => Ok(Self { q: 1, r: 0, s: -1 }),
            unexpected => Err(anyhow!("Unexpected token: {}", unexpected)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_distance_from_start() {
        let input = "ne,ne,s,s";
        let expected = 2;
        let journey = parse_journey(input).unwrap();
        let dist = distance(&journey).0;
        assert_eq!(expected, dist);

        let input = "se,sw,se,sw,sw";
        let expected = 3;
        let journey = parse_journey(input).unwrap();
        let dist = distance(&journey).0;
        assert_eq!(expected, dist);
    }

    #[test]
    fn should_find_furthest_point() {
        let input = "ne,ne,s,s";
        let expected = 2;
        let journey = parse_journey(input).unwrap();
        let furthest = distance(&journey).1;
        assert_eq!(expected, furthest);
    }
}
