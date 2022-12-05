use std::{collections::HashSet, fs};

use itertools::Itertools;

pub fn count_overlap_priority(file: &str) -> i32 {
    fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(|line| find_overlap(line) as i32)
        .sum()
}

pub fn count_group_priority(file: &str) -> i32 {
    fs::read_to_string(file)
        .unwrap()
        .lines()
        .chunks(3)
        .into_iter()
        .map(|group| {
            intersect_all(
                group
                    .into_iter()
                    .map(|row| row.bytes().collect::<HashSet<u8>>())
                    .collect::<Vec<_>>(),
            )
        })
        .into_iter()
        .map(|set| priority(set.iter().next().unwrap()) as i32)
        .sum()
}

fn intersect_all(mut sets: Vec<HashSet<u8>>) -> HashSet<u8> {
    if sets.is_empty() {
        return HashSet::new();
    }

    if sets.len() == 1 {
        return sets.pop().unwrap();
    }

    let mut result = sets.pop().unwrap();

    result.retain(|item| sets.iter().all(|set| set.contains(item)));

    result
}

fn find_overlap(rucksacks: &str) -> u8 {
    let (set_one, set_two): (HashSet<u8>, HashSet<u8>) = {
        let midway = rucksacks.len() / 2;
        let (bag_one, bag_two) = rucksacks.split_at(midway);
        (bag_one.bytes().collect(), bag_two.bytes().collect())
    };

    let item = intersect_all(vec![set_one, set_two]);
    let item = item.iter().collect::<Vec<_>>();
    let item = item.first().unwrap();

    priority(item)
}

fn priority(i: &u8) -> u8 {
    match i > &90 {
        // lowercase
        true => *i - 96,
        // uppercase
        false => *i - 38,
    }
}

#[cfg(test)]
mod tests {
    use crate::{day3, fetch};

    #[test]
    fn count_total_score() {
        fetch::input(3);

        let tests = vec![("example/day03.txt", 157), ("input/day03.txt", 7793)];

        for test in tests {
            let (file, want) = test;
            let got = day3::count_overlap_priority(file);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }

    #[test]
    fn count_total_group_score() {
        fetch::input(3);

        let tests = vec![("example/day03.txt", 70), ("input/day03.txt", 2499)];

        for test in tests {
            let (file, want) = test;
            let got = day3::count_group_priority(file);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }
}
