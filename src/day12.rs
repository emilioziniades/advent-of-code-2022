use std::{collections::HashMap, fs, str};

use crate::queue::PriorityQueue;

// ascii encodings of S, E, a and z
const S: u8 = 83;
const E: u8 = 69;
const A: u8 = 97; //lowercase a
const Z: u8 = 122; // lowercase z

const COST: usize = 1;

#[derive(Debug)]
struct Graph {
    neighbours: HashMap<Point, Vec<Point>>,
    start: Point,
    end: Point,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Graph {
    fn from_file(filename: &str) -> Self {
        let file = fs::read_to_string(filename).expect("file exists");
        let mut grid: Vec<Vec<u8>> = file.lines().map(|line| line.as_bytes().to_vec()).collect();

        let rows = grid.len();
        let cols = grid[0].len();

        let mut start = Point::new(0, 0);
        let mut end = Point::new(0, 0);
        let mut neighbours: HashMap<Point, Vec<Point>> = HashMap::new();
        for (y, row) in grid.iter().enumerate() {
            for (x, letter) in row.iter().enumerate() {
                if letter == &S {
                    start = Point::new(x, y);
                }
                if letter == &E {
                    end = Point::new(x, y)
                }
            }
        }

        grid[start.y][start.x] = A;
        grid[end.y][end.x] = Z;

        for (y, row) in grid.iter().enumerate() {
            for (x, letter) in row.iter().enumerate() {
                let current_pt = Point::new(x, y);

                let current_neighbours: Vec<Point> = current_pt
                    .neighbours(cols, rows)
                    .into_iter()
                    .filter(|pt| grid[pt.y][pt.x] <= letter + 1)
                    .collect();

                neighbours.insert(current_pt, current_neighbours);
            }
        }

        Self {
            neighbours,
            start,
            end,
        }
    }

    // A-star with manhattan distance as heuristic
    fn find_shortest_path(&self) -> Vec<Point> {
        let mut frontier: PriorityQueue<Point> = PriorityQueue::new();
        frontier.push(self.start, 0);

        let mut came_from: HashMap<Point, Point> = HashMap::new();

        let mut cost_so_far: HashMap<Point, usize> = HashMap::new();
        cost_so_far.insert(self.start, 0);

        while let Some(current) = frontier.pop() {
            if current == self.end {
                break;
            }

            let neighbours = match self.neighbours.get(&current) {
                Some(n) => n,
                None => panic!("no neighbours for {current:?}"),
            };

            for neighbour in neighbours {
                let new_cost = cost_so_far.get(&current).unwrap() + COST;

                if cost_so_far.get(neighbour).unwrap_or(&usize::MAX) > &new_cost {
                    cost_so_far.insert(*neighbour, new_cost);
                    let priority = new_cost + neighbour.manhattan_distance(self.end);
                    came_from.insert(*neighbour, current);
                    frontier.push(*neighbour, priority);
                }
            }
        }

        self.recreate_path(&mut came_from)
    }

    fn recreate_path(&self, came_from: &mut HashMap<Point, Point>) -> Vec<Point> {
        let mut current = self.end;
        let mut path: Vec<Point> = Vec::new();
        while current != self.start {
            path.push(current);
            current = *came_from.get(&current).unwrap();
        }

        path.push(self.start);

        path
    }
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    fn manhattan_distance(&self, other: Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn neighbours(&self, cols: usize, rows: usize) -> Vec<Self> {
        let (cols, rows) = (cols as isize, rows as isize);

        let deltas = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        deltas
            .iter()
            .filter_map(|(dx, dy)| {
                let x_n = self.x as isize + dx;
                let y_n = self.y as isize + dy;
                match [x_n >= 0, x_n < cols, y_n >= 0, y_n < rows]
                    .into_iter()
                    .all(|b| b)
                {
                    true => Some(Point {
                        x: x_n as usize,
                        y: y_n as usize,
                    }),
                    _ => None,
                }
            })
            .collect()
    }
}

pub fn find_shortest_path(filename: &str) -> usize {
    let graph = Graph::from_file(filename);
    graph.find_shortest_path().len() - 1
}

#[cfg(test)]
mod tests {
    use crate::day12;
    use crate::fetch_input;

    #[test]
    fn find_shortest_path() {
        fetch_input(12);
        let tests = vec![("example/day12.txt", 31), ("input/day12.txt", 462)];

        for test in tests {
            let (filename, want) = test;
            let got = day12::find_shortest_path(filename);

            assert_eq!(got, want, "got {got}, wanted {want}, for {filename}");
        }
    }
}
