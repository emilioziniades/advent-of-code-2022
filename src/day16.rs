use std::{
    collections::{HashMap, HashSet},
    fs,
};

use crate::pathfind::{self, recreate_all_paths, PathAlgo};

const A: u16 = 65;
const MAX_MINUTES: u16 = 30;
const OPEN_TIME: u16 = 1;

/*
 * 1. Find shortest paths between all valves
 * 2. At every time step, find the benefit of all actions. Actions include:
 *      - opening current valve
 *      - travelling to another valve
 *    Pick the action that maximizes benefit
 */

type FlowRates = HashMap<u16, u16>;

type Graph = HashMap<u16, Vec<u16>>;

impl pathfind::Graph<u16> for Graph {
    fn neighbours(&self, current: u16) -> &Vec<u16> {
        self.get(&current).unwrap()
    }

    fn nodes(&self) -> Vec<u16> {
        self.keys().cloned().collect()
    }

    fn cost(&self, _current: &u16, _next: &u16) -> usize {
        // all paths cost the same
        1
    }
}

pub fn maximize_pressure_release(file: &str) -> usize {
    let (graph, flow_rates) = parse_input(file);
    // println!("{graph:#?}");
    // println!("{flow_rates:#?}");

    let mut current_valve = A;
    let mut minutes = 0;
    let mut pressure_release = 0;

    let mut open_valves = HashSet::new();

    loop {
        if minutes >= MAX_MINUTES {
            break;
        }

        let paths = recreate_all_paths(&graph, current_valve, PathAlgo::Djikstra);
        // println!("{paths:#?}");

        if let Some((next_valve, benefit, time_elapsed)) = paths
            .into_iter()
            .filter(|(target, _path)| !open_valves.contains(target))
            .filter(|(target, _path)| flow_rates.get(target).unwrap() != &0)
            .map(|(target, path)| {
                let distance = path.len() as u16 + OPEN_TIME;
                let time_left = MAX_MINUTES - minutes - distance;
                let flow_rate = flow_rates.get(&target).unwrap();
                let benefit = time_left * flow_rate;

                (target, benefit, distance)
            })
            .max_by_key(|(_target, benefit, _distance)| *benefit)
        {
            println!(
                "from {} to {} benefit: {benefit} travel time: {time_elapsed}, time: {minutes}",
                char::from(current_valve as u8),
                char::from(next_valve as u8)
            );

            open_valves.insert(next_valve);
            current_valve = next_valve;
            minutes += time_elapsed;
            pressure_release += benefit;
        } else {
            break;
        }
    }

    pressure_release.into()
}

fn parse_input(file: &str) -> (Graph, FlowRates) {
    let mut graph: Graph = HashMap::new();
    let mut flow_rates: FlowRates = HashMap::new();

    let lines: Vec<(u16, u16, Vec<u16>)> = fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(|line| {
            let line = line
                .replace("Valve ", "")
                .replace("has flow rate=", "")
                .replace("; tunnels lead to valves", "")
                .replace("; tunnel leads to valve", "")
                .replace(", ", ",");
            let line: Vec<&str> = line.split_whitespace().collect();

            (
                line[0].as_bytes()[0] as u16,
                line[1].clone().parse().unwrap(),
                line[2]
                    .split(",")
                    .map(|valve| valve.as_bytes()[0] as u16)
                    .collect(),
            )
        })
        .collect();

    for (valve, flow_rate, neighbours) in lines {
        graph.insert(valve.clone(), neighbours);
        flow_rates.insert(valve.clone(), flow_rate);
    }

    (graph, flow_rates)
}

#[cfg(test)]
mod tests {
    use crate::{day16, fetch_input};

    #[test]
    #[ignore = "to revisit"]
    fn maximize_pressure_release() {
        fetch_input(16);

        let tests = vec![
            ("example/day16.txt", 1651), /*, ("input/day15.txt", 2_000_000)*/
        ];

        for test in tests {
            let (file, want) = test;
            let got = day16::maximize_pressure_release(file);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }
}
