use std::{collections::HashMap, fmt::Display, fs};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    col: isize,
    row: isize,
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
struct Input {
    tiles: HashMap<Point, Tile>,
    instructions: Vec<Instruction>,
}
impl Input {
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

    fn two_dimensional_wrap_around(&self, state: &State) -> Point {
        match state.facing.0 {
            Direction::UP => *self.next_point(|pt| pt.col == state.position.col, |pt| pt.row),
            Direction::RIGHT => *self.next_point(|pt| pt.row == state.position.row, |pt| -pt.col),
            Direction::DOWN => *self.next_point(|pt| pt.col == state.position.col, |pt| -pt.row),
            Direction::LEFT => *self.next_point(|pt| pt.row == state.position.row, |pt| pt.col),
            _ => panic!("invalid direction: {:?}", state.facing),
        }
    }

    fn three_dimensional_wrap_around(&self, _state: &State) -> Point {
        todo!("HOW TF DO U WRAP AROUND?")
    }
}

impl From<String> for Input {
    fn from(raw_input: String) -> Self {
        let (map_rows, instructions): (Vec<&str>, &str) = {
            let (map, instructions) = raw_input.split_once("\n\n").unwrap();
            (map.lines().collect(), instructions.trim())
        };

        let mut input = Input {
            tiles: HashMap::new(),
            instructions: Vec::new(),
        };

        for (map_row, row) in map_rows.into_iter().zip(1..) {
            for (cell, col) in map_row.chars().zip(1..) {
                match cell {
                    '#' => {
                        input.tiles.insert(Point { col, row }, Tile::Wall);
                    }
                    '.' => {
                        input.tiles.insert(Point { col, row }, Tile::Open);
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

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_col = self.tiles.keys().max_by_key(|point| point.col).unwrap().col;
        let max_row = self.tiles.keys().max_by_key(|point| point.row).unwrap().row;

        for row in 1..max_row {
            for col in 1..max_col {
                let point = Point { col, row };
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

#[derive(Debug)]
struct Direction(isize);
impl Direction {
    const RIGHT: isize = 0;
    const DOWN: isize = 1;
    const LEFT: isize = 2;
    const UP: isize = 3;
}

#[derive(Debug)]
struct State {
    position: Point,
    facing: Direction,
}

impl State {
    fn new(input: &Input) -> Self {
        let top_left_col = input
            .tiles
            .keys()
            .filter(|point| point.row == 1)
            .min_by_key(|point| point.col)
            .unwrap()
            .col;

        Self {
            position: Point {
                col: top_left_col,
                row: 1,
            },
            facing: Direction(Direction::RIGHT),
        }
    }

    fn step(&mut self) {
        match self.facing.0 {
            Direction::UP => self.position.row -= 1,
            Direction::RIGHT => self.position.col += 1,
            Direction::DOWN => self.position.row += 1,
            Direction::LEFT => self.position.col -= 1,
            _ => panic!("invalid direction: {:?}", self.facing),
        }
    }

    fn step_back(&mut self) {
        match self.facing.0 {
            Direction::UP => self.position.row += 1,
            Direction::RIGHT => self.position.col -= 1,
            Direction::DOWN => self.position.row -= 1,
            Direction::LEFT => self.position.col += 1,
            _ => panic!("invalid direction: {:?}", self.facing),
        }
    }

    fn rotate(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::RotateLeft => {
                self.facing = Direction((self.facing.0 - 1).rem_euclid(4));
            }
            Instruction::RotateRight => {
                self.facing = Direction((self.facing.0 + 1).rem_euclid(4));
            }
            Instruction::Forward(_) => panic!("cannot rotate on a forward instruction"),
        }
    }
}

enum Wrapping {
    Flat,
    Cube,
}

fn follow_instructions(filename: &str, wrapping: &Wrapping) -> isize {
    let wrap_around = match wrapping {
        Wrapping::Flat => Input::two_dimensional_wrap_around,
        Wrapping::Cube => Input::three_dimensional_wrap_around,
    };

    let input = Input::from(fs::read_to_string(filename).unwrap());
    let mut state = State::new(&input);

    for instruction in &input.instructions {
        match instruction {
            Instruction::Forward(n) => {
                for _ in 0..*n {
                    state.step();
                    match input.tiles.get(&state.position) {
                        Some(Tile::Open) => continue,
                        Some(Tile::Wall) => {
                            state.step_back();
                            break;
                        }
                        None => {
                            // we have stepped off the tiles, or gone off the map. step back and wrap around
                            state.step_back();
                            let next_point = wrap_around(&input, &state);
                            match input.tiles.get(&next_point).unwrap() {
                                Tile::Wall => break,
                                Tile::Open => state.position = next_point,
                            }
                        }
                    }
                }
            }
            Instruction::RotateLeft | Instruction::RotateRight => state.rotate(instruction),
        }
    }
    1000 * state.position.row + 4 * state.position.col + state.facing.0
}

pub fn find_password(filename: &str) -> isize {
    follow_instructions(filename, &Wrapping::Flat)
}

pub fn find_password_with_cube_wrapping(filename: &str) -> isize {
    follow_instructions(filename, &Wrapping::Cube)
}

#[cfg(test)]
mod tests {
    use crate::{day22, fetch_input};

    #[test]
    fn find_final_password() {
        fetch_input(22);

        let tests = vec![("example/day22.txt", 6032), ("input/day22.txt", 88226)];

        for (infile, want) in tests {
            let got = day22::find_password(infile);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }

    #[test]
    #[ignore]
    fn find_final_password_on_cube_net() {
        fetch_input(22);

        let tests = vec![
            ("example/day22.txt", 5031),
            // ("input/day22.txt", 000000000000000),
        ];

        for (infile, want) in tests {
            let got = day22::find_password_with_cube_wrapping(infile);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }
}
