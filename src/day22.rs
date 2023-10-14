use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs,
};

#[derive(Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq)]
struct Point {
    col: isize,
    row: isize,
}

impl Point {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    fn neighbours(&self) -> [(Self, Direction); 4] {
        [
            (Self::new(self.row - 1, self.col), Direction::Up),
            (Self::new(self.row, self.col + 1), Direction::Right),
            (Self::new(self.row + 1, self.col), Direction::Down),
            (Self::new(self.row, self.col - 1), Direction::Left),
        ]
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Side {
    Top,
    Right,
    Front,
    Back,
    Left,
    Bottom,
}

// this is if you are looking at the side face on! It does not
// account for rotation
pub fn side_face(side: Side, direction: Direction) -> Side {
    println!("{side:?}\t{direction:?}");
    match (side, direction) {
        (Side::Top, Direction::Up) => Side::Back,
        (Side::Top, Direction::Right) => Side::Right,
        (Side::Top, Direction::Left) => Side::Left,
        (Side::Top, Direction::Down) => Side::Front,

        (Side::Right, Direction::Up) => Side::Top,
        (Side::Right, Direction::Right) => Side::Back,
        (Side::Right, Direction::Left) => Side::Front,
        (Side::Right, Direction::Down) => Side::Bottom,

        (Side::Front, Direction::Up) => Side::Top,
        (Side::Front, Direction::Right) => Side::Right,
        (Side::Front, Direction::Left) => Side::Left,
        (Side::Front, Direction::Down) => Side::Bottom,

        (Side::Back, Direction::Up) => Side::Top,
        (Side::Back, Direction::Right) => Side::Left,
        (Side::Back, Direction::Left) => Side::Right,
        (Side::Back, Direction::Down) => Side::Bottom,

        (Side::Left, Direction::Up) => Side::Top,
        (Side::Left, Direction::Right) => Side::Front,
        (Side::Left, Direction::Left) => Side::Back,
        (Side::Left, Direction::Down) => Side::Bottom,

        (Side::Bottom, Direction::Up) => Side::Front,
        (Side::Bottom, Direction::Right) => Side::Right,
        (Side::Bottom, Direction::Left) => Side::Left,
        (Side::Bottom, Direction::Down) => Side::Back,
    }
}

fn fold_cube(points: HashSet<Point>) -> HashMap<Point, Side> {
    let top_left_face = points
        .iter()
        .min_by_key(|face| face.col + face.row * 1000)
        .unwrap();

    let mut faces = HashMap::new();

    recursive_fold_cube(&mut faces, &points, *top_left_face, Side::Top, 0);

    faces
}

fn recursive_fold_cube(
    faces: &mut HashMap<Point, Side>,
    points: &HashSet<Point>,
    point: Point,
    side: Side,
    rotation: isize,
) {
    faces.insert(point, side);

    for (neighbour, direction) in point.neighbours() {
        if points.contains(&neighbour) && !faces.contains_key(&neighbour) {
            let direction = direction.rotate(rotation);
            let rotation = rotation
                + match (side, direction) {
                    (Side::Top, Direction::Right) => 90,
                    (Side::Bottom, Direction::Right) => 270,
                    (Side::Top, Direction::Left) => 270,
                    (Side::Bottom, Direction::Left) => 90,
                    (Side::Top | Side::Bottom, Direction::Up) => 180,
                    (Side::Top | Side::Bottom, Direction::Down) => 0,
                    (_, _) => 0,
                };
            let side = side_face(side, direction.into());
            recursive_fold_cube(faces, points, neighbour, side, rotation);
        }
    }
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

#[derive(Debug, Clone, Copy)]
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
    fn wrap_around(&self, state: &State) -> Point;
    fn instructions(&self) -> &[Instruction];
    fn get_tile(&self, point: &Point) -> Option<&Tile>;
}

impl Input for FlatInput {
    fn wrap_around(&self, state: &State) -> Point {
        match state.facing {
            Direction::Up => *self.next_point(|pt| pt.col == state.position.col, |pt| pt.row),
            Direction::Right => *self.next_point(|pt| pt.row == state.position.row, |pt| -pt.col),
            Direction::Down => *self.next_point(|pt| pt.col == state.position.col, |pt| -pt.row),
            Direction::Left => *self.next_point(|pt| pt.row == state.position.row, |pt| pt.col),
        }
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
    // cube_faces: (),
}

impl ThreeDimensionalInput {
    fn new(raw_input: String, _face_size: usize) -> Self {
        let input = FlatInput::new(raw_input);
        Self { input }
    }
}

impl Input for ThreeDimensionalInput {
    fn wrap_around(&self, _state: &State) -> Point {
        todo!()
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
                            let next_point = input.wrap_around(&state);
                            match input.get_tile(&next_point).unwrap() {
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
    1000 * state.position.row + 4 * state.position.col + state.facing.value()
}

pub fn find_password(filename: &str) -> isize {
    let input = FlatInput::new(fs::read_to_string(filename).unwrap());
    let state = State::new(&input);
    follow_instructions(state, input)
}

pub fn find_password_with_cube_wrapping(filename: &str, face_size: usize) -> isize {
    let input = ThreeDimensionalInput::new(fs::read_to_string(filename).unwrap(), face_size);
    let state = State::new(&input.input);
    follow_instructions(state, input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{day22, fetch_input};
    use pretty_assertions::assert_eq;

    const SMALL_FACE: usize = 4;
    // const BIG_FACE: usize = 50;

    #[test]
    // #[ignore]
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
            ("example/day22.txt", SMALL_FACE, 5031),
            // ("input/day22.txt", BIG_FACE, 000000000000000),
        ];

        for (infile, face_size, want) in tests {
            let got = day22::find_password_with_cube_wrapping(infile, face_size);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }

    fn fold_cube_test_runner(expected_faces: HashMap<Point, Side>) {
        let nets = expected_faces.keys().copied().collect();
        let actual_faces = fold_cube(nets);

        let mut expected_faces: Vec<(Point, Side)> = expected_faces.into_iter().collect();
        expected_faces.sort();

        let mut actual_faces: Vec<(Point, Side)> = actual_faces.into_iter().collect();
        actual_faces.sort();

        assert_eq!(expected_faces, actual_faces)
    }

    #[test]
    fn fold_t_net() {
        let expected_faces = HashMap::from([
            (Point::new(0, 0), Side::Top),
            (Point::new(0, 1), Side::Right),
            (Point::new(0, 2), Side::Bottom),
            (Point::new(1, 1), Side::Front),
            (Point::new(2, 1), Side::Left),
            (Point::new(3, 1), Side::Back),
        ]);

        fold_cube_test_runner(expected_faces);
    }

    #[test]
    fn fold_cross_net() {
        let expected_faces = HashMap::from([
            (Point::new(0, 1), Side::Top),
            (Point::new(1, 0), Side::Left),
            (Point::new(1, 1), Side::Front),
            (Point::new(1, 2), Side::Right),
            (Point::new(2, 1), Side::Bottom),
            (Point::new(3, 1), Side::Back),
        ]);

        fold_cube_test_runner(expected_faces);
    }

    #[test]
    fn fold_aoc_example_net() {
        let expected_faces = HashMap::from([
            (Point::new(0, 2), Side::Top),
            (Point::new(1, 2), Side::Front),
            (Point::new(1, 1), Side::Left),
            (Point::new(1, 0), Side::Back),
            (Point::new(2, 2), Side::Bottom),
            (Point::new(2, 3), Side::Right),
        ]);

        fold_cube_test_runner(expected_faces);
    }

    #[test]
    fn fold_aoc_input_net() {
        let expected_faces = HashMap::from([
            (Point::new(0, 1), Side::Top),
            (Point::new(0, 2), Side::Right),
            (Point::new(1, 1), Side::Front),
            (Point::new(2, 1), Side::Bottom),
            (Point::new(2, 0), Side::Left),
            (Point::new(3, 0), Side::Back),
        ]);

        fold_cube_test_runner(expected_faces);
    }
}
