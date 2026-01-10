use std::{fmt::Display, str::FromStr};

#[derive(Debug, Eq, PartialEq)]
enum Loc {
    Start,
    Splitter,
    Space,
    Beam,
}

impl FromStr for Loc {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(Loc::Start),
            "|" => Ok(Loc::Beam),
            "." => Ok(Loc::Space),
            "^" => Ok(Loc::Splitter),
            _ => Err(anyhow::anyhow!("Failed to parse Loc from {}", s)),
        }
    }
}

impl Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Loc::Start => 'S',
            Loc::Beam => '|',
            Loc::Space => '.',
            Loc::Splitter => '^',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug)]
struct Grid {
    nb_splits: usize,
    inner: Vec<Vec<Loc>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.inner {
            for loc in row {
                write!(f, "{}", loc)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn new(inner: Vec<Vec<Loc>>) -> Self {
        Grid {
            nb_splits: 0,
            inner,
        }
    }

    pub fn run(&mut self) -> anyhow::Result<usize> {
        let pos = self.find_start().ok_or(anyhow::anyhow!("No start found"))?;
        self.progress(pos);
        Ok(self.nb_splits)
    }

    fn progress(&mut self, initial_pos: (isize, isize)) {
        let mut pos = initial_pos;

        match self.get_mut(&pos) {
            None => {
                // Should not happen, but just in case..
                return;
            }
            Some(loc) => {
                *loc = Loc::Beam;
            }
        };

        loop {
            let next_pos = (pos.0, pos.1 + 1);

            match self.get_mut(&next_pos) {
                None => {
                    println!("Got here??");

                    return;
                }
                Some(loc) => match loc {
                    Loc::Space => {
                        // Continue down
                        *loc = Loc::Beam;
                        pos = next_pos;
                    }
                    Loc::Splitter => {
                        self.nb_splits += 1;
                        self.progress((next_pos.0 - 1, next_pos.1)); //left
                        self.progress((next_pos.0 + 1, next_pos.1)); //right
                        return;
                    }
                    _ => {
                        return;
                    }
                },
            }
        }
    }

    fn find_start(&self) -> Option<(isize, isize)> {
        for (y, row) in self.inner.iter().enumerate() {
            for (x, loc) in row.iter().enumerate() {
                if let Loc::Start = loc {
                    return Some((x as isize, y as isize));
                }
            }
        }
        None
    }

    /// Get a specific point mutably
    fn get_mut(&mut self, loc: &(isize, isize)) -> Option<&mut Loc> {
        if loc.0 < 0 || loc.1 < 0 {
            return None;
        }
        self.inner.get_mut(loc.1 as usize)?.get_mut(loc.0 as usize)
    }
}

fn main() -> anyhow::Result<()> {
    let input = aoc::fetch_puzzle_input(7)?;
    println!("part1 ans -> {}", part1(&input)?);
    println!("part2 ans -> {}", part2(&input)?);
    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<Grid> {
    let grid = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| Loc::from_str(&c.to_string()))
                .collect::<Result<Vec<Loc>, _>>()
        })
        .collect::<Result<Vec<Vec<Loc>>, _>>()?;
    Ok(Grid::new(grid))
}

fn part1(input: &str) -> anyhow::Result<String> {
    let mut grid = parse_input(input)?;
    let ans = grid.run()?;

    Ok(ans.to_string())
}
fn part2(input: &str) -> anyhow::Result<String> {
    let grid = parse_input(input)?;
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "21");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "40");
    }
}
