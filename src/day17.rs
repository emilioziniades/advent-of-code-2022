use std::{fmt, fs, iter::Cycle, slice::Iter};

const TEST_CHAMBER_SIZE: usize = 10_000;

const ROCK_ORDER: [RockShape; 5] = [
    RockShape::Minus,
    RockShape::Plus,
    RockShape::L,
    RockShape::Bar,
    RockShape::Square,
];

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
enum RockShape {
    Minus,
    Plus,
    L,
    Bar,
    Square,
}

#[derive(Debug, Clone, Copy)]
struct Rock {
    bottom_left: Point,
    shape: RockShape,
}

impl Rock {
    fn from(shape: RockShape, chamber: &Chamber) -> Self {
        let highest_rock = chamber.height();
        let bottom_left = match shape {
            RockShape::Bar | RockShape::Minus | RockShape::L | RockShape::Square => Point {
                x: 2,
                y: highest_rock + 3,
            },
            RockShape::Plus => Point {
                x: 3,
                y: highest_rock + 3,
            },
        };
        Self { bottom_left, shape }
    }

    fn push(&mut self, chamber: &Chamber, direction: Direction) {
        if !self.touches(chamber, direction) {
            match direction {
                Direction::Left => self.bottom_left.x -= 1,
                Direction::Right => self.bottom_left.x += 1,
            }
        }
    }

    fn fall(&mut self) {
        self.bottom_left.y -= 1;
    }

    fn touches(&self, chamber: &Chamber, direction: Direction) -> bool {
        self.touches_wall(direction) || self.touches_rocks(chamber, direction)
    }

    fn touches_wall(&self, side: Direction) -> bool {
        let (x, _) = (self.bottom_left.x, self.bottom_left.y);
        match side {
            Direction::Left => match self.shape {
                RockShape::Plus => x == 1,
                RockShape::Minus | RockShape::L | RockShape::Square | RockShape::Bar => x == 0,
            },
            Direction::Right => match self.shape {
                RockShape::Bar => x == 6,
                RockShape::Plus | RockShape::Square => x == 5,
                RockShape::L => x == 4,
                RockShape::Minus => x == 3,
            },
        }
    }

    fn touches_rocks(&self, chamber: &Chamber, direction: Direction) -> bool {
        let (x, y) = (self.bottom_left.x, self.bottom_left.y);
        match direction {
            Direction::Left => {
                let left_edges = match self.shape {
                    RockShape::Bar => {
                        vec![(x - 1, y), (x - 1, y + 1), (x - 1, y + 2), (x - 1, y + 3)]
                    }
                    RockShape::Plus => {
                        vec![(x - 1, y), (x - 2, y + 1), (x - 1, y + 2)]
                    }
                    RockShape::Minus => vec![(x - 1, y)],
                    RockShape::L => vec![(x - 1, y), (x + 1, y + 1), (x + 1, y + 2)],
                    RockShape::Square => vec![(x - 1, y), (x - 1, y + 1)],
                };

                chamber.any(left_edges)
            }
            Direction::Right => {
                let right_edges = match self.shape {
                    RockShape::Bar => {
                        vec![(x + 1, y), (x + 1, y + 1), (x + 1, y + 2), (x + 1, y + 3)]
                    }
                    RockShape::Plus => {
                        vec![(x + 1, y), (x + 2, y + 1), (x + 1, y + 2)]
                    }
                    RockShape::Minus => vec![(x + 4, y)],
                    RockShape::L => vec![(x + 3, y), (x + 3, y + 1), (x + 3, y + 2)],
                    RockShape::Square => vec![(x + 2, y), (x + 2, y + 1)],
                };

                chamber.any(right_edges)
            }
        }
    }

    fn touches_bottom(&self, chamber: &Chamber) -> bool {
        if self.bottom_left.y == 0 {
            return true;
        }

        let (x, y) = (self.bottom_left.x, self.bottom_left.y);
        let bottom_edges = match self.shape {
            RockShape::Bar => vec![(x, y - 1)],
            RockShape::Plus => vec![(x, y - 1), (x - 1, y), (x + 1, y)],
            RockShape::Minus => vec![(x, y - 1), (x + 1, y - 1), (x + 2, y - 1), (x + 3, y - 1)],
            RockShape::L => vec![(x, y - 1), (x + 1, y - 1), (x + 2, y - 1)],
            RockShape::Square => vec![(x, y - 1), (x + 1, y - 1)],
        };

        let bottom_edges = bottom_edges.into_iter().collect();

        chamber.any(bottom_edges)
    }
}

#[derive(Default)]
struct Chamber {
    columns: Vec<[bool; 7]>,
}

impl Chamber {
    fn add_empty_rows(&mut self) {
        // 3 rows above highest rock
        // +3 for plus and L
        // +1 for minus
        // +4 for bar
        // +2 for square
        // just add 7 to be safe
        let n_rows = 7;

        let mut rows_to_add = (0..n_rows).map(|_| [false; 7]).collect();

        self.columns.append(&mut rows_to_add);
    }

    fn update(&mut self, rock: Rock) {
        let (x, y) = (rock.bottom_left.x, rock.bottom_left.y);
        // fill in array based on rock location
        let positions = match rock.shape {
            RockShape::Plus => vec![
                (x, y),
                (x, y + 1),
                (x, y + 2),
                (x - 1, y + 1),
                (x + 1, y + 1),
            ],
            RockShape::Minus => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            RockShape::L => vec![
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ],
            RockShape::Bar => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            RockShape::Square => vec![(x, y), (x + 1, y), (x + 1, y + 1), (x, y + 1)],
        };

        for (x, y) in positions {
            assert!(x < 7, "can't insert rock outside of chamber {x},{y}");
            assert!(
                !self.columns[y][x],
                "can't insert rock into occupied position {x},{y}"
            );
            self.columns[y][x] = true;
        }

        // remove empty rows
        let last_rock_row = self
            .columns
            .iter()
            .rposition(|row| row.iter().any(|cell| *cell))
            .unwrap();

        self.columns.drain(last_rock_row + 1..);
    }

    fn any(&self, positions: Vec<(usize, usize)>) -> bool {
        positions
            .into_iter()
            .map(|(x, y)| self.columns[y][x])
            .any(|b| b)
    }

    fn insert_rock(
        &mut self,
        jet_flows: &mut Cycle<Iter<Direction>>,
        rocks: &mut Cycle<Iter<RockShape>>,
    ) {
        let rock = rocks.next().unwrap();
        let mut rock = Rock::from(*rock, self);
        self.add_empty_rows();
        loop {
            let jet_flow = jet_flows.next().unwrap();
            rock.push(self, *jet_flow);

            if rock.touches_bottom(self) {
                self.update(rock);
                break;
            }

            rock.fall();
        }
    }

    fn height(&self) -> usize {
        self.columns.len()
    }
}

impl fmt::Display for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut to_print = vec![String::from("+-------+")];
        for row in &self.columns {
            let row = row
                .iter()
                .map(|cell| match cell {
                    true => "@",
                    false => ".",
                })
                .collect::<Vec<&str>>()
                .join("");

            let row = "|".to_string() + &row + "|";
            to_print.push(row);
        }
        for line in to_print.iter().rev() {
            writeln!(f, "{line}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            other => panic!("unrecognized char {other}"),
        }
    }
}

fn fill_chamber(jet_flows: &[Direction], n_rocks: usize) -> Chamber {
    let mut chamber = Chamber::default();
    let mut jet_flows = jet_flows.iter().cycle();
    let mut rocks = ROCK_ORDER.iter().cycle();

    for _ in 0..n_rocks {
        chamber.insert_rock(&mut jet_flows, &mut rocks);
    }

    chamber
}

fn rocks_until_height(jet_flows: &[Direction], height: usize) -> usize {
    let mut chamber = Chamber::default();
    let mut jet_flows = jet_flows.iter().cycle();
    let mut rocks = ROCK_ORDER.iter().cycle();

    let mut n_rocks = 1;
    while chamber.height() < height {
        chamber.insert_rock(&mut jet_flows, &mut rocks);
        n_rocks += 1;
    }

    n_rocks
}

fn detect_cycle_period(chamber: &Chamber) -> Option<usize> {
    for cycle_period in 4..=chamber.height() {
        let pairs: Vec<&[[bool; 7]]> = chamber.columns.chunks_exact(cycle_period).collect();
        for (i, pair) in pairs.windows(2).enumerate() {
            let (first, second) = (pair[0], pair[1]);
            if first == second {
                println!("CYCLE!: window size {cycle_period} at window {i} starting from index 0");
                return Some(cycle_period);
            }
        }
    }
    None
}

pub fn count_tower_height(file: &str, n_rocks: usize) -> usize {
    let jet_flows = fs::read_to_string(file).unwrap();
    let jet_flows: Vec<Direction> = jet_flows.trim().chars().map(Direction::from).collect();
    let chamber = fill_chamber(&jet_flows, n_rocks);
    chamber.height()
}

pub fn count_very_tall_tower_height(file: &str, n_rocks: usize) -> usize {
    let jet_flows = fs::read_to_string(file).unwrap();
    let jet_flows: Vec<Direction> = jet_flows.trim().chars().map(Direction::from).collect();

    let chamber = fill_chamber(&jet_flows, TEST_CHAMBER_SIZE);
    let cycle_period = detect_cycle_period(&chamber).unwrap();

    // for some weird reason, the repeats only occur after one cycle. So
    // cycle 0 != cycle 1, but cycle 1 == cycle 2, cycle 2 == cycle 3, etc...
    // So first we calculate the rocks in the first cycle
    let rocks_before_cycles = rocks_until_height(&jet_flows, cycle_period);

    // And the rocks in the next cycle (which will repeat indefinitely)
    let rocks_after_one_cycle = rocks_until_height(&jet_flows, cycle_period * 2);
    let rocks_in_one_cycle = rocks_after_one_cycle - rocks_before_cycles;

    // See how many full cycles will occur
    let rocks_left = n_rocks - rocks_before_cycles;
    let cycles = rocks_left / rocks_in_one_cycle;
    let rocks_in_cycles = rocks_in_one_cycle * cycles;
    let rocks_after_cycles = n_rocks - rocks_in_cycles - rocks_before_cycles;

    // get height of last unfinished cycle
    // a little hacky but it's fine for now
    let chamber = fill_chamber(
        &jet_flows,
        rocks_before_cycles + rocks_in_one_cycle + rocks_after_cycles,
    );
    let unfinished_cycle_height = chamber.height() - cycle_period * 2;

    cycle_period + cycle_period * cycles + unfinished_cycle_height
}

#[cfg(test)]
mod tests {
    use crate::{day17, fetch_input};

    #[test]
    fn count_tower_height() {
        fetch_input(17);

        let n_rocks = 2022;

        let tests = vec![("example/day17.txt", 3068), ("input/day17.txt", 3184)];

        for test in tests {
            let (file, want) = test;
            let got = day17::count_tower_height(file, n_rocks);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }

    #[test]
    fn count_very_tall_tower_height() {
        fetch_input(17);

        let n_rocks = 1_000_000_000_000;

        let tests = vec![
            ("example/day17.txt", 1514285714288),
            ("input/day17.txt", 1577077363915),
        ];

        for test in tests {
            let (file, want) = test;
            let got = day17::count_very_tall_tower_height(file, n_rocks);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }
}
