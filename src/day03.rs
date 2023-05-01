use std::{collections::HashSet, fs};

pub fn count_overlap_priority(file: &str) -> i32 {
    fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(|line| i32::from(find_overlap(line)))
        .sum()
}

pub fn count_group_priority(file: &str) -> i32 {
    fs::read_to_string(file)
        .unwrap()
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            intersect_all(
                group
                    .iter()
                    .map(|row| row.bytes().collect::<HashSet<u8>>())
                    .collect::<Vec<HashSet<u8>>>(),
            )
        })
        .map(|set| i32::from(priority(*set.iter().next().unwrap())))
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
    let item = item.iter().collect::<Vec<&u8>>();
    let item = item.first().unwrap();

    priority(**item)
}

fn priority(i: u8) -> u8 {
    if i > 90 {
        i - 96
    } else {
        i - 38
    }
}

#[cfg(test)]
mod tests {
    use crate::{day03, fetch_input};

    #[test]
    fn count_total_score() {
        fetch_input(3);

        let tests = vec![("example/day03.txt", 157), ("input/day03.txt", 7793)];

        for test in tests {
            let (file, want) = test;
            let got = day03::count_overlap_priority(file);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }

    #[test]
    fn count_total_group_score() {
        fetch_input(3);

        let tests = vec![("example/day03.txt", 70), ("input/day03.txt", 2499)];

        for test in tests {
            let (file, want) = test;
            let got = day03::count_group_priority(file);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }
}
