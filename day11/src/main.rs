use std::{cell::RefCell, collections::HashMap, str::FromStr};

fn main() -> anyhow::Result<()> {
    let input = aoc::fetch_puzzle_input(11)?;
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

#[derive(Debug)]
struct ServerRack {
    devices: HashMap<String, Vec<String>>,
    ways: RefCell<usize>,
}
impl ServerRack {
    pub fn travel(&self, start_device: &str) {
        *self.ways.borrow_mut() = 0;
        self.travel_recurse(start_device);
    }
    fn travel_recurse(&self, device: &str) {
        if device == "out" {
            *self.ways.borrow_mut() += 1;
            return;
        }

        for next_device in self.devices.get(device).unwrap() {
            self.travel_recurse(next_device);
        }
    }
}

impl FromStr for ServerRack {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let devices = s
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut line = line.split(":");
                let it = line.by_ref();
                let device = it.next().unwrap();
                let peer_devices: Vec<String> = it
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .filter(|dev| !dev.is_empty())
                    .map(|dev| dev.to_string())
                    .collect();
                (device.to_string(), peer_devices)
            })
            .collect();

        Ok(Self {
            devices,
            ways: RefCell::new(0),
        })
    }
}

fn part1(input: &str) -> anyhow::Result<String> {
    let rack = ServerRack::from_str(input)?;
    rack.travel("you");
    Ok(rack.ways.borrow().to_string())
}
fn part2(input: &str) -> anyhow::Result<String> {
    let rack = ServerRack::from_str(input)?;
    println!("{rack:?}");
    rack.travel("svr");
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    // First problem where part2 has a different example !
    const INPUT2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "5");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT2).unwrap(), "");
    }
}
