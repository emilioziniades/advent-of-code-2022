use std::collections::{HashMap, VecDeque};
use std::fs;

const MINUTES: usize = 24;

const ALL_RESOURCES: [Resource; 4] = [
    Resource::Geode,
    Resource::Obsidian,
    Resource::Clay,
    Resource::Ore,
];
/*

impl Factory {

    fn next_action(&self) -> Option<Robot> {
        let mut possible_actions = self.possible_next_actions();

        if possible_actions.contains(&Robot::Clay) {
            let clay_ore_cost_ratio_for_obsidian_robot =
                self.blueprint.obsidian_robot.clay_cost / self.blueprint.obsidian_robot.ore_cost;

            let clay_ore_robot_ratio = self.n_clay_robots / self.n_ore_robots;

            // we have enough clay robots, remove it as an option
            if dbg!(clay_ore_robot_ratio) >= dbg!(clay_ore_cost_ratio_for_obsidian_robot) {
                println!("we have enough clay robots, don't consider it an option");
                possible_actions.retain(|action| action != &Robot::Clay);
            }
        }

        if possible_actions.contains(&Robot::Obsidian) {
            let obsidian_ore_cost_ratio_for_geode_robot =
                self.blueprint.geode_robot.obsidian_cost / self.blueprint.geode_robot.ore_cost;

            let obsidian_ore_robot_ratio = self.n_obsidian_robots / self.n_ore_robots;

            // we have enough obsidian robots, remove it as an option
            if dbg!(obsidian_ore_robot_ratio) >= dbg!(obsidian_ore_cost_ratio_for_geode_robot) {
                println!("we have enough obsidian robots, don't consider it an option");
                possible_actions.retain(|action| action != &Robot::Obsidian);
            }
        }

        let next_action = possible_actions.into_iter().next();
        println!("next action: {next_action:?}");
        next_action
    }
}

*/

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Robot(Resource);

#[derive(Debug)]
struct Cost {
    resource: Resource,
    amount: usize,
}

impl Cost {
    fn new(resource: Resource, amount: usize) -> Self {
        Cost { resource, amount }
    }
}

type Costs = Vec<Cost>;

#[derive(Debug)]
struct Blueprint {
    id: usize,
    costs: HashMap<Robot, Costs>,
}

impl Blueprint {
    fn from_line(line: &str) -> Self {
        use Resource::*;

        let line = line
            .to_string()
            .replace([':', '.'], "")
            .replace("Blueprint", "")
            .replace("Each", "")
            .replace("robot", "")
            .replace("costs", "")
            .replace("ore", "")
            .replace("clay", "")
            .replace("obsidian", "")
            .replace("geode", "")
            .replace("robot", "")
            .replace("and", "");

        let mut numbers = line.split_whitespace().map(|n| n.parse::<usize>().unwrap());

        Self {
            id: numbers.next().unwrap(),
            costs: HashMap::from([
                (Robot(Ore), vec![Cost::new(Ore, numbers.next().unwrap())]),
                (Robot(Clay), vec![Cost::new(Ore, numbers.next().unwrap())]),
                (
                    Robot(Obsidian),
                    vec![
                        Cost::new(Ore, numbers.next().unwrap()),
                        Cost::new(Clay, numbers.next().unwrap()),
                    ],
                ),
                (
                    Robot(Geode),
                    vec![
                        Cost::new(Ore, numbers.next().unwrap()),
                        Cost::new(Obsidian, numbers.next().unwrap()),
                    ],
                ),
            ]),
        }
    }

    fn quality_level(self) -> usize {
        let id = self.id;
        let max_geodes = max_possible_geodes(self);
        id * max_geodes
    }
}

type Resources = HashMap<Resource, usize>;

type Robots = HashMap<Robot, usize>;

#[derive(Debug, Clone)]
struct Factory {
    resources: Resources,
    robots: Robots,
    time: usize,
}

impl Factory {
    fn new() -> Self {
        Self {
            resources: HashMap::from([
                (Resource::Ore, 0),
                (Resource::Clay, 0),
                (Resource::Obsidian, 0),
                (Resource::Geode, 0),
            ]),
            robots: HashMap::from([
                (Robot(Resource::Ore), 1),
                (Robot(Resource::Clay), 0),
                (Robot(Resource::Obsidian), 0),
                (Robot(Resource::Geode), 0),
            ]),
            time: 1,
        }
    }

    fn gather_resources(&mut self) {
        for resource in ALL_RESOURCES {
            let resource_count = self.resources.get_mut(&resource).unwrap();
            let robots_count = self.robots.get(&Robot(resource)).unwrap();
            *resource_count += robots_count;
        }
    }

    fn update_robots(&mut self, new_robot: &Option<Robot>) {
        if let Some(new_robot) = new_robot {
            let robots_count = self.robots.get_mut(new_robot).unwrap();
            *robots_count += 1;
        }
    }

    fn possible_next_actions(&self, blueprint: &Blueprint) -> Vec<Robot> {
        let mut possible_actions = Vec::new();
        for resource in ALL_RESOURCES {
            let robot = Robot(resource);
            let costs = blueprint.costs.get(&robot).unwrap();
            if self.is_affordable(costs) && robot != Robot(Resource::Ore) {
                possible_actions.push(robot);
            }
        }

        // println!("possible next actions: {possible_actions:?}");

        possible_actions
    }

    fn is_affordable(&self, costs: &Costs) -> bool {
        for cost in costs {
            let resource_count = self.resources.get(&cost.resource).unwrap();
            if resource_count < &cost.amount {
                return false;
            }
        }
        true
    }

    /* fn next_action(&self) -> Option<Robot> {
        let possible_actions = self.possible_next_actions();
        // TODO: be clever about what to build
        let next_action = possible_actions.into_iter().next();
        println!("next action: {next_action:?}");
        next_action
    } */

    fn spend_resources(&mut self, new_robot: &Option<Robot>, blueprint: &Blueprint) {
        if let Some(new_robot) = new_robot {
            let costs = blueprint.costs.get(new_robot).unwrap();
            for cost in costs {
                let resource_count = self.resources.get_mut(&cost.resource).unwrap();
                *resource_count -= cost.amount;
            }
        }
    }

    fn tick(&mut self, new_robot: Option<Robot>, blueprint: &Blueprint) {
        self.spend_resources(&new_robot, blueprint);
        self.gather_resources();
        self.update_robots(&new_robot);
        self.time += 1;
    }
}

fn max_possible_geodes(blueprint: Blueprint) -> usize {
    println!("{blueprint:?}");
    let mut factories = VecDeque::from([Factory::new()]);

    let mut finished_factories = Vec::new();

    while let Some(mut factory) = factories.pop_back() {
        println!("{}", factory.time);
        if factory.time > MINUTES {
            finished_factories.push(factory);
            continue;
        }

        for next_action in factory.possible_next_actions(&blueprint) {
            let mut new_factory = factory.clone();
            new_factory.tick(Some(next_action), &blueprint);
            factories.push_front(new_factory);
        }

        // we can also do nothing
        factory.tick(None, &blueprint);
        factories.push_front(factory);
    }

    finished_factories
        .into_iter()
        .map(|factory| *factory.resources.get(&Resource::Geode).unwrap())
        .max()
        .unwrap()
    /*
    for i in 1..=MINUTES {
        println!("\ntime: {i}");
        let next_action = factory.possible_next_actions(&blueprint).into_iter().nth(1);
        factory.tick(&next_action, &blueprint);
    }
    println!("{factory:#?}"); */

    // *factory.resources.get(&Resource::Geode).unwrap()
}

pub fn sum_quality_level(file: &str) -> usize {
    fs::read_to_string(file)
        .unwrap()
        .lines()
        .take(1) // TODO: REMOVE ONCE DONE TESTING
        .map(Blueprint::from_line)
        .map(Blueprint::quality_level)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{day19, fetch_input};

    #[test]
    #[ignore = "to revisit"]
    fn sum_quality_level() {
        fetch_input(19);
        let tests = vec![
            ("example/day19.txt", 33), /* ("input/day19.txt", 3412) */
        ];

        for (infile, want) in tests {
            let got = day19::sum_quality_level(infile);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }
}
