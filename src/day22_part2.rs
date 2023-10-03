use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn neighbours(&self) -> [(Self, Direction); 4] {
        [
            (Self::new(self.x - 1, self.y), Direction::Up),
            (Self::new(self.x, self.y + 1), Direction::Right),
            (Self::new(self.x + 1, self.y), Direction::Down),
            (Self::new(self.x, self.y - 1), Direction::Left),
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

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up = 0,
    Right = 90,
    Down = 180,
    Left = 270,
}

impl Direction {
    fn rotate(&self, degrees: i64) -> Self {
        let rotated_direction = (*self as i64 + degrees).rem_euclid(360);
        rotated_direction.into()
    }
}

impl From<i64> for Direction {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Up,
            90 => Self::Right,
            180 => Self::Down,
            270 => Self::Left,
            _ => panic!("invalid direction"),
        }
    }
}

// this is if you are looking at the side face on! It does not
// account for rotation
pub fn side_face(side: Side, direction: Direction) -> Side {
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
        (Side::Bottom, Direction::Right) => Side::Left,
        (Side::Bottom, Direction::Left) => Side::Right,
        (Side::Bottom, Direction::Down) => Side::Back,
    }
}

pub fn fold_cube(points: HashSet<Point>) -> HashMap<Point, Side> {
    let top_left_face = points
        .iter()
        .min_by_key(|face| face.y + face.x * 1000)
        .unwrap();

    let mut faces = HashMap::new();

    recursive_fold_cube(&mut faces, &points, *top_left_face, Side::Top, 0);

    faces
}

pub fn recursive_fold_cube(
    faces: &mut HashMap<Point, Side>,
    points: &HashSet<Point>,
    point: Point,
    side: Side,
    mut rotation: i64,
) {
    faces.insert(point, side);

    for (neighbour, direction) in point.neighbours() {
        if points.contains(&neighbour) && !faces.contains_key(&neighbour) {
            let direction = direction.rotate(rotation);
            if matches!(side, Side::Top | Side::Bottom) {
                rotation = direction as i64;
            }
            let side = side_face(side, direction.into());
            recursive_fold_cube(faces, points, neighbour, side, rotation);
        }
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn fold_cube_test() {
        let nets = HashSet::from([
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(0, 2),
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(3, 1),
        ]);

        let mut actual_cube_faces: Vec<(Point, Side)> = fold_cube(nets).into_iter().collect();
        actual_cube_faces.sort();

        let expected_cube_faces = Vec::from([
            (Point::new(0, 0), Side::Top),
            (Point::new(0, 1), Side::Right),
            (Point::new(0, 2), Side::Bottom),
            (Point::new(1, 1), Side::Front),
            (Point::new(2, 1), Side::Left),
            (Point::new(3, 1), Side::Back),
        ]);

        assert_eq!(expected_cube_faces, actual_cube_faces)
    }
}
