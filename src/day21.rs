use std::{collections::HashMap, fs};

type ExpressionTable<'a> = HashMap<&'a str, Yell<'a>>;

#[derive(Debug)]
enum Yell<'a> {
    Number(usize),
    Expression {
        left: &'a str,
        operator: &'a str,
        right: &'a str,
    },
}

impl<'a> From<&'a str> for Yell<'a> {
    fn from(value: &'a str) -> Self {
        let tokens: Vec<&str> = value.split_whitespace().collect();
        match &tokens[..] {
            [left, operator, right] => Yell::Expression {
                left,
                operator,
                right,
            },
            [number] => Yell::Number(number.parse().unwrap()),
            _ => panic!("invalid input"),
        }
    }
}
pub fn find_root_number(filename: &str) -> usize {
    let file = fs::read_to_string(filename).unwrap();
    let expressions: ExpressionTable = file
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(left, right)| (left.trim(), Yell::from(right.trim())))
        .collect();
    println!("{expressions:#?}");

    evaluate_expression(&expressions, "root")
}

fn evaluate_expression(expressions: &ExpressionTable, key: &'_ str) -> usize {
    let value = expressions.get(key).unwrap();

    match value {
        Yell::Number(n) => *n,
        Yell::Expression {
            left,
            operator,
            right,
        } => {
            let left = evaluate_expression(expressions, left);
            let right = evaluate_expression(expressions, right);

            match *operator {
                "+" => left + right,
                "*" => left * right,
                "-" => left - right,
                "/" => left / right,
                _ => panic!("invalid operator"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{day21, fetch_input};

    #[test]
    fn mix_once() {
        fetch_input(21);

        let tests = vec![
            ("example/day21.txt", 152),
            ("input/day21.txt", 21120928600114),
        ];

        for (infile, want) in tests {
            let got = day21::find_root_number(infile);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }
}
