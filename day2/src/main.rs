use std::str::FromStr;

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}
impl FromStr for Range {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace("\n", "");
        let mut ss = s.split("-");
        let start: usize = ss
            .next()
            .ok_or(anyhow::anyhow!("should have a start id"))?
            .parse()?;
        let end: usize = ss
            .next()
            .ok_or(anyhow::anyhow!("should have an end id"))?
            .parse()?;
        Ok(Range { start, end })
    }
}
impl Range {
    fn find_invalids_part1(&self) -> impl Iterator<Item = usize> {
        (self.start..=self.end).filter(|x| {
            let digits = x.to_string();
            //simply check that the lower-half of the number equals the upper-half of the number
            digits[0..digits.len() / 2] == digits[digits.len() / 2..digits.len()]
        })
    }
    fn find_invalids_part2(&self) -> impl Iterator<Item = usize> {
        (self.start..=self.end).filter(|x| {
            let digits = x.to_string();
            for n in 1..=digits.len() / 2 {
                /* We can't only check the lower and upper half only anymore. Instead we need to check from 1 digit to digits.len() / 2 possibilities.
                Ex: for the number 565656 we would check the following possibilities:
                n = 1 -> 5 6 5 6 5 6        token = 5       invalid = false
                n = 2 -> 56 56 56           token = 56      invalid = true
                n = 3 -> 565 565            token = 565     invalid = false <- would not be checked because it would return from i = 2
                */
                let token = &digits[0..n];
                // if all the chunks equals the token, then we have found an invalid id, we can return true!
                if (0..digits.len())
                    .step_by(n)
                    .map(|i| &digits[i..std::cmp::min(i + n, digits.len())])
                    .all(|chunk| chunk == token)
                {
                    return true;
                }
            }
            // None of the possibilities worked, so we don't have an invalid id
            false
        })
    }
}

fn main() -> anyhow::Result<()> {
    let input = aoc::fetch_puzzle_input(2)?;
    println!("part1 ans -> {}", part1(&input)?);
    println!("part2 ans -> {}", part2(&input)?);
    Ok(())
}
fn parse_input(input: &str) -> anyhow::Result<Vec<Range>> {
    input
        .split(',')
        .filter(|line| !line.is_empty())
        .map(Range::from_str)
        .collect()
}
fn part1(input: &str) -> anyhow::Result<String> {
    let ranges = parse_input(input)?;
    let nb_invalids: usize = ranges
        .iter()
        .flat_map(|range| range.find_invalids_part1())
        .sum();
    Ok(nb_invalids.to_string())
}
fn part2(input: &str) -> anyhow::Result<String> {
    let ranges = parse_input(input)?;
    let nb_invalids: usize = ranges
        .iter()
        .flat_map(|range| range.find_invalids_part2())
        .sum();
    Ok(nb_invalids.to_string())
}
