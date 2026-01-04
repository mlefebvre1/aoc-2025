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
            .ok_or(anyhow::anyhow!("should have a first id"))?
            .parse()?;
        let end: usize = ss
            .next()
            .ok_or(anyhow::anyhow!("should have a second id"))?
            .parse()?;
        Ok(Range { start, end })
    }
}
impl Range {
    fn find_invalids(&self) -> impl Iterator<Item = usize> {
        (self.start..=self.end).filter(|x| {
            let digits = x.to_string();
            digits.len() % 2 == 0
                && digits[0..digits.len() / 2] == digits[digits.len() / 2..digits.len()]
        })
    }
}
// 0..3. 3..6
//101 101

fn main() -> anyhow::Result<()> {
    let input = aoc::fetch_puzzle_input(2)?;
    //     let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
    // 1698522-1698528,446443-446449,38593856-38593862,565653-565659,
    // 824824821-824824827,2121212118-2121212124";
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
    let nb_invalids: usize = ranges.iter().flat_map(|range| range.find_invalids()).sum();
    Ok(nb_invalids.to_string())
}
fn part2(input: &str) -> anyhow::Result<String> {
    Ok("".to_string())
}
