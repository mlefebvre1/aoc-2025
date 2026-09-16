use std::{str::FromStr, usize};

use highs::{HighsModelStatus, RowProblem, Sense};
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

#[derive(Debug, PartialEq, Eq)]
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
impl Joltage {
    pub fn new(nb_joltage: usize) -> Self {
        Self(vec![0; nb_joltage])
    }
    pub fn press(&mut self, button: &Button, count: usize) {
        for &index in &button.0 {
            self.0[index] += count;
        }
    }
    pub fn unpress(&mut self, button: &Button, count: usize) {
        for &index in &button.0 {
            self.0[index] -= count;
        }
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
    let ans = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| {
            let line = Line::from_str(line).unwrap();
            let (matrix_a, matrix_b) = build_matrices(&line);
            solve_matrices(matrix_a, matrix_b)
        })
        .sum::<usize>();

    Ok(ans.to_string())
}

fn build_matrices(line: &Line) -> (Vec<Vec<f64>>, Vec<f64>) {
    // (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    // a*X3 + b*(X1 + X3) + c*X2 + d*(X2+X3) + e*(X0+X2) + f*(X0+X1) = 3*X0 + 5*X1 + 4*X2 + 7*X3
    //
    // e + f = 3
    // b + f = 5
    // c + d + e = 4
    // a + b + d = 7
    //
    //  [0., 0., 0., 0., 1., 1.],
    //  [0., 1., 0., 0., 0., 1.],
    //  [0., 0., 1., 1., 1., 0.],
    //  [1., 1., 0., 1., 0., 0.],
    let mut matrix_a = vec![vec![0f64; line.buttons.len()]; line.joltage.0.len()];
    let matrix_b = line.joltage.0.iter().map(|&x| x as f64).collect::<Vec<_>>();

    for (counter_target, row) in matrix_a.iter_mut().enumerate() {
        for (button_col, button) in line.buttons.iter().enumerate() {
            for counter in button.0.iter() {
                if *counter == counter_target {
                    row[button_col] = 1.0;
                }
            }
        }
    }

    (matrix_a, matrix_b)
}
fn solve_matrices(matrix_a: Vec<Vec<f64>>, matrix_b: Vec<f64>) -> usize {
    // just use a linear equation solver.. way too much work to implement my own..
    let matrix_c = vec![1.0; matrix_a[0].len()];
    let mut pb = RowProblem::default();
    let cols: Vec<_> = matrix_c
        .iter()
        .map(|&cost| pb.add_integer_column(cost, 0..))
        .collect();
    for (i, row) in matrix_a.iter().enumerate() {
        let terms = cols
            .iter()
            .copied()
            .zip(row.iter().copied())
            .filter(|(_, v)| *v != 0.0);
        pb.add_row(matrix_b[i]..=matrix_b[i], terms);
    }

    let solved = pb.optimise(Sense::Minimise).solve();
    if solved.status() != HighsModelStatus::Optimal {
        panic!(
            "Failed to find optimal solution status: {:?}",
            solved.status()
        );
    }

    solved.objective_value() as usize
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
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "33");
    }
}
