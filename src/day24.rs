use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Write},
    fs,
    ops::Add,
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

    fn neighbours(self) -> [Self; 4] {
        [
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y + 1),
            Point::new(self.x, self.y - 1),
        ]
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
    ground: HashSet<Point>,
    blizzards: HashMap<Point, Vec<Blizzard>>,
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

                if let Some(direction) = direction {
                    blizzards
                        .entry(point)
                        .or_default()
                        .push(Blizzard(direction));
                }
            }
        }

        Self { ground, blizzards }
    }

    fn wrap_around(&self, point: Point, Blizzard(direction): Blizzard) -> Point {
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
        let old_blizzards: HashMap<Point, Vec<Blizzard>> = self.blizzards.drain().collect();
        for (point, blizzards) in old_blizzards {
            for blizzard in blizzards {
                let new_point = point.neighbour(blizzard.0);
                let new_point = if self.ground.contains(&new_point) {
                    new_point
                } else {
                    self.wrap_around(point, blizzard)
                };
                self.blizzards.entry(new_point).or_default().push(blizzard);
            }
        }
    }

    fn end_point(&self) -> Point {
        *self.ground.iter().max_by_key(|pt| (pt.x, pt.y)).unwrap()
    }

    fn start_point(&self) -> Point {
        *self.ground.iter().min_by_key(|pt| (pt.x, pt.y)).unwrap()
    }

    fn fingerprint(&self, current_position: Option<Point>) -> Result<String, std::fmt::Error> {
        let mut buffer = String::new();

        let max_y = self.ground.iter().max_by_key(|point| point.y).unwrap().y;
        let max_x = self.ground.iter().max_by_key(|point| point.x).unwrap().x;

        for x in 0..=max_x {
            for y in 0..=max_y {
                let point = Point::new(x, y);

                if let Some(current_position) = current_position {
                    if current_position == point {
                        write!(buffer, "E")?;
                        continue;
                    }
                }

                if let Some(blizzards) = self.blizzards.get(&point) {
                    assert!(!blizzards.is_empty(), "we have an empty blizzard");
                    match blizzards.len() {
                        0 => write!(buffer, ".")?,
                        1 => write!(buffer, "{}", blizzards[0])?,
                        _ => write!(buffer, "{}", blizzards.len())?,
                    }
                    continue;
                }

                if self.ground.contains(&point) {
                    write!(buffer, ".")?;
                } else {
                    write!(buffer, "#")?;
                }
            }
            writeln!(buffer, "#")?;
        }

        Ok(buffer)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    position: Point,
    valley_id: usize,
}

struct Valleys(Vec<Valley>);

impl Valleys {
    // because the blizzards cycle in a predictable way,
    // there are only a finite number of possible arrangements
    // of blizzards in the valley. We can precalculate all the
    // possible arrangements, instead of calculating it each time.
    fn distinct(mut valley: Valley) -> Self {
        let mut valleys = Vec::new();

        let initial_valley_fingerprint = valley.fingerprint(None);

        valleys.push(valley.clone());

        loop {
            valley.tick();
            let next_valley_fingerprint = valley.fingerprint(None);
            if next_valley_fingerprint == initial_valley_fingerprint {
                break;
            }

            valleys.push(valley.clone());
        }

        Self(valleys)
    }

    fn next_positions(&self, state: State) -> Vec<State> {
        let next_valley_id = state.valley_id.add(1).rem_euclid(self.0.len());
        let next_valley = self.0.get(next_valley_id).unwrap();

        state
            .position
            .neighbours()
            .into_iter()
            .chain([state.position])
            .filter(|pt| next_valley.ground.contains(pt) && !next_valley.blizzards.contains_key(pt))
            .map(|pt| State {
                position: pt,
                valley_id: next_valley_id,
            })
            .collect()
    }
}

// A-star search, using manhattan distance as a heuristic,
// and representing the state as a struct of current position
// and valley index in valleys array.
fn find_shortest_path(valley: Valley) -> isize {
    let start_state = State {
        valley_id: 0,
        position: valley.start_point(),
    };
    let end_point = valley.end_point();

    let valleys = Valleys::distinct(valley);

    let mut queue = queue::MinPriority::default();
    let mut came_from: HashMap<State, Option<State>> = HashMap::new();
    let mut cost_so_far: HashMap<State, isize> = HashMap::new();

    queue.push(start_state, 0);
    came_from.insert(start_state, None);
    cost_so_far.insert(start_state, 0);

    while let Some(current) = queue.pop() {
        if current.position == end_point {
            return *cost_so_far.get(&current).unwrap();
        }

        for next in valleys.next_positions(current) {
            let new_cost = *cost_so_far.get(&current).unwrap() + 1;
            if !came_from.contains_key(&next) || Some(&new_cost) < cost_so_far.get(&next) {
                cost_so_far.insert(next, new_cost);
                came_from.insert(next, Some(current));
                let priority = new_cost + manhattan_distance(current.position, end_point);
                queue.push(next, priority.try_into().unwrap());
            }
        }
    }

    panic!("did not get to the end");
}

fn manhattan_distance(a: Point, b: Point) -> isize {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

pub fn find_shortest_path_through_blizzard(filename: &str) -> isize {
    let input = fs::read_to_string(filename).unwrap();
    let valley = Valley::new(&input);
    find_shortest_path(valley)
}

#[cfg(test)]
mod tests {
    use crate::{day24, fetch_input};

    #[test]
    fn find_shortest_path_through_blizzard() {
        fetch_input(24);
        let tests = vec![("example/day24.txt", 18), ("input/day24.txt", 332)];

        for (filename, want) in tests {
            let got = day24::find_shortest_path_through_blizzard(filename);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }
}
