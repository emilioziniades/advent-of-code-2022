use std::{collections::HashSet, fmt::Display, fs};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    col: usize,
    row: usize,
}

#[derive(Debug)]
enum Instruction {
    Forward(usize),
    RotateLeft,
    RotateRight,
}

#[derive(Debug)]
struct Input {
    tiles: HashSet<Point>,
    walls: HashSet<Point>,
    instructions: Vec<Instruction>,
}

impl From<String> for Input {
    fn from(raw_input: String) -> Self {
        let (map_rows, instructions): (Vec<&str>, &str) = {
            let (map, instructions) = raw_input.split_once("\n\n").unwrap();
            (map.lines().collect(), instructions.trim())
        };

        let mut input = Input {
            tiles: HashSet::new(),
            walls: HashSet::new(),
            instructions: Vec::new(),
        };

        for (map_row, row) in map_rows.into_iter().zip(1..) {
            for (cell, col) in map_row.chars().zip(1..) {
                match cell {
                    '#' => {
                        input.walls.insert(Point { col, row });
                    }
                    '.' => {
                        input.tiles.insert(Point { col, row });
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
        let max_wall_col = self.walls.iter().max_by_key(|c| c.col).unwrap();
        let max_tile_col = self.tiles.iter().max_by_key(|c| c.col).unwrap();
        let max_wall_row = self.walls.iter().max_by_key(|c| c.row).unwrap();
        let max_tile_row = self.tiles.iter().max_by_key(|c| c.row).unwrap();

        let max_col = max_wall_col.col.max(max_tile_col.col);
        let max_row = max_wall_row.row.max(max_tile_row.row);

        for row in 1..max_row {
            for col in 1..max_col {
                let point = Point { col, row };
                if self.tiles.contains(&point) {
                    write!(f, ".")?;
                } else if self.walls.contains(&point) {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct State {
    position: Point,
    facing: Direction,
}

impl State {
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
        match (instruction, self.facing) {
            (Instruction::RotateLeft, Direction::Up) => self.facing = Direction::Left,
            (Instruction::RotateLeft, Direction::Right) => self.facing = Direction::Up,
            (Instruction::RotateLeft, Direction::Down) => self.facing = Direction::Right,
            (Instruction::RotateLeft, Direction::Left) => self.facing = Direction::Down,
            (Instruction::RotateRight, Direction::Up) => self.facing = Direction::Right,
            (Instruction::RotateRight, Direction::Right) => self.facing = Direction::Down,
            (Instruction::RotateRight, Direction::Down) => self.facing = Direction::Left,
            (Instruction::RotateRight, Direction::Left) => self.facing = Direction::Up,
            (Instruction::Forward(_), _) => panic!("cannot rotate on a forward instruction"),
        }
    }
}

pub fn find_final_password(filename: &str) -> usize {
    let input = Input::from(fs::read_to_string(filename).unwrap());

    let top_left_col = input
        .tiles
        .iter()
        .filter(|tile| tile.row == 1)
        .min_by_key(|tile| tile.col)
        .unwrap()
        .col;

    let mut state = State {
        position: Point {
            col: top_left_col,
            row: 1,
        },
        facing: Direction::Right,
    };

    let min_row = 1;
    let min_col = 1;
    let max_row = input.tiles.iter().max_by_key(|tile| tile.row).unwrap().row;
    let max_col = input.tiles.iter().max_by_key(|tile| tile.col).unwrap().col;

    for instruction in &input.instructions {
        match instruction {
            Instruction::Forward(n) => {
                for _ in 0..*n {
                    state.step();
                    if input.tiles.contains(&state.position) {
                        continue;
                    } else if input.walls.contains(&state.position) {
                        state.step_back();
                        break;
                    } else if state.position.row > max_row
                        || state.position.row < min_row
                        || state.position.col > max_col
                        || state.position.col < min_col
                        || (!input.tiles.contains(&state.position)
                            && !input.walls.contains(&state.position))
                    {
                        // we have stepped off the tiles, or gone off the map. step back and wrap around
                        state.step_back();
                        match state.facing {
                            Direction::Up => {
                                let next_row = input
                                    .tiles
                                    .iter()
                                    .filter(|tile| tile.col == state.position.col)
                                    .max_by_key(|tile| tile.row)
                                    .unwrap()
                                    .row;

                                let next_wall_position = input
                                    .walls
                                    .iter()
                                    .filter(|tile| tile.col == state.position.col)
                                    .max_by_key(|tile| tile.row);

                                if let Some(next_wall_position) = next_wall_position {
                                    if next_wall_position.row > next_row {
                                        // hit a wall on wrap arouond
                                        break;
                                    }
                                }

                                state.position = Point {
                                    col: state.position.col,
                                    row: next_row,
                                };
                            }
                            Direction::Right => {
                                let next_col = input
                                    .tiles
                                    .iter()
                                    .filter(|tile| tile.row == state.position.row)
                                    .min_by_key(|tile| tile.col)
                                    .unwrap()
                                    .col;

                                let next_wall_position = input
                                    .walls
                                    .iter()
                                    .filter(|tile| tile.row == state.position.row)
                                    .min_by_key(|tile| tile.col);

                                if let Some(next_wall_position) = next_wall_position {
                                    if next_wall_position.col < next_col {
                                        // hit a wall on wrap arouond
                                        break;
                                    }
                                }

                                state.position = Point {
                                    col: next_col,
                                    row: state.position.row,
                                };
                            }
                            Direction::Down => {
                                let next_row = input
                                    .tiles
                                    .iter()
                                    .filter(|tile| tile.col == state.position.col)
                                    .min_by_key(|tile| tile.row)
                                    .unwrap()
                                    .row;

                                let next_wall_position = input
                                    .walls
                                    .iter()
                                    .filter(|tile| tile.col == state.position.col)
                                    .min_by_key(|tile| tile.row);

                                if let Some(next_wall_position) = next_wall_position {
                                    if next_wall_position.row < next_row {
                                        // hit a wall on wrap arouond
                                        break;
                                    }
                                }

                                state.position = Point {
                                    col: state.position.col,
                                    row: next_row,
                                };
                            }
                            Direction::Left => {
                                let next_col = input
                                    .tiles
                                    .iter()
                                    .filter(|tile| tile.row == state.position.row)
                                    .max_by_key(|tile| tile.col)
                                    .unwrap()
                                    .col;

                                let next_wall_position = input
                                    .walls
                                    .iter()
                                    .filter(|tile| tile.row == state.position.row)
                                    .max_by_key(|tile| tile.col);

                                if let Some(next_wall_position) = next_wall_position {
                                    if next_wall_position.col > next_col {
                                        // hit a wall on wrap arouond
                                        break;
                                    }
                                }

                                state.position = Point {
                                    col: next_col,
                                    row: state.position.row,
                                };
                            }
                        }
                    } else {
                        panic!("in an unknown state");
                    }
                }
            }
            Instruction::RotateLeft | Instruction::RotateRight => state.rotate(instruction),
        }
    }

    let facing_value = match state.facing {
        Direction::Up => 3,
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
    };

    1000 * state.position.row + 4 * state.position.col + facing_value
}

#[cfg(test)]
mod tests {
    use crate::{day22, fetch_input};

    #[test]
    fn find_final_password() {
        fetch_input(22);

        let tests = vec![("example/day22.txt", 6032), ("input/day22.txt", 88226)];

        for (infile, want) in tests {
            let got = day22::find_final_password(infile);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }
}
