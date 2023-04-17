/*
 * It works but is slow. Three ways to speed it up before moving on to part 2:
 * 1. Prune graph before doing search
 * 2. Don't do check for next steps twice - only once
 * 3. Don't pursue paths that could not possibly yield the maximum: what would happen if
 *    I turned on all valves? If that still yields a lower benefit than the existing maximum,
 *    then don't pursue it.
 */
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

use crate::queue::PriorityQueue;

const MAX_MINUTES: usize = 30;
const MAX_MINUTES_WITH_ELEPHANT: usize = 26;
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
struct ValveOpenBenefit {
    target_valve: Valve,
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

    fn benefit_from_opening_valve(
        &self,
        visited: &HashSet<Valve>,
        current_valve: Valve,
        target_valve: Valve,
        current_minute: usize,
        pairwise_distances: &Distances,
        max_minutes: usize,
    ) -> Option<ValveOpenBenefit> {
        if visited.contains(&target_valve) {
            return None;
        }

        let minutes_travelled = pairwise_distances.get_nested(&current_valve, &target_valve)?;
        let first_minute_of_pressure_release = current_minute + minutes_travelled + 1; // include 1 minute to open

        if first_minute_of_pressure_release > max_minutes {
            return None;
        }

        let flow_rate = self.flow_rate(&target_valve);
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
        visited: &HashSet<Valve>,
        current_valve: Valve,
        current_minute: usize,
        pairwise_distances: &Distances,
        max_minutes: usize,
    ) -> Vec<ValveOpenBenefit> {
        self.valves()
            .into_iter()
            .filter_map(|target_valve| {
                self.benefit_from_opening_valve(
                    visited,
                    current_valve,
                    *target_valve,
                    current_minute,
                    pairwise_distances,
                    max_minutes,
                )
            })
            .collect()
    }

    fn pruned(mut self) -> Self {
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

        self
    }
}

#[derive(Debug, Clone)]
struct TeamRoute {
    visited: HashSet<Valve>,
    benefit: usize,
    human_current_valve: Valve,
    human_minute: usize,
    elephant_current_valve: Valve,
    elephant_minute: usize,
}

fn find_all_routes(volcano: &Volcano, pairwise_distances: &Distances) -> Vec<usize> {
    let visited = HashSet::from([START]);
    let current_valve = START;
    let start_minute = 1;
    let start_benefit = 0;
    let initial_possible_next_moves = volcano.benefit_from_opening_all_valves(
        &visited,
        current_valve,
        start_minute,
        pairwise_distances,
        MAX_MINUTES,
    );
    find_all_routes_recursive(
        &visited,
        start_benefit,
        volcano,
        pairwise_distances,
        initial_possible_next_moves,
        MAX_MINUTES,
    )
}

fn find_all_routes_recursive(
    visited: &HashSet<Valve>,
    benefit: usize,
    volcano: &Volcano,
    pairwise_distances: &Distances,
    next_possible_benefits: Vec<ValveOpenBenefit>,
    max_minutes: usize,
) -> Vec<usize> {
    next_possible_benefits
        .iter()
        .flat_map(|valve_open_benefit| {
            let mut visited = visited.clone();
            visited.insert(valve_open_benefit.target_valve);

            let next_possible_valve_openings = volcano.benefit_from_opening_all_valves(
                &visited,
                valve_open_benefit.target_valve,
                valve_open_benefit.current_minute,
                pairwise_distances,
                max_minutes,
            );

            if next_possible_valve_openings.is_empty() {
                vec![benefit + valve_open_benefit.benefit]
            } else {
                find_all_routes_recursive(
                    &visited,
                    benefit + valve_open_benefit.benefit,
                    volcano,
                    pairwise_distances,
                    next_possible_valve_openings,
                    max_minutes,
                )
            }
        })
        .collect()
}

pub fn maximize_pressure_release(filename: &str) -> usize {
    let volcano = Volcano::from_file(filename).pruned();
    let pairwise_distances = volcano.compute_pairwise_distances();
    find_all_routes(&volcano, &pairwise_distances)
        .into_iter()
        .max()
        .unwrap()
}

pub fn maximize_pressure_release_with_elephant(filename: &str) -> usize {
    let volcano = Volcano::from_file(filename).pruned();
    let pairwise_distances = volcano.compute_pairwise_distances();

    let mut possible_routes: VecDeque<TeamRoute> = VecDeque::new();
    possible_routes.push_front(TeamRoute {
        visited: HashSet::from([START]),
        benefit: 0,
        human_current_valve: START,
        human_minute: 1,
        elephant_current_valve: START,
        elephant_minute: 1,
    });

    let mut max_benefit = 0;

    while let Some(team_route) = possible_routes.pop_back() {
        // what happens if we open all the valves right now
        /* let maximum_possible_benefit: usize = volcano
            .flow_rates
            .iter()
            .map(|(valve, _)| {
                let benefit = volcano.benefit_from_opening_valve(
                    &team_route.visited,
                    *valve,
                    *valve,
                    min(team_route.human_minute, team_route.elephant_minute),
                    &pairwise_distances,
                    MAX_MINUTES_WITH_ELEPHANT,
                );

                match benefit {
                    None => 0,
                    Some(benefit) => benefit.benefit,
                }
            })
            .sum();

        if maximum_possible_benefit < max_benefit {
            println!("purged a possibility!");
            continue;
        } */

        println!("{team_route:?}");
        let possible_human_benefits = volcano.benefit_from_opening_all_valves(
            &team_route.visited,
            team_route.human_current_valve,
            team_route.human_minute,
            &pairwise_distances,
            MAX_MINUTES_WITH_ELEPHANT,
        );

        let possible_elephant_benefits = volcano.benefit_from_opening_all_valves(
            &team_route.visited,
            team_route.elephant_current_valve,
            team_route.elephant_minute,
            &pairwise_distances,
            MAX_MINUTES_WITH_ELEPHANT,
        );

        if possible_human_benefits.is_empty() && possible_elephant_benefits.is_empty() {
            if team_route.benefit > max_benefit {
                max_benefit = team_route.benefit;
            }
            continue;
        }

        for human_benefit in possible_human_benefits {
            for elephant_benefit in possible_elephant_benefits.iter() {
                if human_benefit.target_valve == elephant_benefit.target_valve {
                    continue;
                }

                let mut team_route = team_route.clone();
                team_route.visited.insert(human_benefit.target_valve);
                team_route.visited.insert(elephant_benefit.target_valve);
                team_route.human_current_valve = human_benefit.target_valve;
                team_route.elephant_current_valve = elephant_benefit.target_valve;
                team_route.human_minute = human_benefit.current_minute;
                team_route.elephant_minute = elephant_benefit.current_minute;
                team_route.benefit += human_benefit.benefit + elephant_benefit.benefit;

                possible_routes.push_front(team_route);
            }
        }
    }

    max_benefit
}

#[cfg(test)]
mod tests {
    use crate::{day16, fetch_input};

    #[test]
    // #[ignore]
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

    #[test]
    // #[ignore]
    fn maximize_pressure_release_with_elephant() {
        fetch_input(16);

        let tests = vec![("example/day16.txt", 1707)];
        // let tests = vec![("input/day16.txt", 0)];
        // let tests = vec![("example/day16.txt", 1707), ("input/day16.txt", 0)];

        for test in tests {
            let (file, want) = test;
            let got = day16::maximize_pressure_release_with_elephant(file);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }
}
