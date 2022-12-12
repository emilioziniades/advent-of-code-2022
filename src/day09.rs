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
    head: Point,
    tail: Point,
}

impl Rope {
    fn new() -> Self {
        Rope {
            head: Point::new(0, 0),
            tail: Point::new(0, 0),
        }
    }

    fn update(&mut self, movement: Movement, tail_positions: &mut HashSet<Point>) {
        for _ in 0..movement.steps {
            // update head
            match movement.direction {
                Direction::Left => self.head.x -= 1,
                Direction::Right => self.head.x += 1,
                Direction::Up => self.head.y += 1,
                Direction::Down => self.head.y -= 1,
            }

            if self.are_touching() {
                continue;
            }

            // update tail
            let Point { x: x_h, y: y_h } = self.head;
            let Point { x: x_t, y: y_t } = self.tail;

            match (x_h == x_t, y_h == y_t) {
                // overlapping
                (true, true) => (),

                // horizontally aligned
                (true, false) => {
                    if y_h > y_t {
                        self.tail.y += 1
                    } else {
                        self.tail.y -= 1
                    }
                }

                // vertically aligned
                (false, true) => {
                    if x_h > x_t {
                        self.tail.x += 1
                    } else {
                        self.tail.x -= 1
                    }
                }

                // diagonal movement needed
                (false, false) => {
                    if x_h > x_t {
                        self.tail.x += 1
                    } else {
                        self.tail.x -= 1
                    }

                    if y_h > y_t {
                        self.tail.y += 1
                    } else {
                        self.tail.y -= 1
                    }
                }
            };

            tail_positions.insert(self.tail);
        }
    }

    fn are_touching(&self) -> bool {
        let Point { x: x_h, y: y_h } = self.head;
        let Point { x: x_t, y: y_t } = self.tail;

        x_t <= x_h + 1 && x_t >= x_h - 1 && y_t <= y_h + 1 && y_t >= y_h - 1
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
}

pub fn count_tail_positions(filename: &str) -> usize {
    let movements = fs::read_to_string(filename).unwrap();
    let movements: Vec<Movement> = movements.lines().map(Movement::from).collect();
    let mut rope = Rope::new();
    let mut tail_positions: HashSet<Point> = HashSet::new();

    tail_positions.insert(rope.tail);

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
        let tests = vec![("example/day09.txt", 13), ("input/day09.txt", 6181)];

        for test in tests {
            let (filename, want) = test;
            let got = day09::count_tail_positions(filename);

            assert_eq!(got, want, "got {got}, wanted {want}, for {filename}");
        }
    }
}
