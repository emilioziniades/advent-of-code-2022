use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs, iter, slice,
};

const N_ROUNDS: usize = 10;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn north(&self) -> Self {
        Self::new(self.x - 1, self.y)
    }

    fn north_east(&self) -> Self {
        Self::new(self.x - 1, self.y + 1)
    }

    fn east(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }

    fn south_east(&self) -> Self {
        Self::new(self.x + 1, self.y + 1)
    }

    fn south(&self) -> Self {
        Self::new(self.x + 1, self.y)
    }

    fn south_west(&self) -> Self {
        Self::new(self.x + 1, self.y - 1)
    }

    fn west(&self) -> Self {
        Self::new(self.x, self.y - 1)
    }

    fn north_west(&self) -> Self {
        Self::new(self.x - 1, self.y - 1)
    }

    fn neighbours(&self) -> [Self; 8] {
        [
            self.north(),
            self.north_east(),
            self.east(),
            self.south_east(),
            self.south(),
            self.south_west(),
            self.west(),
            self.north_west(),
        ]
    }
}

#[derive(Debug)]
struct Grove {
    elves: HashSet<Point>,
}

impl Grove {
    fn try_move_elf(&self, elf: &Point, direction: &Direction) -> Option<Point> {
        match direction {
            Direction::North => {
                if self.is_occupied([elf.north(), elf.north_east(), elf.north_west()]) {
                    None
                } else {
                    Some(elf.north())
                }
            }
            Direction::East => {
                if self.is_occupied([elf.east(), elf.north_east(), elf.south_east()]) {
                    None
                } else {
                    Some(elf.east())
                }
            }
            Direction::South => {
                if self.is_occupied([elf.south(), elf.south_west(), elf.south_east()]) {
                    None
                } else {
                    Some(elf.south())
                }
            }
            Direction::West => {
                if self.is_occupied([elf.west(), elf.south_west(), elf.north_west()]) {
                    None
                } else {
                    Some(elf.west())
                }
            }
        }
    }

    fn is_occupied(&self, points: [Point; 3]) -> bool {
        points.iter().any(|point| self.elves.contains(point))
    }

    fn move_elves(&mut self, legal_movements: HashMap<Point, Point>) {
        for (next_point, current_point) in legal_movements {
            self.elves.remove(&current_point);
            self.elves.insert(next_point);
        }
    }
}

impl Display for Grove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.elves.iter().max_by_key(|elf| elf.x).unwrap().x;
        let max_y = self.elves.iter().max_by_key(|elf| elf.y).unwrap().y;

        for x in 0..=max_x {
            for y in 0..=max_y {
                if self.elves.contains(&Point::new(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn cycle() -> iter::Cycle<slice::Iter<'static, Direction>> {
        [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
        .iter()
        .cycle()
    }
}

pub fn count_empty_ground_tiles(filename: &str) -> isize {
    let input = fs::read_to_string(filename).unwrap();

    let elves: HashSet<Point> = input
        .lines()
        .enumerate()
        .flat_map(|(x, row)| {
            row.trim().chars().enumerate().filter_map(move |(y, cell)| {
                if cell == '#' {
                    Some(Point::new(x.try_into().unwrap(), y.try_into().unwrap()))
                } else {
                    None
                }
            })
        })
        .collect();

    let mut grove = Grove { elves };

    // println!("initial_state: \n{grove}");
    for n in 0..N_ROUNDS {
        let mut proposed_movements: HashMap<Point, Vec<Point>> = HashMap::new();
        for elf in &grove.elves {
            if elf
                .neighbours()
                .iter()
                .all(|neighbour| !grove.elves.contains(neighbour))
            {
                continue;
            }

            for direction in Direction::cycle().skip(n).take(4) {
                if let Some(new_point) = grove.try_move_elf(elf, direction) {
                    proposed_movements.entry(new_point).or_default().push(*elf);
                    break;
                }
            }
        }

        let legal_movements: HashMap<Point, Point> = proposed_movements
            .iter()
            .filter(|(_dest, srcs)| srcs.len() == 1)
            .map(|(dest, srcs)| (*dest, srcs[0]))
            .collect();

        grove.move_elves(legal_movements);

        // println!("after round {}:\n{grove}", n + 1);
    }

    let largest_x = grove.elves.iter().max_by_key(|elf| elf.x).unwrap().x;
    let smallest_x = grove.elves.iter().min_by_key(|elf| elf.x).unwrap().x;

    let largest_y = grove.elves.iter().max_by_key(|elf| elf.y).unwrap().y;
    let smallest_y = grove.elves.iter().min_by_key(|elf| elf.y).unwrap().y;

    let grove_area = (largest_x - smallest_x + 1) * (largest_y - smallest_y + 1);

    grove_area - grove.elves.len() as isize
}

#[cfg(test)]
mod tests {
    use crate::{day23, fetch_input};

    #[test]
    fn surface_area() {
        fetch_input(23);
        let tests = vec![("example/day23.txt", 110), ("input/day23.txt", 0)];

        for (filename, want) in tests {
            let got = day23::count_empty_ground_tiles(filename);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }
}
