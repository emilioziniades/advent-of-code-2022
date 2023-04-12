use std::{collections::HashMap, fs};

const _MAX_MINUTES: u16 = 30;

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

    fn _neighbours(&self, valve: &char) -> &Vec<char> {
        self.graph.get(valve).unwrap()
    }

    fn _flow_rate(&self, valve: &char) -> usize {
        *self.flow_rates.get(valve).unwrap()
    }
}
pub fn maximize_pressure_release(file: &str) -> usize {
    let volcano = Volcano::from_file(file);
    println!("{:#?}", volcano.flow_rates);
    println!("{:#?}", volcano.graph);

    0
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
