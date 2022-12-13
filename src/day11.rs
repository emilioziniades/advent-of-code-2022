use std::{collections::VecDeque, fs};

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u32>,
    operation: String,
    test_divisor: u32,
    test_true_monkey: usize,
    test_false_monkey: usize,
    // test: fn(monk),
}

impl Monkey {
    fn from(lines: &str) -> Self {
        /* Expects input in this form
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3
         */
        let mut lines = lines.lines().skip(1); // skip "Monkey n:"

        let items = lines.next().expect("another line");
        let items: VecDeque<u32> = items
            .trim()
            .strip_prefix("Starting items: ")
            .expect("starts with Starting items:")
            .replace(',', "")
            .split_whitespace()
            .map(|n| n.parse().expect("a number"))
            .collect();

        let operation = lines
            .next()
            .expect("another line")
            .trim()
            .strip_prefix("Operation:")
            .expect("starts with Operation:")
            .trim()
            .to_string();

        let test_divisor: u32 = lines
            .next()
            .expect("another line")
            .trim()
            .strip_prefix("Test: divisible by ")
            .expect("starts with Test: divisible by")
            .parse()
            .expect("a number");

        let test_true_monkey: usize = lines
            .next()
            .expect("another line")
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .expect("starts with If true: throw to monkey")
            .parse()
            .expect("a number");

        let test_false_monkey: usize = lines
            .next()
            .expect("another line")
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .expect("starts with If false: throw to monkey")
            .parse()
            .expect("a number");

        Monkey {
            items,
            operation,
            test_divisor,
            test_true_monkey,
            test_false_monkey,
        }
    }

    fn do_operation(&self, old: u32) -> u32 {
        let operation: Vec<&str> = self.operation.split_whitespace().collect();
        match operation[..] {
            ["new", "=", "old", "*", "old"] => old * old,
            ["new", "=", "old", "+", "old"] => old + old,
            ["new", "=", "old", "*", n] => {
                let n: u32 = n.parse().expect("a number");
                old * n
            }
            ["new", "=", "old", "+", n] => {
                let n: u32 = n.parse().expect("a number");
                old + n
            }
            [..] => panic!("unrecognized operation {operation:?}"),
        }
    }
}

#[derive(Debug)]
struct Monkeys {
    m: Vec<Monkey>,
    count: Vec<u32>,
}

impl Monkeys {
    fn from(filename: &str) -> Self {
        let monkeys = fs::read_to_string(filename).unwrap();
        let monkeys: Vec<Monkey> = monkeys.split("\n\n").map(Monkey::from).collect();
        let n_monkeys = monkeys.len();
        Self {
            m: monkeys,
            count: (0..n_monkeys).map(|_| 0).collect(),
        }
    }
    fn do_round(&mut self, div_by_three: bool) {
        for i in 0..self.m.len() {
            while let Some(old_item) = self.m[i].items.pop_front() {
                self.count[i] += 1;

                let mut new_item = self.m[i].do_operation(old_item);

                if div_by_three {
                    new_item /= 3;
                }

                let target_monkey_id = match new_item % self.m[i].test_divisor == 0 {
                    true => self.m[i].test_true_monkey,
                    false => self.m[i].test_false_monkey,
                };

                self.m[target_monkey_id].items.push_back(new_item);
            }
        }
    }

    fn do_rounds(&mut self, n: u32, div_by_three: bool) {
        for _ in 0..n {
            self.do_round(div_by_three);
        }
    }
}

pub fn measure_monkey_business(filename: &str, n_rounds: u32, div_by_three: bool) -> u32 {
    let mut monkeys = Monkeys::from(filename);
    monkeys.do_rounds(n_rounds, div_by_three);
    monkeys.count.sort_by(|a, b| b.cmp(a));
    monkeys.count[0] * monkeys.count[1]
}

#[cfg(test)]
mod tests {
    use crate::day11;
    use crate::fetch_input;

    #[test]
    fn measure_monkey_business() {
        fetch_input(11);
        let tests = vec![
            ("example/day11.txt", 20, true, 10605),
            ("input/day11.txt", 20, true, 58056),
        ];

        for test in tests {
            let (filename, n_rounds, div_by_three, want) = test;
            let got = day11::measure_monkey_business(filename, n_rounds, div_by_three);

            assert_eq!(got, want, "got {got}, wanted {want}, for {filename}");
        }
    }
}
