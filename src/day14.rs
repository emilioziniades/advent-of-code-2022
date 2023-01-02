use std::{collections::HashSet, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub fn count_sand(file: &str) -> usize {
    let rocks = get_rocks(file);
    let mut sands: HashSet<Point> = HashSet::new();

    let lowest_rock_height = rocks.iter().map(|point| point.y).max().unwrap();

    while let Some(sand) = fall(&rocks, &sands, lowest_rock_height) {
        sands.insert(sand);
    }

    sands.len()
}

fn fall(
    rocks: &HashSet<Point>,
    sands: &HashSet<Point>,
    lowest_rock_height: usize,
) -> Option<Point> {
    let mut sand = Point::from(500, 0);

    loop {
        if sand.y > lowest_rock_height {
            return None;
        }

        let below = Point::from(sand.x, sand.y + 1);
        let below_left = Point::from(sand.x - 1, sand.y + 1);
        let below_right = Point::from(sand.x + 1, sand.y + 1);

        // try go down
        if rocks.contains(&below) || sands.contains(&below) {
            // try go left
            if rocks.contains(&below_left) || sands.contains(&below_left) {
                // try go right
                if rocks.contains(&below_right) || sands.contains(&below_right) {
                    // can't go anywhere
                    return Some(sand);
                } else {
                    sand = below_right;
                }
            } else {
                sand = below_left;
            }
        } else {
            sand = below;
        }
    }
}

fn get_rocks(file: &str) -> HashSet<Point> {
    let rock_paths = fs::read_to_string(file).unwrap();
    let rock_paths = rock_paths.lines();

    let mut rocks: HashSet<Point> = HashSet::new();

    for rock_path in rock_paths {
        let rock_path = rock_path
            .split("->")
            .map(|point| {
                let (x, y) = point.trim().split_once(',').unwrap();
                let (x, y): (usize, usize) = (x.parse().unwrap(), y.parse().unwrap());
                Point::from(x, y)
            })
            .collect::<Vec<Point>>();

        // println!("{rock_path:#?}");
        for points in rock_path.windows(2) {
            let (start, end) = (points[0], points[1]);
            if start.x == end.x && start.y < end.y {
                let points = (start.y..=end.y).map(|y| Point::from(start.x, y));
                rocks.extend(points);
            } else if start.x == end.x && start.y > end.y {
                let points = (end.y..=start.y).map(|y| Point::from(start.x, y));
                rocks.extend(points);
            } else if start.y == end.y && start.x < end.x {
                let points = (start.x..=end.x).map(|x| Point::from(x, start.y));
                rocks.extend(points);
            } else if start.y == end.y && start.x > end.x {
                let points = (end.x..=start.x).map(|x| Point::from(x, start.y));
                rocks.extend(points);
            } else {
                panic!("not a straight line")
            }
        }
    }

    rocks
}

#[cfg(test)]
mod tests {
    use crate::{day14, fetch_input};

    #[test]
    fn count_sand() {
        fetch_input(14);

        let tests = vec![("example/day14.txt", 24), ("input/day14.txt", 1068)];

        for test in tests {
            let (file, want) = test;
            let got = day14::count_sand(file);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }
}
