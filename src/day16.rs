/*
 * It works but is slow. Three ways to speed it up before moving on to part 2:
 * 1. Prune graph before doing search
 * 2. Don't do check for next steps twice - only once
 * 3. Don't pursue paths that could not possibly yield the maximum: what would happen if
 *    I turned on all valves? If that still yields a lower benefit than the existing maximum,
 *    then don't pursue it.
 */
use std::{collections::HashMap, fs};

use crate::queue::PriorityQueue;

const MAX_MINUTES: usize = 30;
const START: Valve = ['A', 'A'];

type Distances<'a> = HashMap<&'a Valve, HashMap<&'a Valve, usize>>;

trait NestedGetter<T, S> {
    fn get_nested(&self, a: &T, b: &T) -> Option<S>;
}

impl<'a> NestedGetter<Valve, usize> for Distances<'a> {
    fn get_nested(&self, a: &Valve, b: &Valve) -> Option<usize> {
        Some(*self.get(a)?.get(b)?)
    }
}

#[derive(Debug)]
struct ValveOpenBenefit<'a> {
    target_valve: &'a Valve,
    current_minute: usize,
    benefit: usize,
}

type Valve = [char; 2];

#[derive(Debug)]
struct Volcano {
    flow_rates: HashMap<Valve, usize>,
    graph: HashMap<Valve, HashMap<Valve, usize>>,
}

impl Volcano {
    fn from_file(filename: &str) -> Self {
        let mut graph = HashMap::new();
        let mut flow_rates = HashMap::new();

        let lines: Vec<(Valve, usize, HashMap<Valve, usize>)> = fs::read_to_string(filename)
            .unwrap()
            .lines()
            .map(|line| {
                let line = line
                    .replace("Valve ", "")
                    .replace("has flow rate=", "")
                    .replace("; tunnels lead to valves", "")
                    .replace("; tunnel leads to valve", "")
                    .replace(", ", ",");

                let mut line = line.split_whitespace();

                (
                    line.next()
                        .unwrap()
                        .chars()
                        .collect::<Vec<char>>()
                        .try_into()
                        .unwrap(),
                    line.next().unwrap().parse().unwrap(),
                    line.next()
                        .unwrap()
                        .split(',')
                        .map(|valve| valve.chars().collect::<Vec<char>>().try_into().unwrap())
                        .map(|valve| (valve, 1)) // to be pruned later
                        .collect(),
                )
            })
            .collect();

        for (valve, flow_rate, neighbours) in lines {
            graph.insert(valve, neighbours);
            flow_rates.insert(valve, flow_rate);
        }

        Self { graph, flow_rates }
    }

    fn neighbours(&self, valve: &Valve) -> &HashMap<Valve, usize> {
        self.graph.get(valve).unwrap()
    }

    fn valves(&self) -> Vec<&Valve> {
        self.graph.keys().collect()
    }

    fn flow_rate(&self, valve: &Valve) -> usize {
        *self.flow_rates.get(valve).unwrap()
    }

    // djikstra to find shortest path between every pair of nodes.
    fn compute_pairwise_distances(&self) -> Distances {
        let mut distances = HashMap::new();

        for start in self.graph.keys() {
            let mut frontier = PriorityQueue::new();
            frontier.push(start, 0);
            let mut cost_so_far = HashMap::new();
            cost_so_far.insert(start, 0);

            while let Some(current) = frontier.pop() {
                for (next, distance) in self.neighbours(current) {
                    let new_cost = cost_so_far.get(&current).unwrap() + distance;
                    let next_cost_so_far = cost_so_far.get(&next);

                    if next_cost_so_far.is_none() || Some(&new_cost) < next_cost_so_far {
                        cost_so_far.insert(next, new_cost);
                        frontier.push(next, new_cost);
                    }
                }
            }

            distances.insert(start, cost_so_far);
        }

        distances
    }

    fn benefit_from_opening_valve<'a>(
        &self,
        path: &[&Valve],
        target_valve: &'a Valve,
        current_minute: usize,
        pairwise_distances: &Distances,
    ) -> Option<ValveOpenBenefit<'a>> {
        let current_valve = path.last().unwrap();

        if path.contains(&target_valve) {
            return None;
        }

        let minutes_travelled = pairwise_distances.get_nested(current_valve, target_valve)?;
        let first_minute_of_pressure_release = current_minute + minutes_travelled + 1; // include 1 minute to open

        if first_minute_of_pressure_release > MAX_MINUTES {
            return None;
        }

        let flow_rate = self.flow_rate(target_valve);
        let minutes_of_pressure_release = MAX_MINUTES - first_minute_of_pressure_release + 1; // inclusive counting
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
        path: &[&Valve],
        current_minute: usize,
        pairwise_distances: &Distances,
    ) -> Vec<ValveOpenBenefit> {
        self.valves()
            .into_iter()
            .filter_map(|target_valve| {
                self.benefit_from_opening_valve(
                    path,
                    target_valve,
                    current_minute,
                    pairwise_distances,
                )
            })
            .collect()
    }

    fn prune_graph(&mut self) {
        let valves_to_prune: Vec<&Valve> = self
            .flow_rates
            .iter()
            .filter_map(|(valve, flow_rate)| {
                if valve == &START || flow_rate > &0 {
                    None
                } else {
                    Some(valve)
                }
            })
            .collect();

        for valve in valves_to_prune {
            let neighbours = self.graph.remove(valve).unwrap();
            for (neighbour, distance_to_prunee) in neighbours.iter() {
                let others: Vec<&Valve> = neighbours.keys().filter(|x| x != &neighbour).collect();
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
    }
}

fn find_all_routes(volcano: &Volcano, pairwise_distances: &Distances) -> Vec<usize> {
    let start = vec![&START];
    let start_minute = 1;
    let start_benefit = 0;
    let initial_possible_next_moves =
        volcano.benefit_from_opening_all_valves(&start, start_minute, pairwise_distances);
    find_all_routes_recursive(
        start,
        start_benefit,
        volcano,
        pairwise_distances,
        initial_possible_next_moves,
    )
}

fn find_all_routes_recursive(
    path: Vec<&Valve>,
    benefit: usize,
    volcano: &Volcano,
    pairwise_distances: &Distances,
    next_possible_benefits: Vec<ValveOpenBenefit>,
) -> Vec<usize> {
    next_possible_benefits
        .iter()
        .flat_map(|valve_open_benefit| {
            let mut path = path.clone();
            path.push(valve_open_benefit.target_valve);

            let next_possible_valve_openings = volcano.benefit_from_opening_all_valves(
                &path,
                valve_open_benefit.current_minute,
                pairwise_distances,
            );

            if next_possible_valve_openings.is_empty() {
                vec![benefit + valve_open_benefit.benefit]
            } else {
                find_all_routes_recursive(
                    path,
                    benefit + valve_open_benefit.benefit,
                    volcano,
                    pairwise_distances,
                    next_possible_valve_openings,
                )
            }
        })
        .collect()
}

pub fn maximize_pressure_release(filename: &str) -> usize {
    let mut volcano = Volcano::from_file(filename);
    volcano.prune_graph();
    let pairwise_distances = volcano.compute_pairwise_distances();
    find_all_routes(&volcano, &pairwise_distances)
        .into_iter()
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{day16, fetch_input};

    #[test]
    fn maximize_pressure_release() {
        fetch_input(16);

        // let tests = vec![("example/day16.txt", 1651)];
        // let tests = vec![("input/day16.txt", 1789)];
        let tests = vec![("example/day16.txt", 1651), ("input/day16.txt", 1789)];

        for test in tests {
            let (file, want) = test;
            let got = day16::maximize_pressure_release(file);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }
}
