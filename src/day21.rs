use std::{collections::HashMap, fs};

type ExpressionTable = HashMap<String, Yell>;

#[derive(Debug)]
enum Yell {
    Number(usize),
    Variable,
    Expression {
        left: String,
        operator: Operator,
        right: String,
    },
}

impl From<String> for Yell {
    fn from(value: String) -> Self {
        let tokens: Vec<&str> = value.split_whitespace().collect();
        match &tokens[..] {
            [left, operator, right] => Yell::Expression {
                left: (*left).to_string(),
                operator: Operator::from(*operator),
                right: (*right).to_string(),
            },
            [number] => Yell::Number(number.parse().unwrap()),
            _ => panic!("invalid input"),
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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
    let mut expressions = parse_input(&file);
    let tree = parse_tree(&mut expressions, "root");
    evaluate_tree(&tree)
}

pub fn find_human_number(filename: &str) -> usize {
    let file = fs::read_to_string(filename).unwrap();
    let mut expressions = parse_input(&file);

    expressions.insert("humn".to_string(), Yell::Variable);

    let tree = parse_tree(&mut expressions, "root");
    let tree = prune_tree(tree);

    let (left_tree, right_tree) = match tree {
        Node::Expression { left, right, .. } => (left, right),
        Node::Number(_) | Node::Variable => panic!("expected expression"),
    };

    let (number, variable_tree) = match (*left_tree, *right_tree) {
        (
            Node::Number(n),
            Node::Expression {
                left,
                operator,
                right,
            },
        )
        | (
            Node::Expression {
                left,
                operator,
                right,
            },
            Node::Number(n),
        ) => (
            n,
            Node::Expression {
                left,
                operator,
                right,
            },
        ),
        (_, _) => panic!("unexpected top level branches"),
    };

    solve_for_human_number(variable_tree, number)
}

fn parse_input(file: &str) -> ExpressionTable {
    file.lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(left, right)| {
            (
                left.trim().to_string(),
                Yell::from(right.trim().to_string()),
            )
        })
        .collect()
}

fn parse_tree(expressions: &'_ mut ExpressionTable, key: &str) -> Node {
    let value = expressions.remove(key).unwrap();

    match value {
        Yell::Number(n) => Node::Number(n),
        Yell::Variable => Node::Variable,
        Yell::Expression {
            left,
            operator,
            right,
        } => Node::Expression {
            left: Box::new(parse_tree(expressions, &left)),
            operator,
            right: Box::new(parse_tree(expressions, &right)),
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

fn prune_tree(node: Node) -> Node {
    match node {
        Node::Number(n) => Node::Number(n),
        Node::Variable => Node::Variable,
        Node::Expression {
            left,
            operator,
            right,
        } => {
            if node_has_variable(&left) {
                let left = prune_tree(*left);
                let right = evaluate_tree(&right);
                Node::Expression {
                    left: Box::new(left),
                    operator,
                    right: Box::new(Node::Number(right)),
                }
            } else if node_has_variable(&right) {
                let left = evaluate_tree(&left);
                let right = prune_tree(*right);
                Node::Expression {
                    left: Box::new(Node::Number(left)),
                    operator,
                    right: Box::new(right),
                }
            } else {
                panic!("neither branch has variable");
            }
        }
    }
}

fn solve_for_human_number(
    // the side of the tree with the variable in it
    variable_tree: Node,
    // the number on the other side of the tree we want to make the above equal to
    number: usize,
) -> usize {
    match variable_tree {
        Node::Number(_) => panic!("not expecting number"),
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
                solve_for_human_number(*left, new_number)
            } else if node_has_variable(&right) {
                let left = evaluate_tree(&left);
                let new_number = match operator {
                    Operator::Plus => number - left,
                    Operator::Minus => left - number, // number = left - variable => varaible = left - number
                    Operator::Multiply => number / left,
                    Operator::Divide => left / number, // number = left / variable => variable = left / number
                };
                solve_for_human_number(*right, new_number)
            } else {
                panic!("neither branch has variable");
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

        let tests = vec![
            ("example/day21.txt", 301),
            ("input/day21.txt", 3453748220116),
        ];

        for (infile, want) in tests {
            let got = day21::find_human_number(infile);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }
}
