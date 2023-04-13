use std::{collections::HashMap, fs};

use crate::queue::PriorityQueue;

const MAX_MINUTES: usize = 30;
const START: char = 'A';
const TRAVEL_COST: usize = 1; // this is an unweighted graph

type Distances<'a> = HashMap<&'a char, HashMap<&'a char, usize>>;

trait NestedGetter<T, S> {
    fn get_nested(&self, a: &T, b: &T) -> Option<S>;
}

impl<'a> NestedGetter<char, usize> for Distances<'a> {
    fn get_nested(&self, a: &char, b: &char) -> Option<usize> {
        Some(*self.get(a)?.get(b)?)
    }
}

struct ValveOpenBenefit<'a> {
    target_valve: &'a char,
    time_elapsed: usize,
    benefit: usize,
}

struct Volcano {
    flow_rates: HashMap<char, usize>,
    graph: HashMap<char, Vec<char>>,
}

impl Volcano {
    fn from_file(filename: &str) -> Self {
        let mut graph = HashMap::new();
        let mut flow_rates = HashMap::new();

        let lines: Vec<(char, usize, Vec<char>)> = fs::read_to_string(filename)
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
                    line.next().unwrap().chars().next().unwrap(),
                    line.next().unwrap().parse().unwrap(),
                    line.next()
                        .unwrap()
                        .split(',')
                        .map(|valve| valve.chars().next().unwrap())
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

    fn neighbours(&self, valve: &char) -> &Vec<char> {
        self.graph.get(valve).unwrap()
    }

    fn valves(&self) -> Vec<&char> {
        self.graph.keys().collect()
    }

    fn flow_rate(&self, valve: &char) -> usize {
        *self.flow_rates.get(valve).unwrap()
    }

    fn compute_pairwise_distances(&self) -> Distances {
        let mut distances = HashMap::new();

        for start in self.graph.keys() {
            let mut frontier = PriorityQueue::new();
            frontier.push(start, 0);
            let mut cost_so_far = HashMap::new();
            cost_so_far.insert(start, 0);

            while let Some(current) = frontier.pop() {
                for next in self.neighbours(current) {
                    let new_cost = cost_so_far.get(&current).unwrap() + TRAVEL_COST;
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
        path: Vec<&char>,
        target_valve: &'a char,
        time_elapsed: usize,
        pairwise_distances: &Distances,
    ) -> Option<ValveOpenBenefit<'a>> {
        let current_valve = path.last().unwrap();

        if &target_valve == current_valve {
            return None;
        }

        if path.contains(&target_valve) {
            return None;
        }

        let flow_rate = self.flow_rate(target_valve);

        if flow_rate == 0 {
            return None;
        }

        let distance = pairwise_distances
            .get_nested(current_valve, target_valve)
            .unwrap();

        let time_after_valve_opened = time_elapsed + distance + 1;

        if time_after_valve_opened > MAX_MINUTES {
            return None;
        }

        let time_remaining = MAX_MINUTES - time_after_valve_opened;
        let benefit = flow_rate * time_remaining;

        Some(ValveOpenBenefit {
            target_valve,
            time_elapsed: time_after_valve_opened,
            benefit,
        })
    }

    fn benefit_from_opening_all_valves(
        &self,
        path: Vec<&char>,
        time_elapsed: usize,
        pairwise_distances: &Distances,
    ) -> Vec<ValveOpenBenefit> {
        self.valves()
            .into_iter()
            .filter_map(|target_valve| {
                self.benefit_from_opening_valve(
                    path.clone(),
                    target_valve,
                    time_elapsed,
                    pairwise_distances,
                )
            })
            .collect()
    }
}

fn find_all_routes(
    path: Vec<&char>,
    time_elapsed: usize,
    benefit: usize,
    volcano: &Volcano,
    pairwise_distances: &Distances,
) -> Vec<usize> {
    volcano
        .benefit_from_opening_all_valves(path.clone(), time_elapsed, pairwise_distances)
        .iter()
        .flat_map(|valve_open_benefit| {
            let mut path = path.clone();
            path.push(valve_open_benefit.target_valve);

            let next_possible_valve_openings = volcano.benefit_from_opening_all_valves(
                path.clone(),
                valve_open_benefit.time_elapsed,
                pairwise_distances,
            );

            if next_possible_valve_openings.is_empty() {
                println!("{path:?}: {benefit}");
                vec![benefit]
            } else {
                find_all_routes(
                    path.clone(),
                    valve_open_benefit.time_elapsed,
                    benefit + valve_open_benefit.benefit,
                    volcano,
                    pairwise_distances,
                )
            }
        })
        // .copied()
        .collect()
}

pub fn maximize_pressure_release(filename: &str) -> usize {
    let volcano = Volcano::from_file(filename);
    let pairwise_distances = volcano.compute_pairwise_distances();

    find_all_routes(vec![&START], 0, 0, &volcano, &pairwise_distances)
        .into_iter()
        .max()
        .unwrap()

    /* for (target, time_remaining, benefit) in
        volcano.benefit_from_all_valve_openings(&pairwise_distances, current_time, path.clone())
    {
        let mut new_path = path.clone();
        new_path.push(target);
        for (target_two, time_remaining_two, benefit_two) in volcano
            .benefit_from_all_valve_openings(
                &pairwise_distances,
                MAX_MINUTES - time_remaining,
                new_path.clone(),
            )
        {
            let mut new_new_path = new_path.clone();
            new_new_path.push(target_two);
            println!(
                "{:?}: {} pressure release, {} minutes left",
                new_new_path,
                benefit + benefit_two,
                time_remaining_two
            );
        }
    } */
}

#[cfg(test)]
mod tests {
    use crate::{day16, fetch_input};

    #[test]
    #[ignore = "in progress"]
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
