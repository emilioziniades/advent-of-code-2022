use std::fs;

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
    minutes_left: usize,
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
    fn new(total_minutes: usize) -> Self {
        Self {
            minutes_left: total_minutes,
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
    fn tick(&mut self) {
        self.minutes_left -= 1;
        self.ores += self.orebots;
        self.clays += self.claybots;
        self.obsidians += self.obsidianbots;
        self.geodes += self.geodebots;
    }
}

fn max_geodes(state: &mut State, blueprint: &Blueprint) -> usize {
    // max ores: 147
    loop {
        println!("{state:#?}");

        if state.minutes_left == 0 {
            return state.ores;
        }

        if state.ores >= blueprint.ore_for_orebot {
            //units: ore
            let ore_benefit_from_new_orebot = state.minutes_left - 1;
            let ore_cost_of_new_orebot = blueprint.ore_for_orebot;

            if ore_benefit_from_new_orebot > ore_cost_of_new_orebot {
                state.ores -= blueprint.ore_for_orebot;
                state.orebots += 1;
                state.tick();
                continue;
            }
        }

        // wait
        state.tick();
    }
}

fn quality_level(blueprint: Blueprint) -> usize {
    println!("{blueprint:#?}");
    const MINUTES: usize = 24;
    let mut state = State::new(MINUTES + 1);
    let max_geodes = max_geodes(&mut state, &blueprint);

    max_geodes * blueprint.id
}

pub fn sum_quality_levels(filename: &str) -> usize {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .take(1) // TODO: remove once happy
        .map(Blueprint::new)
        .map(quality_level)
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
