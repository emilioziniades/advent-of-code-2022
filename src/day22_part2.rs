use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Side {
    Top,
    Right,
    Front,
    Back,
    Left,
    Bottom,
}

pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

pub fn side_face(current_side: Side, direction: Direction) -> Side {
    match (current_side, direction) {
        (Side::Top, Direction::Up) => Side::Back,
        (Side::Right, Direction::Right) => Side::Bottom,
        (Side::Left, Direction::Left) => Side::Bottom,
        (Side::Bottom, Direction::Down) => Side::Back,

        (Side::Top, Direction::Right) => Side::Right,
        (Side::Front, Direction::Right) => Side::Right,
        (Side::Back, Direction::Left) => Side::Right,
        (Side::Bottom, Direction::Left) => Side::Right,

        (Side::Top, Direction::Left) => Side::Left,
        (Side::Front, Direction::Left) => Side::Left,
        (Side::Back, Direction::Right) => Side::Left,
        (Side::Bottom, Direction::Right) => Side::Left,

        (Side::Top, Direction::Down) => Side::Front,
        (Side::Right, Direction::Left) => Side::Front,
        (Side::Left, Direction::Right) => Side::Front,
        (Side::Bottom, Direction::Up) => Side::Front,

        (Side::Front, Direction::Up) => Side::Top,
        (Side::Right, Direction::Up) => Side::Top,
        (Side::Back, Direction::Up) => Side::Top,
        (Side::Left, Direction::Up) => Side::Top,

        (Side::Front, Direction::Down) => Side::Bottom,
        (Side::Right, Direction::Down) => Side::Front,
        (Side::Back, Direction::Down) => Side::Bottom,
        (Side::Left, Direction::Down) => Side::Bottom,
    }
}

pub fn cube_from_net(points: HashSet<Point>) -> HashMap<Point, Side> {
    let mut cube = HashMap::new();
    let top_left_face = points
        .iter()
        .min_by_key(|face| face.y + face.x * 1000)
        .unwrap();

    cube.insert(*top_left_face, Side::Top);
    let mut queue = vec![(*top_left_face, Side::Top)];

    while let Some((current_point, current_side)) = queue.pop() {
        println!("{current_point:?} {current_side:?}");
        if let Some(down) = points.get(&Point::new(current_point.x + 1, current_point.y)) {
            if !cube.contains_key(&down) {
                let side = side_face(current_side, Direction::Down);
                cube.insert(*down, side);
                queue.push((*down, side));
            }
        }
        if let Some(up) = points.get(&Point::new(current_point.x - 1, current_point.y)) {
            if !cube.contains_key(&up) {
                let side = side_face(current_side, Direction::Up);
                cube.insert(*up, side);
                queue.push((*up, side));
            }
        }
        if let Some(left) = points.get(&Point::new(current_point.x, current_point.y - 1)) {
            if !cube.contains_key(&left) {
                let side = side_face(current_side, Direction::Left);
                cube.insert(*left, side);
                queue.push((*left, side));
            }
        }
        if let Some(right) = points.get(&Point::new(current_point.x, current_point.y + 1)) {
            if !cube.contains_key(&right) {
                let side = side_face(current_side, Direction::Right);
                cube.insert(*right, side);
                queue.push((*right, side));
            }
        }
    }

    cube
}
#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn fold_cube() {
        // let tests = vec![HashSet::from_iter(vec![
        //     Point::new(0, 0),
        //     Point::new(0, 0),
        //     Point::new(0, 0),
        // ])];

        let nets = HashSet::from([
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(0, 2),
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(3, 1),
        ]);

        let actual_cube_faces = cube_from_net(nets);

        let expected_cube_faces = HashMap::from([
            (Point::new(0, 0), Side::Top),
            (Point::new(0, 1), Side::Right),
            (Point::new(0, 2), Side::Bottom),
            (Point::new(1, 1), Side::Front),
            (Point::new(2, 1), Side::Left),
            (Point::new(3, 1), Side::Back),
        ]);

        assert_eq!(actual_cube_faces, expected_cube_faces)
    }
}
