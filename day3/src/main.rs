struct Batteries(Vec<u32>);
impl Batteries {
    /// Calculate the best joltage for a given set of batteries and the number of digits to include (nb).
    fn best_joltage(&self, nb: usize) -> u64 {
        // Now that we have joltages sorted according to their position, we can generate the finally joltage.
        self.joltage_recur(0, nb)
            .into_iter()
            .enumerate()
            .map(|(i, n)| 10_u64.pow(i as u32) * n)
            .sum()
    }

    fn joltage_recur(&self, base_index: usize, nb: usize) -> Vec<u64> {
        // tail call
        if nb == 0 {
            return vec![];
        }

        // Define the search set given the number of digits we still need to collect and the base index.
        let set_size = self.0.len() - (nb - 1) - base_index;

        // Get the greatest digit and its position
        let greatest = *self.0.iter().skip(base_index).take(set_size).max().unwrap();
        let pos = base_index
            + self
                .0
                .iter()
                .skip(base_index)
                .position(|n| *n == greatest)
                .unwrap();

        let mut others = self.joltage_recur(pos + 1, nb - 1);
        others.push(greatest as u64);
        others
    }
}

fn main() -> anyhow::Result<()> {
    let input = aoc::fetch_puzzle_input(3)?;
    println!("part1 ans -> {}", part1(&input)?);
    println!("part2 ans -> {}", part2(&input)?);
    Ok(())
}
fn parse_input(input: &str) -> anyhow::Result<Vec<Batteries>> {
    Ok(input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            Batteries(
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect())
}

fn part1(input: &str) -> anyhow::Result<String> {
    let total_joltage: u64 = parse_input(input)?
        .into_iter()
        .map(|batteries| batteries.best_joltage(2))
        .sum();
    Ok(total_joltage.to_string())
}
fn part2(input: &str) -> anyhow::Result<String> {
    let total_joltage: u64 = parse_input(input)?
        .into_iter()
        .map(|batteries| batteries.best_joltage(12))
        .sum();
    Ok(total_joltage.to_string())
}
