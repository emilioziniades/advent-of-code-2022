/// massive shout out to [this solution](https://github.com/jonathanpaulson/AdventOfCode/blob/master/2022/16.cc).
/// I had tried the naive approach of exploring every possible route, with some optimizations like pruning the graph
/// for valves we will never open. But I wasn't aware of DP (dynamic programming), in which you use a cache to avoid
/// recalculating the same state more than once. This allows you to break up a problem into smaller subproblems. The
/// key insight which I got from [this video](https://www.youtube.com/watch?v=DgqkVDr1WX8&t=585s) is that when the
/// possible number of states is much smaller than the possible number of combinations, then DP makes sense because
/// you avoid doing extra work. This is how you might analyse this situation:
/// Number of combinations = 3^30 = 205 trillion
///      - approximately 3 choices at every time step
///      - 30 time steps
/// Possible states = 2^15 * 30 * 60 = 58 million
///      - 15 non-zero valves that can be open or closed
///      - 30 possible time periods it could be
///      - 60 possible positions you could be in
///  So although there are a lot of possible routes through the graph, a lot of them will be
///  repeated, so we can do DP!
use std::{collections::HashMap, fs};

const MAX_MINUTES: isize = 30;
const MAX_MINUTES_WITH_ELEPHANT: isize = 26;

#[derive(Debug)]
struct Volcano {
    flow_rates: HashMap<isize, isize>,
    graph: HashMap<isize, HashMap<isize, isize>>,
    start_id: isize,
}

impl Volcano {
    fn from_file(filename: &str) -> Self {
        let mut graph = HashMap::new();
        let mut flow_rates = HashMap::new();

        let input = fs::read_to_string(filename).unwrap();

        let valve_name_to_id: HashMap<String, isize> = input
            .lines()
            .enumerate()
            .map(|(id, line)| {
                let id: isize = id.try_into().unwrap();
                let line = line.replace("Valve ", "");
                let valve_name = line.split_whitespace().next().unwrap().to_string();
                (valve_name, id)
            })
            .collect();

        let lines: Vec<(isize, isize, HashMap<isize, isize>)> = fs::read_to_string(filename)
            .unwrap()
            .lines()
            .enumerate()
            .map(|(id, line)| {
                let id: isize = id.try_into().unwrap();
                let line = line
                    .replace("Valve ", "")
                    .replace("has flow rate=", "")
                    .replace("; tunnels lead to valves", "")
                    .replace("; tunnel leads to valve", "")
                    .replace(", ", ",");

                let mut line = line.split_whitespace();

                line.next().unwrap();

                (
                    id,
                    line.next().unwrap().parse().unwrap(),
                    line.next()
                        .unwrap()
                        .split(',')
                        .map(|valve| valve_name_to_id.get(valve).unwrap())
                        .map(|valve| (*valve, 1)) // to be pruned later
                        .collect(),
                )
            })
            .collect();

        for (valve, flow_rate, neighbours) in lines {
            graph.insert(valve, neighbours);
            flow_rates.insert(valve, flow_rate);
        }

        Self {
            graph,
            flow_rates,
            start_id: *valve_name_to_id.get("AA").unwrap(),
        }
    }

    fn pruned(mut self) -> Self {
        let valves_to_prune: Vec<&isize> = self
            .flow_rates
            .iter()
            .filter_map(|(valve, flow_rate)| {
                if valve == &self.start_id || flow_rate > &0 {
                    None
                } else {
                    Some(valve)
                }
            })
            .collect();

        for valve in valves_to_prune {
            let neighbours = self.graph.remove(valve).unwrap();
            for (neighbour, distance_to_prunee) in neighbours.iter() {
                let others: Vec<&isize> = neighbours.keys().filter(|x| x != &neighbour).collect();
                for other in others {
                    let other_distance_to_prunee =
                        self.graph.get_mut(other).unwrap().remove(valve).unwrap();
                    self.graph
                        .get_mut(neighbour)
                        .unwrap()
                        .insert(*other, distance_to_prunee + other_distance_to_prunee);
                }
            }
        }
        self
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    valve: isize,
    opened_valves: isize,
    time_left: isize,
    with_elephant: bool,
}

// recursive, DP-based approach
fn find_best_route(
    valve: isize,
    opened_valves: isize,
    time: isize,
    with_elephant: bool,
    volcano: &Volcano,
    dp: &mut HashMap<State, isize>,
) -> isize {
    if time <= 0 {
        if with_elephant {
            return find_best_route(
                volcano.start_id,
                opened_valves,
                MAX_MINUTES_WITH_ELEPHANT,
                false,
                volcano,
                dp,
            );
        } else {
            return 0;
        }
    }

    let state = State {
        valve,
        time_left: time,
        with_elephant,
        opened_valves,
    };

    if let Some(answer) = dp.get(&state) {
        return *answer;
    };

    let mut answer = 0;
    let not_open = opened_valves & (1 << valve) == 0;
    if not_open {
        let opened_valves = opened_valves | (1 << valve);
        answer = answer.max(
            (time - 1) * volcano.flow_rates.get(&valve).unwrap()
                + find_best_route(valve, opened_valves, time - 1, with_elephant, volcano, dp),
        );
    }

    for (next_valve, distance) in volcano.graph.get(&dbg!(valve)).unwrap() {
        answer = answer.max(find_best_route(
            *next_valve,
            opened_valves,
            time - distance,
            with_elephant,
            volcano,
            dp,
        ))
    }

    dp.insert(state, answer);

    answer
}

pub fn maximize_pressure_release(filename: &str) -> isize {
    let volcano = Volcano::from_file(filename).pruned();

    let mut dp: HashMap<State, isize> = HashMap::new();
    find_best_route(volcano.start_id, 0, MAX_MINUTES, false, &volcano, &mut dp)
}

pub fn maximize_pressure_release_with_elephant(filename: &str) -> isize {
    let volcano = Volcano::from_file(filename).pruned();
    let mut dp: HashMap<State, isize> = HashMap::new();
    find_best_route(
        volcano.start_id,
        0,
        MAX_MINUTES_WITH_ELEPHANT,
        true,
        &volcano,
        &mut dp,
    )
}

#[cfg(test)]
mod tests {
    use crate::{day16, fetch_input};

    #[test]
    fn maximize_pressure_release() {
        fetch_input(16);

        let tests = vec![("example/day16.txt", 1651), ("input/day16.txt", 1789)];

        for test in tests {
            let (file, want) = test;
            let got = day16::maximize_pressure_release(file);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }

    #[test]
    fn maximize_pressure_release_with_elephant() {
        fetch_input(16);

        let tests = vec![("example/day16.txt", 1707), ("input/day16.txt", 2496)];

        for test in tests {
            let (file, want) = test;
            let got = day16::maximize_pressure_release_with_elephant(file);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }
}
