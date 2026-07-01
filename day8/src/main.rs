use std::str::FromStr;

use anyhow::bail;

fn main() -> anyhow::Result<()> {
    let input = aoc::fetch_puzzle_input(8)?;
    println!("Part 1: {}", part1(&input, 1000)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

fn parse_input(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                JunctionBox::from_str(line).ok()
            }
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str, nb_iter: usize) -> anyhow::Result<String> {
    let jboxes = parse_input(input);
    let ans = solve(jboxes, nb_iter)?;
    Ok(ans.to_string())
}
fn part2(input: &str) -> anyhow::Result<String> {
    let jboxes = parse_input(input);
    let ans = solve2(jboxes)?;
    Ok(ans.to_string())
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct JunctionBox {
    x: u32,
    y: u32,
    z: u32,
}
impl std::fmt::Display for JunctionBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{}), ", self.x, self.y, self.z)
    }
}

impl JunctionBox {
    fn euclid_distance(&self, other: &JunctionBox) -> f64 {
        let dx = (self.x as i64 - other.x as i64).pow(2);
        let dy = (self.y as i64 - other.y as i64).pow(2);
        let dz = (self.z as i64 - other.z as i64).pow(2);
        ((dx + dy + dz) as f64).sqrt()
    }
}

impl FromStr for JunctionBox {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ss = s.split(",");
        let x = ss.next().expect("X not present.").parse()?;
        let y = ss.next().expect("Y not present.").parse()?;
        let z = ss.next().expect("Z not present.").parse()?;
        Ok(Self { x, y, z })
    }
}

#[derive(Debug, Clone)]
struct Circuit(Vec<JunctionBox>);
impl Circuit {
    fn contains(&self, jbox: &JunctionBox) -> bool {
        self.0.contains(jbox)
    }
}
impl std::fmt::Display for Circuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for jbox in &self.0 {
            write!(f, "{}", jbox)?;
        }
        write!(f, "]\n")
    }
}

#[derive(Debug, Clone)]
struct Circuits(Vec<Circuit>);
impl std::fmt::Display for Circuits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for circuit in &self.0 {
            write!(f, "{}", circuit)?;
        }
        write!(f, "]")
    }
}
impl Circuits {
    fn new(jboxes: &[JunctionBox]) -> Self {
        Self(
            jboxes
                .iter()
                .map(|jboxes| Circuit(vec![jboxes.clone()]))
                .collect(),
        )
    }

    fn migrate(&mut self, jbox1: &JunctionBox, jbox2: &JunctionBox) {
        // find in which circuit jbox1 and jbox2 are
        let jbox1_pos = self.find_circuit(jbox1);
        let jbox2_pos = self.find_circuit(jbox2);

        if jbox1_pos == jbox2_pos {
            // they are already in the same circuit, nothing to do
            return;
        }

        // migrate jbox1 circuit to jbox2 circuit
        let jbox1_circuit = self.0[jbox1_pos].clone();
        self.0[jbox2_pos].0.extend(jbox1_circuit.0);

        // this needs to happen after the migration otherwise we would possibly have the wrong index for jbox2_pos
        let _jbox1_circuit = self.0.remove(jbox1_pos);
    }

    fn get_score(&self) -> anyhow::Result<usize> {
        let mut counts = self
            .0
            .iter()
            .map(|circuit| circuit.0.len())
            .collect::<Vec<_>>();
        if counts.len() < 3 {
            bail!("Not enough circuits to compute score.");
        }

        counts.sort();

        Ok(counts.iter().rev().take(3).product::<usize>())
    }

    fn find_circuit(&self, jbox: &JunctionBox) -> usize {
        // unwrap SAFETY: the junction box must be present otherwise we would have failed to find the closest pair of junction boxes
        self.0
            .iter()
            .position(|circuit| circuit.contains(jbox))
            .unwrap()
    }
}

fn sort_jbox_pairs_by_distance(jboxes: &[JunctionBox]) -> Vec<(&JunctionBox, &JunctionBox)> {
    let mut pairs = {
        let mut pairs = Vec::new();
        for i in 0..jboxes.len() - 1 {
            for j in i + 1..jboxes.len() {
                let jbox1 = &jboxes[i];
                let jbox2 = &jboxes[j];
                pairs.push((jbox1, jbox2));
            }
        }
        pairs
    };
    pairs.sort_by(|(jbox1a, jbox2a), (jbox1b, jbox2b)| {
        let dist_a = jbox1a.euclid_distance(jbox2a);
        let dist_b = jbox1b.euclid_distance(jbox2b);
        dist_a.partial_cmp(&dist_b).unwrap()
    });
    pairs
}

fn solve(inital_jboxes: Vec<JunctionBox>, nb_iter: usize) -> anyhow::Result<usize> {
    let jbox_pairs = sort_jbox_pairs_by_distance(&inital_jboxes);

    let mut circuits = Circuits::new(&inital_jboxes);

    for jbox_pair in jbox_pairs.into_iter().take(nb_iter) {
        let (jbox1, jbox2) = jbox_pair;
        circuits.migrate(jbox1, jbox2);
    }

    Ok(circuits.get_score()?)
}

fn solve2(inital_jboxes: Vec<JunctionBox>) -> anyhow::Result<usize> {
    let jbox_pairs = sort_jbox_pairs_by_distance(&inital_jboxes);

    let mut circuits = Circuits::new(&inital_jboxes);

    // migrate junction boxes until we only have 1 circuit left,
    // then return the last pair of junction boxes that made this condition true
    let find_last_pair = || {
        for jbox_pair in jbox_pairs.into_iter() {
            let (jbox1, jbox2) = jbox_pair;
            circuits.migrate(jbox1, jbox2);

            if circuits.0.len() == 1 {
                return Some((jbox1.clone(), jbox2.clone()));
            }
        }
        None
    };

    let (p1, p2) = find_last_pair().expect("failed..");
    let score = p1.x as usize * p2.x as usize;
    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT, 10).unwrap(), "40");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "25272");
    }
}
