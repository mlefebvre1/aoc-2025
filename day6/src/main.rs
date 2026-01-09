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
    // println!("input:\n{}", input);
    let mut lines = input.split('\n').filter(|line| !line.is_empty());

    let nums: Vec<Vec<u64>> = lines
        .by_ref()
        .take(4)
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    // Invert dimension
    let mut nums2 = vec![];
    for j in 0..nums[0].len() {
        let mut p = vec![];
        for i in 0..nums.len() {
            p.push(nums[i][j]);
        }
        nums2.push(p);
    }

    let ops: anyhow::Result<Vec<Op>> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(Op::from_str)
        .collect();

    println!("nums: {:?}, ops: {:?}", nums, ops);

    Ok((nums2, ops?))
}
fn parse_input2(input: &str) -> anyhow::Result<(Vec<Vec<u64>>, Vec<Op>)> {
    Ok((vec![], vec![]))
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

fn solve(nums: Vec<Vec<u64>>, ops: Vec<Op>) -> u64 {
    nums.iter()
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
    let (nums, ops) = parse_input(input)?;
    Ok(solve(nums, ops).to_string())
}
fn part2(input: &str) -> anyhow::Result<String> {
    Ok("0".to_string())
}
