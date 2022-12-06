use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

type C = Choice;

enum Outcome {
    Lose,
    Win,
    Draw,
}

type O = Outcome;

#[derive(Debug, PartialEq)]
struct Game {
    me: Choice,
    you: Choice,
}

type G = Game;

impl Game {
    fn outcome(&self) -> Outcome {
        match self {
            G {
                me: C::Rock,
                you: C::Scissors,
            } => O::Win,
            G {
                me: C::Rock,
                you: C::Paper,
            } => O::Lose,
            G {
                me: C::Paper,
                you: C::Scissors,
            } => O::Lose,
            G {
                me: C::Paper,
                you: C::Rock,
            } => O::Win,
            G {
                me: C::Scissors,
                you: C::Paper,
            } => O::Win,
            G {
                me: C::Scissors,
                you: C::Rock,
            } => O::Lose,
            _ => O::Draw,
        }
    }

    fn score(&self) -> i32 {
        let outcome_pts = match self.outcome() {
            O::Win => 6,
            O::Draw => 3,
            O::Lose => 0,
        };

        let choice_pts = match self.me {
            C::Rock => 1,
            C::Paper => 2,
            C::Scissors => 3,
        };

        outcome_pts + choice_pts
    }
}

fn parse_round_into_choice(s: &str) -> Game {
    // expects string of the form "i j"
    // where i = A, B or C and j = X, Y, or Z
    // i = your choice, j = my choice
    let mut encodings = HashMap::new();
    encodings.insert("A", C::Rock);
    encodings.insert("B", C::Paper);
    encodings.insert("C", C::Scissors);
    encodings.insert("X", C::Rock);
    encodings.insert("Y", C::Paper);
    encodings.insert("Z", C::Scissors);

    let (i, j) = {
        let choices: Vec<&str> = s.split_whitespace().collect();
        (choices[0], choices[1])
    };

    Game {
        me: encodings.remove(j).unwrap(),
        you: encodings.remove(i).unwrap(),
    }
}

fn parse_round_into_desired_outcome(s: &str) -> Game {
    // expects string of the form "i j"
    // where i = A, B or C and j = X, Y, or Z
    // i = your choice, j = outcome that should happen
    let mut encoding_choice = HashMap::new();
    encoding_choice.insert("A", C::Rock);
    encoding_choice.insert("B", C::Paper);
    encoding_choice.insert("C", C::Scissors);

    let mut encoding_outcome = HashMap::new();
    encoding_outcome.insert("X", O::Lose);
    encoding_outcome.insert("Y", O::Draw);
    encoding_outcome.insert("Z", O::Win);

    let (i, j) = {
        let choices: Vec<&str> = s.split_whitespace().collect();
        (choices[0], choices[1])
    };

    let your_choice = encoding_choice.remove(i).unwrap();
    let desired_outcome = encoding_outcome.remove(j).unwrap();
    let my_choice = match desired_outcome {
        O::Win => loses(&your_choice),
        O::Draw => draws(&your_choice),
        O::Lose => beats(&your_choice),
    };

    Game {
        me: my_choice,
        you: your_choice,
    }
}

fn draws(c: &Choice) -> Choice {
    match c {
        C::Rock => C::Rock,
        C::Paper => C::Paper,
        C::Scissors => C::Scissors,
    }
}

fn loses(c: &Choice) -> Choice {
    match c {
        C::Rock => C::Paper,
        C::Paper => C::Scissors,
        C::Scissors => C::Rock,
    }
}

fn beats(c: &Choice) -> Choice {
    match c {
        C::Rock => C::Scissors,
        C::Paper => C::Rock,
        C::Scissors => C::Paper,
    }
}

pub fn total_score(file: &str) -> i32 {
    fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(|x| parse_round_into_choice(x).score())
        .sum()
}

pub fn total_score_alternative(file: &str) -> i32 {
    fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(|x| parse_round_into_desired_outcome(x).score())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{
        day02,
        day02::{C, G},
        fetch_input,
    };

    #[test]
    fn count_total_score() {
        fetch_input(2);

        let tests = vec![("example/day02.txt", 15), ("input/day02.txt", 8392)];

        for test in tests {
            let (file, want) = test;
            let got = day02::total_score(file);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }

    #[test]
    fn other_encoding() {
        let tests = vec![
            (
                "A Y",
                G {
                    me: C::Rock,
                    you: C::Rock,
                },
            ),
            (
                "B X",
                G {
                    me: C::Rock,
                    you: C::Paper,
                },
            ),
        ];

        for test in tests {
            let (input, want) = test;
            let got = day02::parse_round_into_desired_outcome(input);
            assert_eq!(want, got, "want {want:?}, got {got:?}")
        }
    }

    #[test]
    fn count_total_score_with_other_encoding() {
        fetch_input(2);

        let tests = vec![("example/day02.txt", 12), ("input/day02.txt", 10116)];

        for test in tests {
            let (file, want) = test;
            let got = day02::total_score_alternative(file);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }
}
