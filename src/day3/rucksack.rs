use std::{collections::HashSet, fs};

pub fn count_overlap_priority(file: &str) -> i32 {
    fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(|x| find_overlap(x) as i32)
        .sum()
}

pub fn count_group_priority(file: &str) -> i32 {
    let data = fs::read_to_string(file).unwrap();
    let mut lines = data.lines();
    let mut result: i32 = 0;
    loop {
        if let Ok([a, b, c]) = lines.next_chunk::<3>() {
            let (mut set_a, set_b, set_c): (HashSet<u8>, HashSet<u8>, HashSet<u8>) = (
                a.bytes().collect(),
                b.bytes().collect(),
                c.bytes().collect(),
            );

            set_a.retain(|i| {
                vec![set_b.clone(), set_c.clone()]
                    .iter()
                    .all(|set| set.contains(i))
            });
            let item = set_a.iter().next().unwrap();
            result += priority(item) as i32;
        } else {
            break;
        }
    }
    return result;
}

fn find_overlap(rucksacks: &str) -> u8 {
    let (set_one, set_two): (HashSet<u8>, HashSet<u8>) = {
        let midway = rucksacks.len() / 2;
        let (bag_one, bag_two) = rucksacks.split_at(midway);
        (bag_one.bytes().collect(), bag_two.bytes().collect())
    };

    let union: Vec<_> = set_one.intersection(&set_two).collect();
    assert_eq!(union.len(), 1);
    let item = union.iter().next().unwrap();
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
    use super::*;
    use crate::fetch;

    fn init() {
        fetch::input(3);
    }

    #[test]
    fn count_total_score() {
        init();

        let tests = vec![("src/day3/example.txt", 157), ("src/day3/input.txt", 7793)];

        for test in tests {
            let (file, want) = test;
            let got = count_overlap_priority(file);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }

    #[test]
    fn count_total_group_score() {
        init();

        let tests = vec![("src/day3/example.txt", 70), ("src/day3/input.txt", 0)];

        for test in tests {
            let (file, want) = test;
            let got = count_group_priority(file);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }
}
