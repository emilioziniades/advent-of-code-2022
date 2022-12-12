use std::{collections::HashSet, fs};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Movement {
    steps: u8,
    direction: Direction,
}

impl Movement {
    fn from(line: &str) -> Self {
        let words = line.split_whitespace().collect::<Vec<&str>>();

        let (direction, steps) = match words[..] {
            [direction, steps] => (direction, steps.parse().unwrap()),
            [..] => panic!("unrecognized command {words:?}"),
        };

        let direction = match direction {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "L" => Direction::Left,
            "D" => Direction::Down,
            _ => panic!("unrecognized direction {direction}"),
        };

        Movement { steps, direction }
    }
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Point>,
}

impl Rope {
    fn new(n_knots: u32) -> Self {
        Rope {
            knots: (0..n_knots).map(|_| Point::new(0, 0)).collect(),
        }
    }

    fn update_knot_pair(&mut self, ahead_ix: usize, behind_ix: usize) {
        if self.knots[behind_ix].is_touching(&self.knots[ahead_ix]) {
            return;
        }

        let Point { x: x_h, y: y_h } = self.knots[ahead_ix];
        let Point { x: x_t, y: y_t } = self.knots[behind_ix];

        match (x_h == x_t, y_h == y_t) {
            // overlapping
            (true, true) => (),

            // horizontally aligned
            (true, false) => {
                if y_h > y_t {
                    self.knots[behind_ix].y += 1
                } else {
                    self.knots[behind_ix].y -= 1
                }
            }

            // vertically aligned
            (false, true) => {
                if x_h > x_t {
                    self.knots[behind_ix].x += 1
                } else {
                    self.knots[behind_ix].x -= 1
                }
            }

            // diagonal movement needed
            (false, false) => {
                if x_h > x_t {
                    self.knots[behind_ix].x += 1
                } else {
                    self.knots[behind_ix].x -= 1
                }

                if y_h > y_t {
                    self.knots[behind_ix].y += 1
                } else {
                    self.knots[behind_ix].y -= 1
                }
            }
        };
    }

    fn update(&mut self, movement: Movement, tail_positions: &mut HashSet<Point>) {
        for _ in 0..movement.steps {
            // update head
            match movement.direction {
                Direction::Left => self.knots[0].x -= 1,
                Direction::Right => self.knots[0].x += 1,
                Direction::Up => self.knots[0].y += 1,
                Direction::Down => self.knots[0].y -= 1,
            }

            // update tails
            let indexes: Vec<usize> = (0..self.knots.len()).collect();
            for knot_pair_index in indexes.windows(2) {
                let (ahead, behind) = (knot_pair_index[0], knot_pair_index[1]);
                self.update_knot_pair(ahead, behind);
                tail_positions.insert(*self.knots.last().unwrap());
            }
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn is_touching(&self, other: &Point) -> bool {
        let Point { x: x_h, y: y_h } = self;
        let Point { x: x_t, y: y_t } = other;

        *x_t <= x_h + 1 && *x_t >= x_h - 1 && *y_t <= y_h + 1 && *y_t >= y_h - 1
    }
}

pub fn count_tail_positions(filename: &str, n_ropes: u32) -> usize {
    let movements = fs::read_to_string(filename).unwrap();
    let movements: Vec<Movement> = movements.lines().map(Movement::from).collect();
    let mut rope = Rope::new(n_ropes);
    let mut tail_positions: HashSet<Point> = HashSet::new();

    tail_positions.insert(*rope.knots.last().unwrap());

    for movement in movements {
        rope.update(movement, &mut tail_positions);
    }

    tail_positions.len()
}

#[cfg(test)]
mod tests {
    use crate::day09;
    use crate::fetch_input;

    #[test]
    fn count_tail_positions() {
        fetch_input(9);
        let tests = vec![
            ("example/day09.txt", 13, 2),
            ("input/day09.txt", 6181, 2),
            ("example/day09.txt", 1, 10),
            ("example/day09_2.txt", 36, 10),
            ("input/day09.txt", 2386, 10),
        ];

        for test in tests {
            let (filename, want, n_ropes) = test;
            let got = day09::count_tail_positions(filename, n_ropes);

            assert_eq!(got, want, "got {got}, wanted {want}, for {filename}");
        }
    }
}
