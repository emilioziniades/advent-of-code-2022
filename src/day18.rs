use std::{
    collections::{HashSet, VecDeque},
    fs,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl Cube {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Cube { x, y, z }
    }

    fn neighbours(&self) -> Vec<Self> {
        vec![
            Cube::new(self.x + 1, self.y, self.z),
            Cube::new(self.x - 1, self.y, self.z),
            Cube::new(self.x, self.y + 1, self.z),
            Cube::new(self.x, self.y - 1, self.z),
            Cube::new(self.x, self.y, self.z + 1),
            Cube::new(self.x, self.y, self.z - 1),
        ]
    }
}

impl From<&str> for Cube {
    fn from(line: &str) -> Self {
        let coordinates: Vec<isize> = line.split(',').map(|n| n.parse().unwrap()).collect();
        Self {
            x: coordinates[0],
            y: coordinates[1],
            z: coordinates[2],
        }
    }
}

struct Cubes(HashSet<Cube>);

impl Cubes {
    fn from_file(file: &str) -> Self {
        Self(
            fs::read_to_string(file)
                .unwrap()
                .lines()
                .map(Cube::from)
                .collect(),
        )
    }

    fn count_sides_touching(&self, cube_set: &Cubes) -> usize {
        self.0
            .iter()
            .map(|cube| {
                cube.neighbours()
                    .iter()
                    .filter(|neighbour| cube_set.0.contains(neighbour))
                    .count()
            })
            .sum()
    }

    fn surface_area(&self) -> usize {
        let total_sides = self.0.len() * 6;
        let sides_touching = self.count_sides_touching(self);
        total_sides - sides_touching
    }
}

pub fn surface_area(file: &str) -> usize {
    Cubes::from_file(file).surface_area()
}

pub fn external_surface_area(file: &str) -> usize {
    let cubes = Cubes::from_file(file);

    // find bounding cube

    let mut max_x = isize::MIN;
    let mut min_x = isize::MAX;
    let mut max_y = isize::MIN;
    let mut min_y = isize::MAX;
    let mut max_z = isize::MIN;
    let mut min_z = isize::MAX;

    for cube in cubes.0.iter() {
        if cube.x > max_x {
            max_x = cube.x;
        }
        if cube.x < min_x {
            min_x = cube.x
        }
        if cube.y > max_y {
            max_y = cube.y;
        }
        if cube.y < min_y {
            min_y = cube.y
        }
        if cube.z > max_z {
            max_z = cube.z;
        }
        if cube.z < min_z {
            min_z = cube.z
        }
    }

    // add one layer of space

    let max_x = max_x + 1;
    let min_x = min_x - 1;
    let max_y = max_y + 1;
    let min_y = min_y - 1;
    let max_z = max_z + 1;
    let min_z = min_z - 1;

    // floodfill

    let start = Cube::new(min_x, min_y, min_z);

    let mut frontier = VecDeque::new();
    let mut exterior_spaces = HashSet::new();

    frontier.push_back(start.clone());
    exterior_spaces.insert(start);

    while let Some(current) = frontier.pop_front() {
        for next in current.neighbours() {
            let out_of_bounds = next.x < min_x
                || next.x > max_x
                || next.y < min_y
                || next.y > max_y
                || next.z < min_z
                || next.z > max_z;

            if out_of_bounds || cubes.0.contains(&next) || exterior_spaces.contains(&next) {
                continue;
            }

            frontier.push_back(next.clone());
            exterior_spaces.insert(next);
        }
    }

    // determine internal spaces

    let mut all_cubes = HashSet::new();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                all_cubes.insert(Cube::new(x, y, z));
            }
        }
    }

    let all_spaces: HashSet<Cube> = all_cubes.difference(&cubes.0).cloned().collect();
    let interior_spaces: HashSet<Cube> = all_spaces.difference(&exterior_spaces).cloned().collect();

    let interior_spaces = Cubes(interior_spaces);

    let surface_area = cubes.surface_area();
    let interior_surface_area = interior_spaces.count_sides_touching(&cubes);

    surface_area - interior_surface_area
}

#[cfg(test)]
mod tests {
    use crate::{day18, fetch_input};

    #[test]
    fn surface_area() {
        fetch_input(18);
        let tests = vec![("example/day18.txt", 64), ("input/day18.txt", 3412)];

        for (infile, want) in tests {
            let got = day18::surface_area(infile);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }

    #[test]
    fn external_surface_area() {
        fetch_input(18);
        let tests = vec![("example/day18.txt", 58), ("input/day18.txt", 2018)];

        for (infile, want) in tests {
            let got = day18::external_surface_area(infile);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }
}
