use std::{fmt, fs};

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
enum RockShape {
    Plus,
    Minus,
    L,
    Bar,
    Square,
}

#[derive(Debug)]
struct Rock {
    bottom_left: Point,
    shape: RockShape,
}

impl Rock {
    fn from(shape: RockShape, chamber: &Chamber) -> Self {
        let highest_rock = chamber.highest_rock();
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
        Self { shape, bottom_left }
    }

    fn push(&mut self, chamber: &Chamber, direction: &Direction) {
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

    fn touches(&self, chamber: &Chamber, direction: &Direction) -> bool {
        self.touches_wall(direction) || self.touches_rocks(chamber, direction)
    }

    fn touches_wall(&self, side: &Direction) -> bool {
        match side {
            Direction::Left => match self.shape {
                RockShape::Plus => self.bottom_left.x == 1,
                RockShape::Minus | RockShape::L | RockShape::Square | RockShape::Bar => {
                    self.bottom_left.x == 0
                }
            },
            Direction::Right => match self.shape {
                RockShape::Bar => self.bottom_left.x == 6,
                RockShape::Plus | RockShape::Square => self.bottom_left.x == 5,
                RockShape::L => self.bottom_left.x == 4,
                RockShape::Minus => self.bottom_left.x == 3,
            },
        }
    }

    fn touches_rocks(&self, chamber: &Chamber, direction: &Direction) -> bool {
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

        let (x, y) = (self.bottom_left.x as isize, self.bottom_left.y as isize);
        let bottom_edges = match self.shape {
            RockShape::Bar => vec![(x, y - 1)],
            RockShape::Plus => vec![(x, y - 1), (x - 1, y), (x + 1, y)],
            RockShape::Minus => vec![(x, y - 1), (x + 1, y - 1), (x + 2, y - 1), (x + 3, y - 1)],
            RockShape::L => vec![(x, y - 1), (x + 1, y - 1), (x + 2, y - 1)],
            RockShape::Square => vec![(x, y - 1), (x + 1, y - 1)],
        };

        let bottom_edges = bottom_edges
            .into_iter()
            .map(|(x, y)| (x as usize, y as usize))
            .collect();

        chamber.any(bottom_edges)
    }
}

struct Chamber(Vec<Vec<bool>>);

impl Chamber {
    fn add_empty_rows(&mut self, rock: &RockShape) {
        let n_rows = match rock {
            RockShape::Plus | RockShape::L => 3 + 3,
            RockShape::Minus => 3 + 1,
            RockShape::Bar => 3 + 4,
            RockShape::Square => 3 + 2,
        };

        let mut rows_to_add = (0..n_rows).map(|_| vec![false; 7]).collect();

        self.0.append(&mut rows_to_add);
    }

    fn new() -> Self {
        Self(vec![])
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
                !self.0[y][x],
                "can't insert rock into occupied position {x},{y}"
            );
            self.0[y][x] = true;
        }
    }

    fn any(&self, positions: Vec<(usize, usize)>) -> bool {
        positions.into_iter().map(|(x, y)| self.0[y][x]).any(|b| b)
    }

    fn highest_rock(&self) -> usize {
        self.0
            .iter()
            .position(|row| !row.iter().any(|cell| *cell))
            .unwrap_or(0)
    }
}

impl fmt::Display for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut to_print = vec![String::from("+-------+")];
        for row in &self.0 {
            let row = row
                .iter()
                .map(|cell| match cell {
                    true => "@",
                    false => ".",
                })
                .collect::<Vec<_>>()
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

#[derive(Debug)]
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

pub fn count_tower_height(file: &str, n_rocks: usize) -> Option<usize> {
    let jet_flows = fs::read_to_string(file).expect("file exists");
    let mut jet_flows = jet_flows.trim().chars().map(Direction::from).cycle();

    let rocks = vec![
        RockShape::Minus,
        RockShape::Plus,
        RockShape::L,
        RockShape::Bar,
        RockShape::Square,
    ];
    let mut rocks = rocks.iter().cycle();

    let mut chamber = Chamber::new();

    for _ in 0..n_rocks {
        let rock = rocks.next()?;
        chamber.add_empty_rows(rock);
        let mut rock = Rock::from(*rock, &chamber);
        loop {
            let jet_flow = jet_flows.next()?;
            rock.push(&chamber, &jet_flow);

            if rock.touches_bottom(&chamber) {
                chamber.update(rock);
                break;
            } else {
                rock.fall();
            }
        }
    }
    Some(chamber.highest_rock())
}

#[cfg(test)]
mod tests {
    use crate::{day17, fetch_input};

    #[test]
    fn count_tower_height() {
        fetch_input(17);

        let tests = vec![("example/day17.txt", 3068), ("input/day17.txt", 3184)];

        for test in tests {
            let (file, want) = test;
            let got = day17::count_tower_height(file, 2022).unwrap();
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }

    // #[test]
    #[allow(dead_code)]
    fn count_very_tall_tower_height() {
        fetch_input(17);

        let tests = vec![
            ("example/day17.txt", 15142857142881),
            ("input/day17.txt", 0),
        ];

        for test in tests {
            let (file, want) = test;
            let got = day17::count_tower_height(file, 1_000_000_000_000).unwrap();
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }
}
