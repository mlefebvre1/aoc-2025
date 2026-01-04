use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let input = aoc::fetch_puzzle_input(1)?;
    println!("part1 ans -> {}", part1(&input)?);
    println!("part2 ans -> {}", part2(&input)?);
    Ok(())
}
fn parse_input(input: &str) -> anyhow::Result<Vec<Rotation>> {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(Rotation::from_str)
        .collect()
}

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

impl FromStr for Rotation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('R') {
            let n = s.replace('R', "").parse::<i32>()?;
            Ok(Rotation::Right(n))
        } else if s.starts_with('L') {
            let n = s.replace('L', "").parse::<i32>()?;
            Ok(Rotation::Left(n))
        } else {
            Err(anyhow::anyhow!(
                "Failed to parse rotation. Not right or left.."
            ))
        }
    }
}

fn part1(input: &str) -> anyhow::Result<String> {
    let rotations = parse_input(input)?;
    let mut dial = 50;

    let total = rotations
        .iter()
        .map(|rot| {
            dial = match rot {
                Rotation::Left(n) => dial - n,
                Rotation::Right(n) => dial + n,
            }
            .rem_euclid(100);
            dial
        })
        .filter(|d| *d == 0)
        .count();

    Ok(total.to_string())
}

fn part2(input: &str) -> anyhow::Result<String> {
    let rotations = parse_input(input)?;
    let mut dial = 50;
    let total: usize = rotations
        .iter()
        .map(|rot| match rot {
            Rotation::Left(n) => {
                let (d, crossings) = rotate_left(dial, *n);
                dial = d;
                crossings
            }
            Rotation::Right(n) => {
                let (d, crossings) = rotate_right(dial, *n);
                dial = d;
                crossings
            }
        })
        .sum();

    Ok(total.to_string())
}

fn rotate_right(mut dial: i32, value: i32) -> (i32, usize) {
    let mut crossings = (value / 100).unsigned_abs() as usize; // If the rotation is greater than 100, then we will cross it value / 100 times (integer division).
    let value = value % 100; // Now we can take care of the remainder of the last rotation

    if (dial + value) >= 100 {
        crossings += 1;
    }
    dial = (dial + value).rem_euclid(100);
    (dial, crossings)
}

fn rotate_left(mut dial: i32, value: i32) -> (i32, usize) {
    let mut crossings = (value / 100).unsigned_abs() as usize; // If the rotation is greater than 100, then we will cross it value / 100 times (integer division).
    let value = value % 100; // Now we can take care of the remainder of the last rotation

    if (dial > 0) && ((dial - value) <= 0) {
        crossings += 1;
    }
    dial = (dial - value).rem_euclid(100);
    (dial, crossings)
}
