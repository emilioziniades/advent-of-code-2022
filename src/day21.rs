use std::{collections::HashMap, fs};

type ExpressionTable<'a> = HashMap<&'a str, Yell<'a>>;

#[derive(Debug)]
enum Yell<'a> {
    Number(usize),
    Variable,
    Expression {
        left: &'a str,
        operator: Operator,
        right: &'a str,
    },
}

impl<'a> From<&'a str> for Yell<'a> {
    fn from(value: &'a str) -> Self {
        let tokens: Vec<&str> = value.split_whitespace().collect();
        match &tokens[..] {
            [left, operator, right] => Yell::Expression {
                left,
                operator: Operator::from(*operator),
                right,
            },
            [number] => Yell::Number(number.parse().unwrap()),
            _ => panic!("invalid input"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl<'a> From<&'a str> for Operator {
    fn from(value: &'a str) -> Self {
        match value {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            _ => panic!("unrecognized operator"),
        }
    }
}

#[derive(Debug, Clone)]
enum Node {
    Number(usize),
    Variable,
    Expression {
        left: Box<Node>,
        operator: Operator,
        right: Box<Node>,
    },
}

pub fn find_root_number(filename: &str) -> usize {
    let file = fs::read_to_string(filename).unwrap();
    let expressions: ExpressionTable = file
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(left, right)| (left.trim(), Yell::from(right.trim())))
        .collect();

    let tree = parse_tree(&expressions, "root");
    evaluate_tree(&tree)
}

pub fn find_human_number(filename: &str) -> usize {
    let file = fs::read_to_string(filename).unwrap();
    let mut expressions: ExpressionTable = file
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(left, right)| (left.trim(), Yell::from(right.trim())))
        .collect();

    expressions.insert("humn", Yell::Variable);

    let tree = parse_tree(&expressions, "root");
    // println!("{tree:#?}");

    let (left_tree, right_tree) = match tree {
        Node::Number(_) => panic!("expected expression"),
        Node::Variable => panic!("expected expression"),
        Node::Expression { left, right, .. } => (Box::clone(&left), Box::clone(&right)),
    };

    let (number, variable_tree) = if node_has_variable(&left_tree) {
        (evaluate_tree(&right_tree), *left_tree)
    } else if node_has_variable(&right_tree) {
        (evaluate_tree(&left_tree), *right_tree)
    } else {
        panic!("neither tree leaf has variable")
    };

    solve_for_variable(variable_tree, number)
}

fn solve_for_variable(
    // the side of the tree with the variable in it
    variable_tree: Node,
    // the number on the other side of the tree we want to make the above equal to
    number: usize,
) -> usize {
    match variable_tree {
        Node::Number(_) => todo!(),
        Node::Variable => number,
        Node::Expression {
            left,
            operator,
            right,
        } => {
            if node_has_variable(&left) {
                let right = evaluate_tree(&right);
                let new_number = match operator {
                    Operator::Plus => number - right,
                    Operator::Minus => number + right,
                    Operator::Multiply => number / right,
                    Operator::Divide => number * right,
                };
                return solve_for_variable(*left, new_number);
            } else if node_has_variable(&right) {
                let left = evaluate_tree(&left);
                let new_number = match operator {
                    Operator::Plus => number - left,
                    Operator::Minus => number + left,
                    Operator::Multiply => number / left,
                    Operator::Divide => number * left,
                };
                return solve_for_variable(*right, new_number);
            } else {
                let left = evaluate_tree(&left);
                let right = evaluate_tree(&right);
                return match operator {
                    Operator::Plus => left + right,
                    Operator::Minus => left - right,
                    Operator::Multiply => left * right,
                    Operator::Divide => left / right,
                };
            }
        }
    }
}

fn parse_tree(expressions: &ExpressionTable, key: &'_ str) -> Node {
    let value = expressions.get(key).unwrap();

    match value {
        Yell::Number(n) => Node::Number(*n),
        Yell::Variable => Node::Variable,
        Yell::Expression {
            left,
            operator,
            right,
        } => Node::Expression {
            left: Box::new(parse_tree(expressions, left)),
            operator: *operator,
            right: Box::new(parse_tree(expressions, right)),
        },
    }
}

fn evaluate_tree(node: &Node) -> usize {
    match node {
        Node::Number(n) => *n,
        Node::Variable => panic!("did not expect variable"),
        Node::Expression {
            left,
            operator,
            right,
        } => {
            let left = evaluate_tree(left);
            let right = evaluate_tree(right);

            match operator {
                Operator::Plus => left + right,
                Operator::Minus => left - right,
                Operator::Multiply => left * right,
                Operator::Divide => left / right,
            }
        }
    }
}

fn node_has_variable(node: &Node) -> bool {
    match node {
        Node::Number(_) => false,
        Node::Variable => true,
        Node::Expression { left, right, .. } => node_has_variable(left) || node_has_variable(right),
    }
}

#[cfg(test)]
mod tests {
    use crate::{day21, fetch_input};

    #[test]
    fn find_root_number() {
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

    #[test]
    fn find_human_number() {
        fetch_input(21);

        // 7500329760670 <- too big
        // let tests = vec![("example/day21.txt", 301), ("input/day21.txt", 0)];
        let tests = vec![("example/day21.txt", 301)];

        for (infile, want) in tests {
            let got = day21::find_human_number(infile);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }
}
