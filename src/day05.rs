use std::{collections::HashMap, fs, str};

pub fn find_top_crates(file: &str, one_by_one: bool) -> String {
    let input = fs::read_to_string(file).unwrap();
    let (raw_crates, instructions) = input.split_once("\n\n").unwrap();

    let raw_crates = raw_crates.lines().map(|line| {
        (1..line.len())
            .step_by(4)
            .enumerate()
            .map(|(i, pos)| (i + 1, line.as_bytes()[pos]))
            .collect::<Vec<(usize, u8)>>()
    });

    let instructions: Vec<(u8, usize, usize)> = instructions
        .lines()
        .map(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();
            (
                //move
                words[1].parse().unwrap(),
                // from
                words[3].parse().unwrap(),
                // to
                words[5].parse().unwrap(),
            )
        })
        .collect();

    let mut crates: HashMap<usize, Vec<u8>> = HashMap::new();

    for line in raw_crates {
        for (i, item) in line {
            if (49..=57).contains(&item) || item == 32 {
                // skip 1..9 and space
                continue;
            }
            match crates.get_mut(&i) {
                Some(vec) => {
                    vec.insert(0, item);
                }
                None => {
                    crates.insert(i, vec![item]);
                }
            };
        }
    }

    for (n_moves, from, to) in instructions {
        if one_by_one {}
        let src = crates.get_mut(&from).unwrap();
        let mut items = src.split_off(src.len() - n_moves as usize);
        if one_by_one {
            items.reverse();
        }
        let dst = crates.get_mut(&to).unwrap();
        dst.append(&mut items);
    }

    let mut crates: Vec<(&usize, &Vec<u8>)> = crates.iter().collect();
    crates.sort();

    let result: Vec<u8> = crates
        .iter()
        .map(|(_, stack)| *stack.last().unwrap())
        .collect();

    str::from_utf8(&result).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use crate::{day05, fetch_input};

    #[test]
    fn find_top_crates() {
        fetch_input(5);

        let tests = vec![
            ("example/day05.txt", "CMZ", true),
            ("input/day05.txt", "JCMHLVGMG", true),
            ("example/day05.txt", "MCD", false),
            ("input/day05.txt", "LVMRWSSPZ", false),
        ];

        for test in tests {
            let (file, want, is_one_by_one) = test;
            let got = day05::find_top_crates(file, is_one_by_one);
            assert_eq!(want, got, "want {want}, got {got}, for {file}");
        }
    }
}
