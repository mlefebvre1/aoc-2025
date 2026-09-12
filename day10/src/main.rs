use std::str::FromStr;

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = aoc::fetch_puzzle_input(10)?;
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Indicator(Vec<bool>);
impl FromStr for Indicator {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let indicators = s
            .chars()
            .map(|c| match c {
                '.' => false,
                '#' => true,
                _ => panic!("unexpected light indicator {c}"),
            })
            .collect();
        Ok(Self(indicators))
    }
}
impl Indicator {
    pub fn new(nb_lights: usize) -> Self {
        Self(vec![false; nb_lights])
    }
}

#[derive(Debug)]
struct Button(Vec<usize>);
impl FromStr for Button {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //first remove leading and ending parenthesis
        let s = s.replace('(', "").replace(')', "");
        let indexes = s.split(',').map(|i| i.parse::<usize>().unwrap()).collect();
        Ok(Button(indexes))
    }
}

#[derive(Debug)]
struct Joltage(Vec<usize>);
impl FromStr for Joltage {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //first remove leading and ending braces
        let s = s.replace('{', "").replace('}', "");
        let indexes = s.split(',').map(|i| i.parse::<usize>().unwrap()).collect();
        Ok(Joltage(indexes))
    }
}

#[derive(Debug)]
struct Line {
    indicator: Indicator,
    buttons: Vec<Button>,
    joltage: Joltage,
}
impl FromStr for Line {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut schar = s.chars();
        let sref = schar.by_ref();

        let indicator_str = sref
            .take_while(|c| *c != ']')
            .collect::<String>()
            .replace('[', "");
        let indicator = Indicator::from_str(&indicator_str)?;

        let buttons_str = sref.take_while(|c| *c != '{').collect::<String>();
        let buttons_str = buttons_str.trim();
        let buttons = buttons_str
            .split_whitespace()
            .map(Button::from_str)
            .collect::<anyhow::Result<_>>();
        let buttons = buttons?;

        let joltage = Joltage::from_str(sref.collect::<String>().as_str())?;

        Ok(Self {
            indicator,
            buttons,
            joltage,
        })
    }
}

fn part1(input: &str) -> anyhow::Result<String> {
    // Brute force solution, we can try all combinations of the buttons and see which one gives us a correct indicator. Then search for the smallest one.
    // The assumption here, is that the optimal solution will never include a sequence where we have to push twice the same button.
    let ans = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let line = Line::from_str(line).unwrap();
            // Test for each combination size 1 to the number of buttons
            (1..=line.buttons.len())
                .filter_map(|r| {
                    line.buttons
                        .iter()
                        .combinations(r) // test each combinations of buttons of size r
                        .find_map(|p| {
                            let mut cur_indicator = Indicator::new(line.indicator.0.len());
                            for (nb_pressed, button) in p.iter().enumerate() {
                                // foreach button in the combination, toggle the lights and check if we have the correct indicator
                                for index in button.0.iter() {
                                    cur_indicator.0[*index] = !cur_indicator.0[*index];
                                }
                                if cur_indicator.0 == line.indicator.0 {
                                    return Some(nb_pressed + 1); // We found a combination of buttons that gives us the correct indicator, we return the number of buttons pressed
                                }
                            }
                            // if we reach here, it means that this combination of buttons does not give us the correct indicator, so we return a large number to indicate that this combination is not valid
                            None
                        })
                })
                .min() // simply find the smallest number of buttons that gives us the correct indicator
                .unwrap()
        })
        .sum::<usize>();
    Ok(ans.to_string())
}
fn part2(input: &str) -> anyhow::Result<String> {
    let ans = "";
    Ok(ans.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "7");
    }
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "TBD");
    }
}
