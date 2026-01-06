#[derive(Debug, Eq, PartialEq)]
enum Point {
    Nothing,
    RollOfPaper,
}
struct Loc(isize, isize);

#[derive(Debug)]
struct Grid(Vec<Vec<Point>>);

impl Grid {
    pub fn solve_part1(&self) -> u64 {
        self.get_removable().len() as u64
    }
    pub fn solve_part2(&mut self) -> u64 {
        let mut ans = 0;
        loop {
            let removable = self.get_removable();
            if removable.is_empty() {
                // If no new removable rolls of paper, we are done
                break;
            }

            ans += removable.len() as u64;

            // Remove all the removable rolls of paper
            for loc in removable {
                if let Some(p) = self.get_mut(&loc) {
                    *p = Point::Nothing;
                }
            }
        }
        ans
    }

    /// Get all the removable rolls of paper
    pub fn get_removable(&self) -> Vec<Loc> {
        const MAX_ROLLS: u64 = 4;
        let (xlen, ylen) = self.shape();
        let mut removable = vec![];

        for y in 0..ylen {
            for x in 0..xlen {
                let loc = Loc(x as isize, y as isize);
                if self.get(&loc) == Some(&Point::RollOfPaper)
                    && self.adjacent_rolls(&loc) < MAX_ROLLS
                {
                    removable.push(loc);
                }
            }
        }

        removable
    }

    /// Get a specific point
    fn get(&self, loc: &Loc) -> Option<&Point> {
        if loc.0 < 0 || loc.1 < 0 {
            return None;
        }
        self.0.get(loc.1 as usize)?.get(loc.0 as usize)
    }

    /// Get a specific point mutably
    fn get_mut(&mut self, loc: &Loc) -> Option<&mut Point> {
        if loc.0 < 0 || loc.1 < 0 {
            return None;
        }
        self.0.get_mut(loc.1 as usize)?.get_mut(loc.0 as usize)
    }

    /// Return (x,y) size of the grid
    fn shape(&self) -> (usize, usize) {
        (self.0[0].len(), self.0.len())
    }

    /// Calculate the number of rolls of paper adjacent to a given location
    fn adjacent_rolls(&self, loc: &Loc) -> u64 {
        let mut nb_roll_of_papers = 0;
        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 {
                    // Skip self
                    continue;
                }
                if let Some(Point::RollOfPaper) = self.get(&Loc(loc.0 + x, loc.1 + y)) {
                    nb_roll_of_papers += 1;
                }
            }
        }
        nb_roll_of_papers
    }
}

impl TryFrom<char> for Point {
    type Error = anyhow::Error;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Point::Nothing),
            '@' => Ok(Point::RollOfPaper),
            _ => Err(anyhow::anyhow!("Invalid point character")),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = aoc::fetch_puzzle_input(4)?;
    println!("part1 ans -> {}", part1(&input)?);
    println!("part2 ans -> {}", part2(&input)?);
    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<Grid> {
    let inner: anyhow::Result<Vec<Vec<Point>>> = input
        .split('\n')
        .map(|line| line.chars().map(Point::try_from).collect())
        .collect();

    Ok(Grid(inner?))
}

fn part1(input: &str) -> anyhow::Result<String> {
    let grid = parse_input(input)?;
    let ans = grid.solve_part1();
    Ok(ans.to_string())
}

fn part2(input: &str) -> anyhow::Result<String> {
    let mut grid = parse_input(input)?;
    let ans = grid.solve_part2();
    Ok(ans.to_string())
}
