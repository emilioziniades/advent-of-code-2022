use std::fs;

#[derive(Debug)]
struct Grid(Vec<Vec<u8>>);

impl Grid {
    fn from(filename: &str) -> Self {
        Self(
            fs::read_to_string(filename)
                .expect("file exists")
                .lines()
                .map(|line| {
                    line.split("")
                        .filter_map(|n| n.parse().ok())
                        .collect::<Vec<u8>>()
                })
                .collect::<Vec<_>>(),
        )
    }

    fn at(&self, pt: &Point) -> u8 {
        self.0[pt.x][pt.y]
    }

    fn scenic_score(&self, pt: &Point) -> u32 {
        self.scenic_score_from(pt, &Direction::Left)
            * self.scenic_score_from(pt, &Direction::Right)
            * self.scenic_score_from(pt, &Direction::Up)
            * self.scenic_score_from(pt, &Direction::Down)
    }

    fn scenic_score_from(&self, pt: &Point, direction: &Direction) -> u32 {
        let val = self.at(pt);
        let mut items_until = self.items_until(pt, direction);

        match direction {
            Direction::Left | Direction::Up => items_until.reverse(),
            _ => (),
        }

        let mut visible_items = 0;
        for item in items_until {
            visible_items += 1;
            if item >= &val {
                break;
            }
        }
        visible_items
    }

    fn is_visible(&self, pt: &Point) -> bool {
        self.is_visible_from(pt, &Direction::Left)
            || self.is_visible_from(pt, &Direction::Right)
            || self.is_visible_from(pt, &Direction::Up)
            || self.is_visible_from(pt, &Direction::Down)
    }

    fn is_visible_from(&self, pt: &Point, direction: &Direction) -> bool {
        let val = self.at(pt);
        let max_val = self.items_until(pt, direction).into_iter().max();
        match max_val {
            Some(n) => n < &val,
            None => true,
        }
    }

    fn items_until(&self, pt: &Point, direction: &Direction) -> Vec<&u8> {
        let Point { x, y } = *pt;
        match direction {
            Direction::Left => self.0.get(x).unwrap().iter().take(y).collect(),
            Direction::Right => self.0.get(x).unwrap().iter().skip(y + 1).collect(),
            Direction::Up => self.0.iter().map(|row| &row[y]).take(x).collect(),
            Direction::Down => self.0.iter().map(|row| &row[y]).skip(x + 1).collect(),
        }
    }
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn count_visible_trees(filename: &str) -> u32 {
    let grid = Grid::from(filename);
    let mut visible_count = 0;
    for (x, row) in grid.0.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            let pt = Point { x, y };
            if grid.is_visible(&pt) {
                visible_count += 1;
            }
        }
    }
    visible_count
}

pub fn max_scenic_score(filename: &str) -> u32 {
    let grid = Grid::from(filename);
    let positions: Vec<Point> = grid
        .0
        .iter()
        .enumerate()
        .flat_map(|(x, row)| row.iter().enumerate().map(move |(y, _)| Point { x, y }))
        .collect();

    positions
        .iter()
        .map(|position| grid.scenic_score(position))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{day08, fetch_input};

    #[test]
    fn count_visible_trees() {
        fetch_input(8);

        let tests = vec![("example/day08.txt", 21), ("input/day08.txt", 1715)];

        for test in tests {
            let (filename, want) = test;
            let got = day08::count_visible_trees(filename);
            assert_eq!(got, want, "got {got}, wanted {want}, for {filename}")
        }
    }

    #[test]
    fn max_scenic_score() {
        fetch_input(8);

        let tests = vec![("example/day08.txt", 8), ("input/day08.txt", 374400)];

        for test in tests {
            let (filename, want) = test;
            let got = day08::max_scenic_score(filename);
            assert_eq!(got, want, "got {got}, wanted {want}, for {filename}")
        }
    }
}
