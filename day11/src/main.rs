use std::{cell::RefCell, collections::HashMap, str::FromStr};

fn main() -> anyhow::Result<()> {
    let input = aoc::fetch_puzzle_input(11)?;
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

#[derive(Debug, Default)]
struct State {
    visited: bool,
    reach: bool,
}

#[derive(Debug)]
struct ServerRack {
    devices: HashMap<String, Vec<String>>,
    ways: RefCell<usize>,
    states: RefCell<HashMap<String, State>>,
}
impl ServerRack {
    pub fn reverse_travel(&self, start_device: &str, end_device: &str) {
        *self.ways.borrow_mut() = 0;
        self.reverse_travel_recurse(start_device, end_device);
    }
    fn reverse_travel_recurse(&self, device: &str, end_device: &str) {
        if device == end_device {
            *self.ways.borrow_mut() += 1;
            return;
        }
        // Find all place where device is used
        let devices = self.devices.iter().filter_map(|(key, value)| {
            if value.iter().find(|dev| *dev == device).is_some() {
                return Some(key);
            }
            None
        });
        for next_device in devices {
            self.reverse_travel_recurse(next_device, end_device);
        }
    }

    pub fn midway_travel(&self, start_device: &str, end_device: &str) {
        *self.ways.borrow_mut() = 0;
        self.reset_states();
        self.midway_travel_recurse(start_device, end_device);
    }
    fn midway_travel_recurse(&self, device: &str, end_device: &str) -> bool {
        if device == end_device {
            *self.ways.borrow_mut() += 1;
            return true;
        }
        if device == "out" {
            return false;
        }

        let mut reached = false;

        for next_device in self.devices.get(device).unwrap() {
            if self.visited(next_device) && !self.reached(next_device) {
                continue;
            }

            if self.midway_travel_recurse(next_device, end_device) {
                reached = true;
            }
        }

        if reached {
            self.set_reach(device);
        }
        self.set_visited(device);
        reached
    }

    fn set_reach(&self, device: &str) {
        self.states.borrow_mut().get_mut(device).unwrap().reach = true;
    }
    fn set_visited(&self, device: &str) {
        self.states.borrow_mut().get_mut(device).unwrap().visited = true;
    }
    fn reset_states(&self) {
        self.states.borrow_mut().iter_mut().for_each(|(_k, v)| {
            v.visited = false;
            v.reach = false;
        });
    }

    fn visited(&self, device: &str) -> bool {
        self.states.borrow().get(device).unwrap().visited
    }
    fn reached(&self, device: &str) -> bool {
        self.states.borrow().get(device).unwrap().visited
            && self.states.borrow().get(device).unwrap().reach
    }

    pub fn travel(&self, start_device: &str, end_device: &str) {
        *self.ways.borrow_mut() = 0;
        self.travel_recurse(start_device, end_device);
    }
    fn travel_recurse(&self, device: &str, end_device: &str) {
        if device == end_device {
            *self.ways.borrow_mut() += 1;
            return;
        }
        if device == "out" {
            return;
        }

        for next_device in self.devices.get(device).unwrap() {
            self.travel_recurse(next_device, end_device)
        }
    }
}

impl FromStr for ServerRack {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let devices: HashMap<String, Vec<String>> = s
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

        let mut states: HashMap<String, State> = devices
            .keys()
            .map(|k| (k.to_string(), State::default()))
            .collect();
        states.insert("out".to_string(), State::default());

        Ok(Self {
            devices,
            ways: RefCell::new(0),
            states: RefCell::new(states),
        })
    }
}

fn part1(input: &str) -> anyhow::Result<String> {
    let rack = ServerRack::from_str(input)?;
    rack.travel("you", "out");
    Ok(rack.ways.borrow().to_string())
}
fn part2(input: &str) -> anyhow::Result<String> {
    let rack = ServerRack::from_str(input)?;
    let mut nb_ways = 1;

    // It was observed that there's no path from dac to fft
    // So we can compute svr -> fft * fft -> dac * dac -> out
    rack.reverse_travel("fft", "svr");
    nb_ways *= *rack.ways.borrow();
    rack.midway_travel("fft", "dac");
    nb_ways *= *rack.ways.borrow();
    rack.travel("dac", "out");
    nb_ways *= *rack.ways.borrow();

    Ok(nb_ways.to_string())
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
        assert_eq!(part2(INPUT2).unwrap(), "2");
    }
}

/*
 *
svr: aaa bbb
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
hhh: out



svr: 0
aaa: 0
fft: 0
bbb: 0
tty: 0
ccc: 0
ddd: 0
hub: 0
eee: 0
dac: 0
fff: 0
ggg: 0
hhh: 0



 *
 */
