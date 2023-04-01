use std::{
    collections::{HashMap, VecDeque},
    fs,
};

const _MAX_MINUTES: u16 = 30;

struct Volcano {
    flow_rates: HashMap<char, usize>,
    graph: HashMap<char, Vec<char>>,
}

impl Volcano {
    fn neighbours(&self, valve: char) -> &Vec<char> {
        self.graph.get(&valve).unwrap()
    }

    fn flow_rate(&self, valve: char) -> usize {
        *self.flow_rates.get(&valve).unwrap()
    }

    fn next_actions(&self, actions: &Actions) -> Actions {
        let mut next_actions = vec![];

        let n_actions = actions.len();
        let last_action = actions.get(n_actions - 1).unwrap();
        let second_last_action = match n_actions {
            0 | 1 => None,
            _ => actions.get(n_actions - 2),
        };

        if let Action::Move(valve) = last_action {
            if self.flow_rate(*valve) > 0 && !actions.contains(&Action::Open(*valve)) {
                next_actions.push(Action::Open(*valve))
            }
            next_actions.extend(
                self.neighbours(*valve)
                    .iter()
                    // don't go backwards if you are still moving
                    .filter(|valve| match second_last_action {
                        Some(second_last_action) => {
                            Action::Move(**valve) != *second_last_action
                                && Action::Open(**valve) != *second_last_action
                        }
                        None => true,
                    })
                    .map(|neighbour| Action::Move(*neighbour)),
            )
        }

        if let Action::Open(valve) = last_action {
            next_actions.extend(
                self.neighbours(*valve)
                    .iter()
                    .map(|neighbour| Action::Move(*neighbour)),
            )
        }

        next_actions
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Action {
    Move(char),
    Open(char),
}

type Actions = Vec<Action>;

fn pressure_release(actions: Actions, volcano: &Volcano) -> usize {
    actions
        .iter()
        .enumerate()
        .map(|(i, action)| match action {
            Action::Open(valve) => volcano.flow_rate(*valve) * (30 - i),
            Action::Move(_) => 0,
        })
        .sum()
}

pub fn maximize_pressure_release(file: &str) -> usize {
    let volcano = parse_input(file);
    println!("{:#?}", volcano.flow_rates);
    println!("{:#?}", volcano.graph);

    let mut queue = VecDeque::from(vec![vec![Action::Move('A')]]);
    let mut pressure_releases = vec![];

    while let Some(sequence) = dbg!(queue.pop_front()) {
        if sequence.len() > 30 {
            // TODO also exit early if all non-zero valves are open
            let pressure_release = pressure_release(sequence, &volcano);
            pressure_releases.push(pressure_release);
            break;
        }
        for next_action in dbg!(volcano.next_actions(&sequence)) {
            let mut new_sequence = sequence.to_vec();
            new_sequence.push(next_action);
            queue.push_back(dbg!(new_sequence))
        }
    }

    // let possible_routes = vec![];

    *pressure_releases.iter().max().unwrap()
}

fn parse_input(file: &str) -> Volcano {
    let mut graph = HashMap::new();
    let mut flow_rates = HashMap::new();

    let lines: Vec<(char, usize, Vec<char>)> = fs::read_to_string(file)
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
                line[0].chars().nth(0).unwrap(),
                line[1].clone().parse().unwrap(),
                line[2]
                    .split(",")
                    .map(|valve| valve.chars().nth(0).unwrap())
                    .collect(),
            )
        })
        .collect();

    for (valve, flow_rate, neighbours) in lines {
        graph.insert(valve.clone(), neighbours);
        flow_rates.insert(valve.clone(), flow_rate);
    }

    Volcano { graph, flow_rates }
}

#[cfg(test)]
mod tests {
    use crate::{day16, fetch_input};

    #[test]
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
