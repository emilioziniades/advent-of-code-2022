use std::{
    collections::{HashMap, HashSet},
    fs,
};

use crate::queue::PriorityQueue;

const MAX_MINUTES: usize = 30;
const MAX_MINUTES_WITH_ELEPHANT: usize = 26;
const START_VALVE: usize = 0;

type Distances = HashMap<usize, HashMap<usize, usize>>;

#[derive(Debug)]
struct ValveOpenBenefit {
    target_valve: usize,
    current_minute: usize,
    benefit: usize,
}

#[derive(Debug)]
struct Volcano {
    flow_rates: HashMap<usize, usize>,
    graph: HashMap<usize, HashMap<usize, usize>>,
    pairwise_distances: Distances,
    start_id: usize,
}

impl Volcano {
    fn from_file(filename: &str) -> Self {
        let mut graph = HashMap::new();
        let mut flow_rates = HashMap::new();

        let input = fs::read_to_string(filename).unwrap();

        let valve_name_to_id: HashMap<String, usize> = input
            .lines()
            .enumerate()
            .map(|(id, line)| {
                let line = line.replace("Valve ", "");
                let valve_name = line
                    .split_whitespace()
                    .into_iter()
                    .next()
                    .unwrap()
                    .to_string();
                (valve_name, id)
            })
            .collect();

        println!("{valve_name_to_id:#?}");

        let lines: Vec<(usize, usize, HashMap<usize, usize>)> = fs::read_to_string(filename)
            .unwrap()
            .lines()
            .enumerate()
            .map(|(id, line)| {
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
            pairwise_distances: HashMap::new(),
            start_id: *valve_name_to_id.get("AA").unwrap(),
        }
    }

    fn pruned(mut self) -> Self {
        let valves_to_prune: Vec<&usize> = self
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
                let others: Vec<&usize> = neighbours.keys().filter(|x| x != &neighbour).collect();
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

    // djikstra to find shortest path between every pair of nodes.
    fn with_pairwise_distances(mut self) -> Self {
        for start in self.graph.keys() {
            let mut frontier = PriorityQueue::new();
            frontier.push(start, 0);
            let mut cost_so_far = HashMap::new();
            cost_so_far.insert(*start, 0);

            while let Some(current) = frontier.pop() {
                for (next, distance) in self.graph.get(current).unwrap() {
                    let new_cost = cost_so_far.get(current).unwrap() + distance;
                    let next_cost_so_far = cost_so_far.get(next);

                    if next_cost_so_far.is_none() || Some(&new_cost) < next_cost_so_far {
                        cost_so_far.insert(*next, new_cost);
                        frontier.push(next, new_cost);
                    }
                }
            }

            self.pairwise_distances.insert(*start, cost_so_far);
        }

        self
    }

    fn benefit_from_opening_valve(
        &self,
        visited: &HashSet<usize>,
        current_valve: usize,
        target_valve: usize,
        current_minute: usize,
        max_minutes: usize,
    ) -> Option<ValveOpenBenefit> {
        if visited.contains(&target_valve) {
            return None;
        }

        let minutes_travelled = self
            .pairwise_distances
            .get(&current_valve)?
            .get(&target_valve)?;
        let first_minute_of_pressure_release = current_minute + minutes_travelled + 1; // include 1 minute to open

        if first_minute_of_pressure_release > max_minutes {
            return None;
        }

        let flow_rate = self.flow_rates.get(&target_valve).unwrap();
        let minutes_of_pressure_release = max_minutes - first_minute_of_pressure_release + 1; // inclusive counting
        let benefit = flow_rate * minutes_of_pressure_release;

        let valve_open_benefit = ValveOpenBenefit {
            target_valve,
            current_minute: first_minute_of_pressure_release,
            benefit,
        };

        Some(valve_open_benefit)
    }

    fn benefit_from_opening_all_valves(
        &self,
        visited: &HashSet<usize>,
        current_valve: usize,
        current_minute: usize,
        max_minutes: usize,
    ) -> Vec<ValveOpenBenefit> {
        self.graph
            .keys()
            .filter_map(|target_valve| {
                self.benefit_from_opening_valve(
                    visited,
                    current_valve,
                    *target_valve,
                    current_minute,
                    max_minutes,
                )
            })
            .collect()
    }
}

fn f(
    valve: usize,
    opened_valves: usize,
    time: usize,
    other_players: usize,
    volcano: &Volcano,
) -> usize {
    let mut answer = 0;
    let not_open = opened_valves & (1 << valve) == 0;
    if not_open {
        let opened_valves = opened_valves | (1 << valve);
        answer = answer.max(
            (time - 1) * volcano.flow_rates.get(&valve).unwrap()
                + f(valve, opened_valves, time - 1, other_players, volcano),
        );
    }
    return answer;
}

fn find_best_route(volcano: &Volcano) -> usize {
    let visited = HashSet::from([volcano.start_id]);
    let current_valve = volcano.start_id;
    let start_minute = 1;
    let start_benefit = 0;
    let mut best = 0;
    let initial_possible_next_moves =
        volcano.benefit_from_opening_all_valves(&visited, current_valve, start_minute, MAX_MINUTES);
    find_all_routes_recursive(
        &visited,
        start_benefit,
        volcano,
        initial_possible_next_moves,
        MAX_MINUTES,
        &mut best,
    );

    best
}

fn find_all_routes_recursive(
    visited: &HashSet<usize>,
    benefit: usize,
    volcano: &Volcano,
    next_possible_benefits: Vec<ValveOpenBenefit>,
    max_minutes: usize,
    best: &mut usize,
) {
    for next_possible_benefit in next_possible_benefits {
        let mut visited = visited.clone();
        visited.insert(next_possible_benefit.target_valve);

        let next_possible_valve_openings = volcano.benefit_from_opening_all_valves(
            &visited,
            next_possible_benefit.target_valve,
            next_possible_benefit.current_minute,
            max_minutes,
        );

        if next_possible_valve_openings.is_empty() {
            let current_best = benefit + next_possible_benefit.benefit;
            if current_best > *best {
                *best = current_best;
            }
        } else {
            find_all_routes_recursive(
                &visited,
                benefit + next_possible_benefit.benefit,
                volcano,
                next_possible_valve_openings,
                max_minutes,
                best,
            );
        }
    }
}

fn find_best_route_with_elephant(volcano: &Volcano) -> usize {
    let visited = HashSet::from([START_VALVE]);
    let current_valve_human = START_VALVE;
    let current_valve_elephant = START_VALVE;
    let start_minute_human = 1;
    let start_minute_elephant = 1;
    let start_benefit = 0;
    let mut best = 0;

    let initial_possible_human_benefits = volcano.benefit_from_opening_all_valves(
        &visited,
        current_valve_human,
        start_minute_human,
        MAX_MINUTES_WITH_ELEPHANT,
    );

    let initial_possible_elephant_benefits = volcano.benefit_from_opening_all_valves(
        &visited,
        current_valve_elephant,
        start_minute_elephant,
        MAX_MINUTES_WITH_ELEPHANT,
    );

    find_all_routes_recursive_with_elephant(
        &visited,
        start_benefit,
        volcano,
        initial_possible_human_benefits,
        initial_possible_elephant_benefits,
        MAX_MINUTES_WITH_ELEPHANT,
        &mut best,
    );

    best
}

fn find_all_routes_recursive_with_elephant(
    visited: &HashSet<usize>,
    benefit: usize,
    volcano: &Volcano,
    next_possible_human_benefits: Vec<ValveOpenBenefit>,
    next_possible_elephant_benefits: Vec<ValveOpenBenefit>,
    max_minutes: usize,
    best_benefit: &mut usize,
) {
    for human_open_benefit in next_possible_human_benefits {
        for elephant_open_benefit in &next_possible_elephant_benefits {
            if human_open_benefit.target_valve == elephant_open_benefit.target_valve {
                continue;
            }

            let mut visited = visited.clone();
            visited.insert(human_open_benefit.target_valve);
            visited.insert(elephant_open_benefit.target_valve);

            let next_possible_human_benefits = volcano.benefit_from_opening_all_valves(
                &visited,
                human_open_benefit.target_valve,
                human_open_benefit.current_minute,
                max_minutes,
            );

            let next_possible_elephant_benefits = volcano.benefit_from_opening_all_valves(
                &visited,
                elephant_open_benefit.target_valve,
                elephant_open_benefit.current_minute,
                max_minutes,
            );

            if next_possible_human_benefits.is_empty() && next_possible_elephant_benefits.is_empty()
            {
                let solution_benefit =
                    benefit + human_open_benefit.benefit + elephant_open_benefit.benefit;
                if solution_benefit > *best_benefit {
                    *best_benefit = solution_benefit;
                }
            } else {
                find_all_routes_recursive_with_elephant(
                    &visited,
                    benefit + human_open_benefit.benefit + elephant_open_benefit.benefit,
                    volcano,
                    next_possible_human_benefits,
                    next_possible_elephant_benefits,
                    max_minutes,
                    best_benefit,
                );
            }
        }
    }
}

/* #[derive(Clone, Copy)]
struct State {
    valve: usize,
    opened_valves: usize,
    time_left: usize,
    other_players: usize,
} */

pub fn maximize_pressure_release(filename: &str) -> usize {
    let volcano = Volcano::from_file(filename)
        .pruned()
        .with_pairwise_distances();

    println!("{:#?}", volcano);
    // return 0;
    // let dp: HashMap<State, usize> = HashMap::new();
    // f(0, 0, MAX_MINUTES, 0, &volcano)
    find_best_route(&volcano)
}

pub fn maximize_pressure_release_with_elephant(filename: &str) -> usize {
    let volcano = Volcano::from_file(filename)
        .pruned()
        .with_pairwise_distances();
    find_best_route_with_elephant(&volcano)
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
    #[ignore = "who wants to wait 20 minutes for a test to run"]
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
