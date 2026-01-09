use std::str::FromStr;

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}
impl FromStr for Op {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mul),
            _ => Err(anyhow::anyhow!("Failed to parse Op from {}", s)),
        }
    }
}

fn parse_input1(input: &str) -> anyhow::Result<(Vec<Vec<u64>>, Vec<Op>)> {
    // Number of lines before the operations line
    let nb_nums_line = input.split("\n").take_while(|s| !s.contains("+")).count();

    let mut lines = input.split('\n').filter(|line| !line.is_empty());

    // Extract and parse numbers
    let nums = lines
        .by_ref()
        .take(nb_nums_line)
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u64>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Transpose problems to column (1 problem per operation)
    let problems = (0..nums[0].len())
        .map(|col| (0..nums.len()).map(|row| nums[row][col]).collect())
        .collect();

    // Extract the operations
    let ops = lines
        .next()
        .ok_or(anyhow::anyhow!("No operations line found"))?
        .split_whitespace()
        .map(Op::from_str)
        .collect::<Result<Vec<Op>, _>>()?;

    Ok((problems, ops))
}
fn parse_input2(input: &str) -> anyhow::Result<(Vec<Vec<u64>>, Vec<Op>)> {
    // Number of lines before the operations line
    let nb_nums_line = input.split("\n").take_while(|s| !s.contains("+")).count();

    let mut lines = input.split('\n').filter(|line| !line.is_empty());

    // Collect first 4 lines as a Vec<Vec<char>>
    let nums: Vec<Vec<char>> = lines
        .by_ref()
        .take(nb_nums_line)
        .map(|line| line.chars().collect())
        .collect();

    // Transpose and clean each column of chars to form String
    let nums2: Vec<String> = (0..nums[0].len())
        .map(|col| {
            (0..nums.len())
                .map(|row| nums[row][col])
                .filter(|c| !c.is_whitespace())
                .collect()
        })
        .collect();

    // Group by non-empty strings, splitting on empty as separators
    let problems: Vec<Vec<u64>> = nums2
        .split(|s| s.is_empty())
        .filter(|group| !group.is_empty())
        .map(|group| {
            group
                .iter()
                .map(|s| s.parse::<u64>())
                .collect::<Result<Vec<u64>, _>>()
        })
        .collect::<Result<Vec<Vec<u64>>, _>>()?;

    // Extract the operations
    let ops = lines
        .next()
        .ok_or(anyhow::anyhow!("No operations line found"))?
        .split_whitespace()
        .map(Op::from_str)
        .collect::<Result<Vec<Op>, _>>()?;

    Ok((problems, ops))
}

fn main() -> anyhow::Result<()> {
    let input = aoc::fetch_puzzle_input(6)?;
    println!("part1 ans -> {}", part1(&input)?);
    println!("part2 ans -> {}", part2(&input)?);
    Ok(())
}

fn solve(problems: Vec<Vec<u64>>, ops: Vec<Op>) -> u64 {
    problems
        .iter()
        .zip(ops.iter())
        .map(|(problem, op)| {
            let init = match op {
                Op::Add => 0,
                Op::Mul => 1,
            };
            problem.iter().fold(init, |acc, &num| match op {
                Op::Add => acc + num,
                Op::Mul => acc * num,
            })
        })
        .sum()
}

fn part1(input: &str) -> anyhow::Result<String> {
    let (problems, ops) = parse_input1(input)?;
    Ok(solve(problems, ops).to_string())
}
fn part2(input: &str) -> anyhow::Result<String> {
    let (problems, ops) = parse_input2(input)?;
    Ok(solve(problems, ops).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "4277556");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "3263827");
    }
}
