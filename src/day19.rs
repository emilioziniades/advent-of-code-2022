// Credit where credit is due: https://todd.ginsberg.com/post/advent-of-code/2022/day19/
// I was getting lost in the combinations again. What this article showed me was that you
// don't need to consider any *intermediate* states. Just consider states that changed.
// Practically this means branching at every possible build opportunity, instead of branching
// at every single time step. This means 'wait' no longer becomes an action, and the branching
// factor is much much smaller. That's the big idea, and then there are a few smaller
// optimizations, like knowing when to stop producing a certain kind of robot, since you can only
// ever make one robot a minute. Another optimization is generating an upper bound on each
// estimate. Generally, the lesson I took from this one, after being stuck on it for way too long,
// stubbornly, before seeking out some clues, is 1) your initial instinct is generally correct,
// 2) the algorithm is not so complicated - you aren't going to find the magic answer in a journal
// article from 2007 and 3) consider the domain very specifically and don't over generalize the
// problem.
use std::fs;

#[derive(Debug)]
struct Blueprint {
    id: isize,
    ore_for_orebot: isize,
    ore_for_claybot: isize,
    ore_for_obsidianbot: isize,
    clay_for_obsidianbot: isize,
    ore_for_geodebot: isize,
    obsidian_for_geodebot: isize,
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
    elapsed_minutes: isize,
    ores: isize,
    clays: isize,
    obsidians: isize,
    geodes: isize,
    orebots: isize,
    claybots: isize,
    obsidianbots: isize,
    geodebots: isize,
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

    fn tick(&mut self, minutes: isize) {
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

    /// build a new geodebot every minute until the end
    fn maximum_possible_geodes(&self, minutes: isize) -> isize {
        let minutes_left = minutes - self.elapsed_minutes;
        self.geodes + (0..minutes_left).map(|t| t + self.geodebots).sum::<isize>()
    }

    fn projected_geodes(&self, minutes: isize) -> isize {
        self.geodes + (minutes - self.elapsed_minutes) * self.geodebots
    }
}

fn max_geodes(state: State, blueprint: &Blueprint, minutes: isize) -> isize {
    let mut queue: Vec<State> = Vec::new();
    let mut max_geodes = 0;
    queue.push(state);

    let max_orebots = blueprint
        .ore_for_orebot
        .max(blueprint.ore_for_claybot)
        .max(blueprint.ore_for_obsidianbot)
        .max(blueprint.ore_for_geodebot);
    let max_claybots = blueprint.clay_for_obsidianbot;
    let max_obsidianbots = blueprint.obsidian_for_geodebot;

    while let Some(state) = queue.pop() {
        if state.maximum_possible_geodes(minutes) < max_geodes {
            // no point in pursuing this state further
            continue;
        }

        max_geodes = max_geodes.max(state.projected_geodes(minutes));

        // build orebot
        if state.orebots < max_orebots {
            let mut state = state;
            let minutes_left =
                div_ceil(blueprint.ore_for_orebot - state.ores, state.orebots).max(0);
            state.tick(minutes_left + 1);
            state.build_orebot(blueprint);
            if state.elapsed_minutes < minutes {
                queue.push(state);
            }
        }

        // build claybot
        if state.claybots < max_claybots {
            let mut state = state;
            let minutes_left =
                div_ceil(blueprint.ore_for_claybot - state.ores, state.orebots).max(0);
            state.tick(minutes_left + 1);
            state.build_claybot(blueprint);
            if state.elapsed_minutes < minutes {
                queue.push(state);
            }
        }

        // build obsidianbots
        if state.obsidianbots < max_obsidianbots && state.claybots > 0 {
            let mut state = state;
            let minutes_until_ore =
                div_ceil(blueprint.ore_for_obsidianbot - state.ores, state.orebots);
            let minutes_until_clay =
                div_ceil(blueprint.clay_for_obsidianbot - state.clays, state.claybots);
            let minutes_left = minutes_until_ore.max(minutes_until_clay).max(0);
            state.tick(minutes_left + 1);
            state.build_obsidianbot(blueprint);
            if state.elapsed_minutes < minutes {
                queue.push(state);
            }
        }

        // build geodebots
        if state.obsidianbots > 0 {
            let mut state = state;
            let minutes_until_ore =
                div_ceil(blueprint.ore_for_geodebot - state.ores, state.orebots);
            let minutes_until_obsidian = div_ceil(
                blueprint.obsidian_for_geodebot - state.obsidians,
                state.obsidianbots,
            );
            let minutes_left = minutes_until_ore.max(minutes_until_obsidian).max(0);
            state.tick(minutes_left + 1);
            state.build_geodebot(blueprint);
            if state.elapsed_minutes < minutes {
                queue.push(state);
            }
        }
    }

    max_geodes
}

fn div_ceil(a: isize, b: isize) -> isize {
    (a + b - 1) / b
}

pub fn sum_quality_levels(filename: &str) -> isize {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(Blueprint::new)
        .map(|blueprint| max_geodes(State::new(), &blueprint, 24) * blueprint.id)
        .sum()
}

pub fn multiply_first_three_blueprints(filename: &str) -> isize {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .take(3)
        .map(Blueprint::new)
        .map(|blueprint| max_geodes(State::new(), &blueprint, 32))
        .product()
}

#[cfg(test)]
mod tests {
    use crate::{day19, fetch_input};

    #[test]
    fn sum_quality_levels() {
        fetch_input(19);
        let tests = vec![("example/day19.txt", 33), ("input/day19.txt", 1192)];

        for (filename, want) in tests {
            let got = day19::sum_quality_levels(filename);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }

    #[test]
    fn multiply_first_three_blueprints() {
        fetch_input(19);
        let tests = vec![("example/day19.txt", 3472), ("input/day19.txt", 14725)];

        for (filename, want) in tests {
            let got = day19::multiply_first_three_blueprints(filename);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }
}
