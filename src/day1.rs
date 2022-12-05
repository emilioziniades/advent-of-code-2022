use std::fs;

pub fn max_calories(file: &str, n_elves: usize) -> i32 {
    let data = fs::read_to_string(file).unwrap();
    let mut calories = data
        .trim()
        .split("\n\n")
        .map(|x| {
            x.trim()
                .split('\n')
                .map(|x| x.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    calories.sort_by(|a, b| b.cmp(a));
    calories.truncate(n_elves);
    calories.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fetch;

    fn init() {
        fetch::input(1);
    }

    #[test]
    fn max_calories_elf() {
        init();

        let tests = vec![
            ("example/day1.txt", 24000, 1),
            ("example/day1.txt", 45000, 3),
            ("input/day1.txt", 69693, 1),
            ("input/day1.txt", 200945, 3),
        ];

        for test in tests {
            let (file, want, n_elves) = test;
            let got = max_calories(file, n_elves);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }
}
