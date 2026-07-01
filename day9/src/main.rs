use std::str::FromStr;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl FromStr for Point {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ss = s.split(",");
        let x = ss.next().expect("X not present.").parse()?;
        let y = ss.next().expect("Y not present.").parse()?;
        Ok(Self { x, y })
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Point>> {
    let points = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.parse::<Point>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(points)
}

fn main() -> anyhow::Result<()> {
    let input = aoc::fetch_puzzle_input(9)?;
    println!("Part 1: {}", part1(&input)?);
    // println!("Part 2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<String> {
    let points = &parse_input(input)?;

    let ans = points
        .iter()
        .flat_map(|p1| {
            points.iter().map(move |p2| {
                let area = ((p2.x - p1.x).abs() + 1) * ((p2.y - p1.y).abs() + 1);
                // println!("{:?}'s area: {}", (p1, p2), area);
                area
            })
        })
        .max()
        .expect("failed..");

    Ok(ans.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "50");
    }
}
