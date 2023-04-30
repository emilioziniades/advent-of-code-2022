use std::{collections::HashSet, fs, ops::Not};

pub fn count_assignment_overlaps(
    file: &str,
    filter_func: fn(&(HashSet<i32>, HashSet<i32>)) -> bool,
) -> i32 {
    i32::try_from(
        fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(parse_ranges)
            .filter(filter_func)
            .count(),
    )
    .unwrap()
}

pub fn has_subset((set_one, set_two): &(HashSet<i32>, HashSet<i32>)) -> bool {
    set_one.is_subset(set_two) || set_two.is_subset(set_one)
}

pub fn has_overlap((set_one, set_two): &(HashSet<i32>, HashSet<i32>)) -> bool {
    set_one
        .intersection(set_two)
        .collect::<HashSet<&i32>>()
        .is_empty()
        .not()
}

fn parse_ranges(line: &str) -> (HashSet<i32>, HashSet<i32>) {
    let numbers: Vec<i32> = line
        .replace(',', "-")
        .split('-')
        .map(|n| n.parse().unwrap())
        .collect();
    let (start_one, end_one, start_two, end_two) = (numbers[0], numbers[1], numbers[2], numbers[3]);
    (
        (start_one..=end_one).collect(),
        (start_two..=end_two).collect(),
    )
}

#[cfg(test)]
mod tests {
    use crate::{day04, fetch_input};

    #[test]
    fn compare_assignments_subsets() {
        fetch_input(4);

        let tests = vec![("example/day04.txt", 2), ("input/day04.txt", 569)];

        for test in tests {
            let (file, want) = test;
            let got = day04::count_assignment_overlaps(file, day04::has_subset);
            assert_eq!(want, got, "want {want}, got {got}, for {file}");
        }
    }

    #[test]
    fn compare_assignments_overlaps() {
        fetch_input(4);

        let tests = vec![("example/day04.txt", 4), ("input/day04.txt", 936)];

        for test in tests {
            let (file, want) = test;
            let got = day04::count_assignment_overlaps(file, day04::has_overlap);
            assert_eq!(want, got, "want {want}, got {got}, for {file}");
        }
    }
}
