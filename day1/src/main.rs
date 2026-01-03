use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let input = aoc::fetch_puzzle_input(1)?;
    //     let input = r"L68
    // L30
    // R48
    // L5
    // R60
    // L55
    // L1
    // L99
    // R14
    // L82";
    println!("part1 ans -> {}", part1(&input)?);
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
    let mut x = 50;
    let mut total = 0;
    for rot in rotations {
        let x2 = match rot {
            Rotation::Left(n) => x - n,
            Rotation::Right(n) => x + n,
        };
        x = x2 % 100;
        if x == 0 {
            total += 1;
            continue;
        }

        if x2 < 0 {
            x += 100;
        }
    }

    Ok(total.to_string())
}

#[test]
fn test() {
    println!("{}", -700i32 % 100)
}
