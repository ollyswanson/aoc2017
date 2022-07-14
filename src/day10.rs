use itertools::Itertools;

type Skip = usize;
type TotalRotation = usize;

pub fn part_1(input: &str) -> anyhow::Result<u32> {
    let lengths: Vec<usize> = input.split(',').map(str::parse).collect::<Result<_, _>>()?;

    let mut knot = init_knot::<256>();
    let mut skip: Skip = 0;
    let mut total_rotation: TotalRotation = 0;
    knot_procedure(&lengths, &mut knot, &mut skip, &mut total_rotation);
    Ok(score(&knot, total_rotation))
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    let lengths: Vec<usize> = input
        .bytes()
        .map(|b| b as usize)
        .chain([17, 31, 73, 47, 23].into_iter())
        .collect();

    let mut knot = init_knot::<256>();
    let mut total_rotation = 0;
    let mut skip = 0;

    for _ in 0..64 {
        knot_procedure(&lengths, &mut knot, &mut skip, &mut total_rotation);
    }

    total_rotation &= 255;
    let start = (256 - total_rotation) & 255;
    knot.rotate_left(start);

    Ok(knot
        .into_iter()
        .chunks(16)
        .into_iter()
        .map(|chunk| chunk.into_iter().fold(0, |acc, cur| cur ^ acc))
        .map(|byte| format!("{:02x?}", byte))
        .collect())
}

fn knot_procedure<const N: usize>(
    lengths: &[usize],
    knot: &mut [u8; N],
    skip: &mut Skip,
    total_rotation: &mut TotalRotation,
) {
    assert!(N & (N - 1) == 0);

    for &length in lengths.iter() {
        let to_reverse = &mut knot[0..length];
        to_reverse.reverse();

        let rotation = (length + *skip) & (N - 1);
        knot.rotate_left(rotation);
        *total_rotation += rotation;
        *skip += 1;
    }
}

fn score<const N: usize>(knot: &[u8; N], mut total_rotation: TotalRotation) -> u32 {
    total_rotation &= N - 1;
    let first = (N - total_rotation) & (N - 1);
    let second = (first + 1) & (N - 1);

    knot[first] as u32 * knot[second] as u32
}

fn init_knot<const N: usize>() -> [u8; N] {
    let mut knot = [0u8; N];
    for (i, v) in knot.iter_mut().enumerate() {
        *v = i as u8;
    }
    knot
}
