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

fn parse_input(input: &str) -> anyhow::Result<(Vec<Vec<u64>>, Vec<Op>)> {
    let mut lines = input.split('\n').filter(|line| !line.is_empty());

    let nums: Vec<Vec<u64>> = lines
        .by_ref()
        .take(4)
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    // Invert dimension
    let mut problems = vec![];
    for col in 0..nums[0].len() {
        let mut p = vec![];
        for row in 0..nums.len() {
            p.push(nums[row][col]);
        }
        problems.push(p);
    }

    let ops: anyhow::Result<Vec<Op>> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(Op::from_str)
        .collect();

    Ok((problems, ops?))
}
fn parse_input2(input: &str) -> anyhow::Result<(Vec<Vec<u64>>, Vec<Op>)> {
    let mut lines = input.split('\n').filter(|line| !line.is_empty());

    let nums: Vec<Vec<char>> = lines
        .by_ref()
        .take(4)
        .map(|line| line.chars().collect())
        .collect();

    let mut nums2 = vec![];
    for col in 0..nums[0].len() {
        let mut num = vec![];
        for row in 0..nums.len() {
            num.push(nums[row][col]);
        }

        nums2.push(
            num.iter()
                .filter(|c| !c.is_whitespace())
                .collect::<String>(),
        );
    }

    let mut problems = vec![];
    let mut problem = vec![];
    for num in nums2 {
        if num.is_empty() {
            problems.push(problem.clone());
            problem.clear();
            continue;
        }
        problem.push(num.parse::<u64>()?);
    }
    problems.push(problem.clone());

    let ops: anyhow::Result<Vec<Op>> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(Op::from_str)
        .collect();

    Ok((problems, ops?))
}

fn main() -> anyhow::Result<()> {
    let input = aoc::fetch_puzzle_input(6)?;
    let input2 = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
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
    let (problems, ops) = parse_input(input)?;
    Ok(solve(problems, ops).to_string())
}
fn part2(input: &str) -> anyhow::Result<String> {
    let (problems, ops) = parse_input2(input)?;
    Ok(solve(problems, ops).to_string())
}
