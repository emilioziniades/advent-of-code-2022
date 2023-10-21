use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs,
};

use crate::queue;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn neighbour(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Point::new(self.x - 1, self.y),
            Direction::Right => Point::new(self.x, self.y + 1),
            Direction::Down => Point::new(self.x + 1, self.y),
            Direction::Left => Point::new(self.x, self.y - 1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Blizzard(Direction);

impl Display for Blizzard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let character = match self {
            Blizzard(Direction::Up) => "^",
            Blizzard(Direction::Right) => ">",
            Blizzard(Direction::Down) => "v",
            Blizzard(Direction::Left) => "<",
        };

        write!(f, "{character}")?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Valley {
    position: Point,
    ground: HashSet<Point>,
    blizzards: HashMap<Point, Vec<Blizzard>>,
    end_point: Point,
    minutes: usize,
}

impl Valley {
    fn new(input: &str) -> Self {
        let mut ground: HashSet<Point> = HashSet::new();
        let mut blizzards: HashMap<Point, Vec<Blizzard>> = HashMap::new();

        for (x, line) in input.lines().enumerate() {
            for (y, character) in line.trim().chars().enumerate() {
                let point = Point::new(x.try_into().unwrap(), y.try_into().unwrap());
                if character != '#' {
                    ground.insert(point);
                }

                let direction = match character {
                    '>' => Some(Direction::Right),
                    '<' => Some(Direction::Left),
                    'v' => Some(Direction::Down),
                    '^' => Some(Direction::Up),
                    _ => None,
                };

                // TODO: sort Vec<Blizzard> so that it is represented consistently for DP later
                if let Some(direction) = direction {
                    blizzards
                        .entry(point)
                        .or_default()
                        .push(Blizzard(direction));
                }
            }
        }

        let position = *ground
            .iter()
            .min_by_key(|point| (point.x, point.y))
            .unwrap();

        let end_point = *ground.iter().max_by_key(|pt| pt.x).unwrap();

        Self {
            position,
            ground,
            blizzards,
            end_point,
            minutes: 0,
        }
    }

    fn wrap_around(&self, point: Point, Blizzard(direction): &Blizzard) -> Point {
        match direction {
            Direction::Up => Point::new(
                self.ground
                    .iter()
                    .filter(|pt| pt.y == point.y)
                    .max_by_key(|pt| pt.x)
                    .unwrap()
                    .x,
                point.y,
            ),
            Direction::Right => Point::new(
                point.x,
                self.ground
                    .iter()
                    .filter(|pt| pt.x == point.x)
                    .min_by_key(|pt| pt.y)
                    .unwrap()
                    .y,
            ),
            Direction::Down => Point::new(
                self.ground
                    .iter()
                    .filter(|pt| pt.y == point.y)
                    .min_by_key(|pt| pt.x)
                    .unwrap()
                    .x,
                point.y,
            ),
            Direction::Left => Point::new(
                point.x,
                self.ground
                    .iter()
                    .filter(|pt| pt.x == point.x)
                    .max_by_key(|pt| pt.y)
                    .unwrap()
                    .y,
            ),
        }
    }

    fn tick(&mut self) {
        self.minutes += 1;
        let old_blizzards: HashMap<_, _> = self.blizzards.drain().collect();
        for (point, blizzards) in old_blizzards {
            for blizzard in blizzards {
                let new_point = point.neighbour(blizzard.0);
                let new_point = if self.ground.contains(&new_point) {
                    new_point
                } else {
                    self.wrap_around(point, &blizzard.clone())
                };
                self.blizzards.entry(new_point).or_default().push(blizzard);
            }
        }

        for (_point, blizzards) in self.blizzards.iter_mut() {
            blizzards.sort();
        }
    }

    fn neighbours(&self) -> [Point; 4] {
        [
            Point::new(self.position.x - 1, self.position.y),
            Point::new(self.position.x + 1, self.position.y),
            Point::new(self.position.x, self.position.y + 1),
            Point::new(self.position.x, self.position.y - 1),
        ]
    }

    fn next_positions(&self) -> Vec<Point> {
        let mut next_positions: Vec<Point> = self
            .neighbours()
            .into_iter()
            .filter(|pt| self.ground.contains(pt) && !self.blizzards.contains_key(pt))
            .collect();
        next_positions.push(self.position);

        next_positions
    }
}

impl Display for Valley {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_y = self.ground.iter().max_by_key(|point| point.y).unwrap().y;
        let max_x = self.ground.iter().max_by_key(|point| point.x).unwrap().x;

        for x in 0..=max_x {
            for y in 0..=max_y {
                let point = Point::new(x, y);

                if self.position == point {
                    write!(f, "E",)?;
                    continue;
                }

                if let Some(blizzards) = self.blizzards.get(&point) {
                    match blizzards.len() {
                        0 => write!(f, ".")?,
                        1 => write!(f, "{}", blizzards[0])?,
                        _ => write!(f, "{}", blizzards.len())?,
                    }
                    continue;
                }

                if !self.ground.contains(&point) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "#")?;
        }

        Ok(())
    }
}

fn find_shortest_path(valley: Valley) -> usize {
    let mut queue = queue::Priority::new();
    queue.push(valley.clone(), 0);
    let mut came_from: HashMap<(Point, String), Option<(Point, String)>> = HashMap::new();
    let mut cost_so_far: HashMap<(Point, String), isize> = HashMap::new();

    let start = (valley.position, format!("{:?}", valley.blizzards));

    came_from.insert(start.clone(), None);
    cost_so_far.insert(start, 0);

    while let Some(mut valley) = queue.pop() {
        // println!("{valley}");

        if valley.position == valley.end_point {
            return valley.minutes + 1;
        }

        let current = (valley.position, format!("{:?}", valley.blizzards));

        valley.tick();

        let next_valley = format!("{:?}", valley.blizzards);

        for next_point in valley.next_positions() {
            let next = (next_point, next_valley.clone());
            let new_cost = *cost_so_far.get(&current).unwrap() + 1;
            if !came_from.contains_key(&next) || Some(&new_cost) < cost_so_far.get(&next) {
                let mut valley = valley.clone();
                cost_so_far.insert(next.clone(), new_cost);
                let priority = new_cost + manhattan_distance(current.0, valley.end_point);
                valley.position = next_point;
                queue.push(valley.clone(), priority.try_into().unwrap());
                came_from.insert(current.clone(), Some(next));
            }
        }

        // queue.push_back(valley.clone());
    }

    panic!("did not get to the end");
}

fn manhattan_distance(current: Point, end_point: Point) -> isize {
    (current.x - end_point.x).abs() + (current.y - end_point.y).abs()
}

pub fn find_shortest_path_through_blizzard(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();
    let valley = Valley::new(&input);
    // println!("initial: \n{valley}\n");
    //
    // for i in 0..19 {
    //     valley.tick();
    //     println!("{}: \n{valley}\n", i + 1);
    // }

    find_shortest_path(valley)
}

#[cfg(test)]
mod tests {
    use crate::{day24, fetch_input};

    #[test]
    fn find_shortest_path_through_blizzard() {
        fetch_input(24);
        let tests = vec![("example/day24.txt", 18), ("input/day24.txt", 0)];

        for (filename, want) in tests {
            let got = day24::find_shortest_path_through_blizzard(filename);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }
}
