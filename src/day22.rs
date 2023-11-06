use std::{collections::HashMap, fmt::Display, fs};

const SMALL_FACE: isize = 4;
const BIG_FACE: isize = 50;

#[derive(Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq)]
struct Point {
    row: isize,
    col: isize,
}

impl Point {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Side {
    Top,
    Right,
    Front,
    Back,
    Left,
    Bottom,
}

#[derive(Debug)]
enum Instruction {
    Forward(isize),
    RotateLeft,
    RotateRight,
}

#[derive(Debug)]
enum Tile {
    Open,
    Wall,
}

#[derive(Debug)]
struct FlatInput {
    tiles: HashMap<Point, Tile>,
    instructions: Vec<Instruction>,
}

impl FlatInput {
    fn next_point<F, M>(&self, filter: F, max_key: M) -> &Point
    where
        F: Fn(&&Point) -> bool,
        M: Fn(&&Point) -> isize,
    {
        self.tiles
            .keys()
            .filter(filter)
            .max_by_key(max_key)
            .unwrap()
    }

    fn new(raw_input: String) -> Self {
        let (map_rows, instructions): (Vec<&str>, &str) = {
            let (map, instructions) = raw_input.split_once("\n\n").unwrap();
            (map.lines().collect(), instructions.trim())
        };

        let mut input = FlatInput {
            tiles: HashMap::new(),
            instructions: Vec::new(),
        };

        for (map_row, row) in map_rows.into_iter().zip(1..) {
            for (cell, col) in map_row.chars().zip(1..) {
                match cell {
                    '#' => {
                        input.tiles.insert(Point::new(row, col), Tile::Wall);
                    }
                    '.' => {
                        input.tiles.insert(Point::new(row, col), Tile::Open);
                    }
                    ' ' => (),
                    _ => panic!("unexpected input"),
                };
            }
        }

        let mut current_number = String::new();
        for character in instructions.chars() {
            if character.is_numeric() {
                current_number.push(character);
            } else if character == 'L' || character == 'R' {
                input
                    .instructions
                    .push(Instruction::Forward(current_number.parse().unwrap()));
                current_number.clear();
                input.instructions.push(match character {
                    'L' => Instruction::RotateLeft,
                    'R' => Instruction::RotateRight,
                    _ => panic!("unexpected non-numeric character"),
                });
            } else {
                panic!("unexpected character: {character}");
            }
        }

        input
            .instructions
            .push(Instruction::Forward(current_number.parse().unwrap()));

        input
    }
}

impl Display for FlatInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_col = self.tiles.keys().max_by_key(|point| point.col).unwrap().col;
        let max_row = self.tiles.keys().max_by_key(|point| point.row).unwrap().row;

        for row in 1..max_row {
            for col in 1..max_col {
                let point = Point::new(row, col);
                if let Some(tile) = self.tiles.get(&point) {
                    match tile {
                        Tile::Open => write!(f, ".")?,
                        Tile::Wall => write!(f, "#")?,
                    }
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Direction {
    Up = 0,
    Right = 90,
    Down = 180,
    Left = 270,
}

impl Direction {
    fn rotate(&self, degrees: isize) -> Self {
        let rotated_direction = (*self as isize + degrees).rem_euclid(360);
        rotated_direction.into()
    }

    fn value(&self) -> isize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

impl From<isize> for Direction {
    fn from(value: isize) -> Self {
        match value {
            0 => Self::Up,
            90 => Self::Right,
            180 => Self::Down,
            270 => Self::Left,
            _ => panic!("invalid direction"),
        }
    }
}

#[derive(Debug)]
struct State {
    position: Point,
    facing: Direction,
}

impl State {
    fn new(input: &FlatInput) -> Self {
        let top_left_col = input
            .tiles
            .keys()
            .filter(|point| point.row == 1)
            .min_by_key(|point| point.col)
            .unwrap()
            .col;

        Self {
            position: Point::new(1, top_left_col),
            facing: Direction::Right,
        }
    }

    fn step(&mut self) {
        match self.facing {
            Direction::Up => self.position.row -= 1,
            Direction::Right => self.position.col += 1,
            Direction::Down => self.position.row += 1,
            Direction::Left => self.position.col -= 1,
        }
    }

    fn step_back(&mut self) {
        match self.facing {
            Direction::Up => self.position.row += 1,
            Direction::Right => self.position.col -= 1,
            Direction::Down => self.position.row -= 1,
            Direction::Left => self.position.col += 1,
        }
    }

    fn rotate(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::RotateLeft => {
                self.facing = self.facing.rotate(270);
            }
            Instruction::RotateRight => {
                self.facing = self.facing.rotate(90);
            }
            Instruction::Forward(_) => panic!("cannot rotate on a forward instruction"),
        }
    }
}

trait Input {
    fn wrap_around(&self, state: &State) -> (Point, Direction);
    fn instructions(&self) -> &[Instruction];
    fn get_tile(&self, point: &Point) -> Option<&Tile>;
}

impl Input for FlatInput {
    fn wrap_around(&self, state: &State) -> (Point, Direction) {
        let next_point = match state.facing {
            Direction::Up => *self.next_point(|pt| pt.col == state.position.col, |pt| pt.row),
            Direction::Right => *self.next_point(|pt| pt.row == state.position.row, |pt| -pt.col),
            Direction::Down => *self.next_point(|pt| pt.col == state.position.col, |pt| -pt.row),
            Direction::Left => *self.next_point(|pt| pt.row == state.position.row, |pt| pt.col),
        };

        (next_point, state.facing)
    }

    fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    fn get_tile(&self, point: &Point) -> Option<&Tile> {
        self.tiles.get(point)
    }
}

struct ThreeDimensionalInput {
    input: FlatInput,
    face_size: isize,
}

impl ThreeDimensionalInput {
    fn new(raw_input: String, face_size: isize) -> Self {
        let input = FlatInput::new(raw_input);
        Self { input, face_size }
    }
}

impl Input for ThreeDimensionalInput {
    fn wrap_around(&self, state: &State) -> (Point, Direction) {
        dbg!(state);

        let example_harcoded_wrappings: HashMap<(Point, Direction), (Point, Direction)> = (5..=8)
            .map(|row| (Point::new(row, 12), Direction::Right))
            .zip(
                (13..=16)
                    .rev()
                    .map(|col| (Point::new(9, col), Direction::Down)),
            )
            .chain(
                (9..=12)
                    .map(|col| (Point::new(12, col), Direction::Down))
                    .zip((1..=4).rev().map(|col| (Point::new(8, col), Direction::Up))),
            )
            .chain(
                (5..=8)
                    .map(|col| (Point::new(5, col), Direction::Up))
                    .zip((1..=4).map(|row| (Point::new(row, 9), Direction::Right))),
            )
            .collect();

        let input_harcoded_wrappings: HashMap<(Point, Direction), (Point, Direction)> = (51..=100)
            .map(|col| (Point::new(1, col), Direction::Up))
            .zip((151..=200).map(|row| (Point::new(row, 1), Direction::Right)))
            .chain(
                (151..=200)
                    .map(|row| (Point::new(row, 50), Direction::Right))
                    .zip((51..=100).map(|col| (Point::new(150, col), Direction::Up))),
            )
            .chain(
                (51..=100)
                    .map(|col| (Point::new(150, col), Direction::Down))
                    .zip((151..=200).map(|row| (Point::new(row, 50), Direction::Left))),
            )
            .chain(
                (1..=50)
                    .map(|col| (Point::new(101, col), Direction::Up))
                    .zip((51..=100).map(|row| (Point::new(row, 51), Direction::Right))),
            )
            .chain(
                (51..=100)
                    .map(|row| (Point::new(row, 51), Direction::Left))
                    .zip((1..=50).map(|col| (Point::new(101, col), Direction::Down))),
            )
            .chain(
                (101..=150)
                    .map(|row| (Point::new(row, 100), Direction::Right))
                    .zip(
                        (1..=50)
                            .rev()
                            .map(|row| (Point::new(row, 150), Direction::Left)),
                    ),
            )
            .chain(
                (1..=50)
                    .rev()
                    .map(|row| (Point::new(row, 150), Direction::Right))
                    .zip((101..=150).map(|row| (Point::new(row, 100), Direction::Left))),
            )
            .chain(
                (101..=150)
                    .map(|col| (Point::new(1, col), Direction::Up))
                    .zip((1..=50).map(|col| (Point::new(200, col), Direction::Up))),
            )
            .chain(
                (1..=50)
                    .map(|col| (Point::new(200, col), Direction::Down))
                    .zip((101..=150).map(|col| (Point::new(1, col), Direction::Down))),
            )
            .chain(
                (151..=200)
                    .map(|row| (Point::new(row, 1), Direction::Left))
                    .zip((51..=100).map(|col| (Point::new(1, col), Direction::Down))),
            )
            .chain(
                (101..=150)
                    .map(|col| (Point::new(50, col), Direction::Down))
                    .zip((51..=100).map(|row| (Point::new(row, 100), Direction::Left))),
            )
            .chain(
                (51..=100)
                    .map(|row| (Point::new(row, 100), Direction::Right))
                    .zip((101..=150).map(|col| (Point::new(50, col), Direction::Up))),
            )
            .chain(
                (101..=150)
                    .map(|row| (Point::new(row, 1), Direction::Left))
                    .zip(
                        (1..=51)
                            .rev()
                            .map(|row| (Point::new(row, 51), Direction::Right)),
                    ),
            )
            .chain(
                (1..=51)
                    .map(|row| (Point::new(row, 51), Direction::Left))
                    .rev()
                    .zip((101..=150).map(|row| (Point::new(row, 1), Direction::Right))),
            )
            .collect();

        let hardcoded_wrapping = if self.face_size == SMALL_FACE {
            example_harcoded_wrappings
        } else if self.face_size == BIG_FACE {
            input_harcoded_wrappings
        } else {
            panic!("unrecongized face size")
        };

        *hardcoded_wrapping
            .get(&(state.position, state.facing))
            .expect("HARDCODED MAPPING")
    }

    fn instructions(&self) -> &[Instruction] {
        &self.input.instructions
    }

    fn get_tile(&self, point: &Point) -> Option<&Tile> {
        self.input.tiles.get(point)
    }
}

fn follow_instructions(mut state: State, input: impl Input) -> isize {
    for instruction in input.instructions() {
        match instruction {
            Instruction::Forward(n) => {
                for _ in 0..*n {
                    state.step();
                    match input.get_tile(&state.position) {
                        Some(Tile::Open) => continue,
                        Some(Tile::Wall) => {
                            state.step_back();
                            break;
                        }
                        None => {
                            // we have stepped off the tiles, or gone off the map. step back and wrap around
                            state.step_back();
                            let (next_point, next_facing) = input.wrap_around(&state);
                            match input.get_tile(&next_point).unwrap() {
                                Tile::Wall => break,
                                Tile::Open => {
                                    state.position = next_point;
                                    state.facing = next_facing
                                }
                            }
                        }
                    }
                }
            }
            Instruction::RotateLeft | Instruction::RotateRight => state.rotate(instruction),
        }
    }
    1000 * state.position.row + 4 * state.position.col + state.facing.value()
}

pub fn find_password(filename: &str) -> isize {
    let input = FlatInput::new(fs::read_to_string(filename).unwrap());
    let state = State::new(&input);
    follow_instructions(state, input)
}

pub fn find_password_with_cube_wrapping(filename: &str, face_size: isize) -> isize {
    let input = ThreeDimensionalInput::new(fs::read_to_string(filename).unwrap(), face_size);
    let state = State::new(&input.input);
    follow_instructions(state, input)
}

#[cfg(test)]
mod tests {
    use crate::{day22, fetch_input};

    #[test]
    #[ignore]
    fn find_final_password() {
        fetch_input(22);

        let tests = vec![("example/day22.txt", 6032), ("input/day22.txt", 88226)];

        for (infile, want) in tests {
            let got = day22::find_password(infile);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }

    #[test]
    fn find_final_password_on_cube_net() {
        fetch_input(22);

        let tests = vec![
            // ("example/day22.txt", day22::SMALL_FACE, 5031),
            ("input/day22.txt", day22::BIG_FACE, 0),
        ];

        for (infile, face_size, want) in tests {
            let got = day22::find_password_with_cube_wrapping(infile, face_size);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }
}
