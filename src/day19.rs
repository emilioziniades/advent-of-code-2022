use std::fs;

const MINUTES: usize = 24;

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_for_orebot: usize,
    ore_for_claybot: usize,
    ore_for_obsidianbot: usize,
    clay_for_obsidianbot: usize,
    ore_for_geodebot: usize,
    obsidian_for_geodebot: usize,
}

impl Blueprint {
    fn new(line: &str) -> Self {
        let line = line
            .replace("Blueprint", "")
            .replace(':', "")
            .replace("Each", "")
            .replace("ore", "")
            .replace("robot", "")
            .replace("costs", "")
            .replace('.', "")
            .replace("clay", "")
            .replace("obsidian", "")
            .replace("geode", "")
            .replace("and", "");
        let mut numbers = line.split_whitespace();
        Self {
            id: numbers.next().unwrap().parse().unwrap(),
            ore_for_orebot: numbers.next().unwrap().parse().unwrap(),
            ore_for_claybot: numbers.next().unwrap().parse().unwrap(),
            ore_for_obsidianbot: numbers.next().unwrap().parse().unwrap(),
            clay_for_obsidianbot: numbers.next().unwrap().parse().unwrap(),
            ore_for_geodebot: numbers.next().unwrap().parse().unwrap(),
            obsidian_for_geodebot: numbers.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    elapsed_minutes: usize,
    ores: usize,
    clays: usize,
    obsidians: usize,
    geodes: usize,
    orebots: usize,
    claybots: usize,
    obsidianbots: usize,
    geodebots: usize,
}

impl State {
    fn new() -> Self {
        Self {
            elapsed_minutes: 0,
            ores: 0,
            clays: 0,
            obsidians: 0,
            geodes: 0,
            orebots: 1,
            claybots: 0,
            obsidianbots: 0,
            geodebots: 0,
        }
    }

    fn tick(&mut self, minutes: usize) {
        self.elapsed_minutes += minutes;
        self.ores += self.orebots * minutes;
        self.clays += self.claybots * minutes;
        self.obsidians += self.obsidianbots * minutes;
        self.geodes += self.geodebots * minutes;
    }

    fn build_orebot(&mut self, blueprint: &Blueprint) {
        self.ores -= blueprint.ore_for_orebot;
        self.orebots += 1;
    }

    fn build_claybot(&mut self, blueprint: &Blueprint) {
        self.ores -= blueprint.ore_for_claybot;
        self.claybots += 1;
    }

    fn build_obsidianbot(&mut self, blueprint: &Blueprint) {
        self.ores -= blueprint.ore_for_obsidianbot;
        self.clays -= blueprint.clay_for_obsidianbot;
        self.obsidianbots += 1;
    }

    fn build_geodebot(&mut self, blueprint: &Blueprint) {
        self.ores -= blueprint.ore_for_geodebot;
        self.obsidians -= blueprint.obsidian_for_geodebot;
        self.geodebots += 1;
    }
}

fn max_geodes(state: State, blueprint: &Blueprint) -> usize {
    let mut queue: Vec<State> = Vec::new();
    let mut max_geodes = 0;
    let mut explored_states = 0;
    queue.push(state);

    let max_orebots = blueprint
        .ore_for_orebot
        .max(blueprint.ore_for_claybot)
        .max(blueprint.ore_for_obsidianbot)
        .max(blueprint.ore_for_geodebot);
    let max_claybots = blueprint.clay_for_obsidianbot;
    let max_obsidianbots = blueprint.obsidian_for_geodebot;

    while let Some(state) = queue.pop() {
        explored_states += 1;

        let max_possible_geodes =
            state.geodes + (MINUTES - state.elapsed_minutes) * state.geodebots;
        max_geodes = max_geodes.max(max_possible_geodes);

        // build orebot
        if state.orebots < max_orebots {
            if state.ores >= blueprint.ore_for_orebot {
                // build orebot now
                let mut state = state;
                state.tick(1);
                state.build_orebot(&blueprint);
                if state.elapsed_minutes < MINUTES {
                    queue.push(state);
                }
            } else {
                // build orebot later
                let mut state = state;
                let minutes_left = div_ceil(blueprint.ore_for_orebot - state.ores, state.orebots);
                state.tick(minutes_left + 1);
                state.build_orebot(blueprint);
                if state.elapsed_minutes < MINUTES {
                    queue.push(state);
                }
            }
        }

        // build claybot
        if state.claybots < max_claybots {
            if state.ores >= blueprint.ore_for_claybot {
                // build claybot now
                let mut state = state;
                state.tick(1);
                state.build_claybot(blueprint);
                if state.elapsed_minutes < MINUTES {
                    queue.push(state);
                }
            } else {
                // build claybot later
                let mut state = state;
                let minutes_left = div_ceil(blueprint.ore_for_claybot - state.ores, state.orebots);
                state.tick(minutes_left + 1);
                state.build_claybot(blueprint);
                if state.elapsed_minutes < MINUTES {
                    queue.push(state);
                }
            }
        }

        // build obsidianbots
        if state.obsidianbots < max_obsidianbots && state.claybots > 0 {
            let enough_ore = state.ores >= blueprint.ore_for_obsidianbot;
            let enough_clay = state.clays >= blueprint.clay_for_obsidianbot;

            if enough_ore && enough_clay {
                // build obsidianbot now
                let mut state = state;
                state.tick(1);
                state.build_obsidianbot(blueprint);
                if state.elapsed_minutes < MINUTES {
                    queue.push(state);
                }
            } else {
                // build obsidianbot later
                let mut state = state;
                let minutes_until_ore = if enough_ore {
                    0
                } else {
                    div_ceil(blueprint.ore_for_obsidianbot - state.ores, state.orebots)
                };
                let minutes_until_clay = if enough_clay {
                    0
                } else {
                    div_ceil(blueprint.clay_for_obsidianbot - state.clays, state.claybots)
                };
                let minutes_left = minutes_until_ore.max(minutes_until_clay);
                state.tick(minutes_left + 1);
                state.build_obsidianbot(blueprint);
                if state.elapsed_minutes < MINUTES {
                    queue.push(state);
                }
            }
        }

        // build geodebots
        if state.obsidianbots > 0 {
            let enough_ore = state.ores >= blueprint.ore_for_geodebot;
            let enough_obsidian = state.obsidians >= blueprint.obsidian_for_geodebot;

            if enough_ore && enough_obsidian {
                // build geodebot now
                let mut state = state;
                state.tick(1);
                state.build_geodebot(blueprint);
                if state.elapsed_minutes < MINUTES {
                    queue.push(state);
                }
            } else {
                // build geodebot later
                let mut state = state;
                let minutes_until_ore = if enough_ore {
                    0
                } else {
                    div_ceil(blueprint.ore_for_geodebot - state.ores, state.orebots)
                };
                let minutes_until_obsidian = if enough_obsidian {
                    0
                } else {
                    div_ceil(
                        blueprint.obsidian_for_geodebot - state.obsidians,
                        state.obsidianbots,
                    )
                };
                let minutes_left = minutes_until_ore.max(minutes_until_obsidian);
                state.tick(minutes_left + 1);
                state.build_geodebot(blueprint);
                if state.elapsed_minutes < MINUTES {
                    queue.push(state);
                }
            }
        }
    }

    dbg!(explored_states);
    dbg!(max_geodes)
}

fn div_ceil(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}

pub fn sum_quality_levels(filename: &str) -> usize {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        // .take(1) // TODO: remove once happy
        .map(Blueprint::new)
        .map(|blueprint| max_geodes(State::new(), &blueprint) * blueprint.id)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{day19, fetch_input};

    #[test]
    fn sum_quality_levels() {
        fetch_input(19);
        let tests = vec![("example/day19.txt", 33)];

        for (filename, want) in tests {
            let got = day19::sum_quality_levels(filename);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }
}
