/* use std::collections::HashMap;

pub struct Stacks {
    crates: HashMap<i32, Vec<String>>,
    steps: Vec<Instruction>,
}

struct Instruction {
    from: i32,
    to: i32,
    num: i32,
}

fn parse_input(input: &str) -> Stacks {
    Stacks {
        crates: HashMap::new(),
        steps: vec![],
    }
}

pub fn find_top_crates(file: &str) -> &'static str {
    &"XXX"
}

#[cfg(test)]
mod tests {
    use crate::day5::crates;
    use crate::fetch;

    #[test]
    fn find_top_crates() {
        fetch::input(4);

        let tests = vec![("src/day4/example.txt", 2), ("src/day4/input.txt", 569)];

        for test in tests {
            let (file, want) = test;
            let got = crates::find_top_crates(file);
            assert_eq!(want, got, "want {want}, got {got}, for {file}");
        }
    }
} */
