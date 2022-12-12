use std::fs;

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn from(line: &str) -> Self {
        let line: Vec<&str> = line.split_whitespace().collect();
        match line[..] {
            ["noop"] => Self::Noop,
            ["addx", n] => Self::AddX(n.parse().unwrap()),
            [..] => panic!("unrecognized instruction"),
        }
    }

    fn from_file(filename: &str) -> Vec<Self> {
        fs::read_to_string(filename)
            .unwrap()
            .lines()
            .map(Self::from)
            .collect()
    }

    fn cycle_length(&self) -> u8 {
        match self {
            Self::Noop => 1,
            Self::AddX(_) => 2,
        }
    }
}

struct Cpu {
    cycles: i32,
    x: i32,
}

impl Cpu {
    fn new() -> Self {
        Self { cycles: 0, x: 1 }
    }

    fn record_signal_strength(&mut self, instructions: Vec<Instruction>) -> i32 {
        let mut total_signal_strength = 0;

        for instruction in instructions {
            for _ in 0..instruction.cycle_length() {
                self.cycles += 1;

                if (self.cycles - 20) % 40 == 0 {
                    total_signal_strength += self.cycles * self.x
                }
            }
            if let Instruction::AddX(addx) = instruction {
                self.x += addx
            }
        }

        total_signal_strength
    }

    fn draw_sprite(&mut self, instructions: Vec<Instruction>) -> String {
        let mut sprite = String::new();

        for instruction in instructions {
            for _ in 0..instruction.cycle_length() {
                self.cycles += 1;

                let crt_position = (self.cycles - 1) % 40;

                if crt_position >= self.x - 1 && crt_position <= self.x + 1 {
                    sprite.push('#');
                } else {
                    sprite.push('.');
                }

                if self.cycles % 40 == 0 {
                    sprite.push('\n');
                }
            }

            if let Instruction::AddX(addx) = instruction {
                self.x += addx
            }
        }

        sprite
    }
}

pub fn sum_signal_strengths(filename: &str) -> i32 {
    let mut cpu = Cpu::new();
    let instructions = Instruction::from_file(filename);
    cpu.record_signal_strength(instructions)
}

pub fn draw_sprite(filename: &str) -> String {
    let mut cpu = Cpu::new();
    let instructions = Instruction::from_file(filename);
    cpu.draw_sprite(instructions)
}

#[cfg(test)]
mod tests {
    use crate::day10;
    use crate::fetch_input;

    #[test]
    fn sum_signal_strengths() {
        fetch_input(10);
        let tests = vec![("example/day10.txt", 13140), ("input/day10.txt", 11820)];

        for test in tests {
            let (filename, want) = test;
            let got = day10::sum_signal_strengths(filename);

            assert_eq!(got, want, "got {got}, wanted {want}, for {filename}");
        }
    }

    #[test]
    fn draw_sprite() {
        fetch_input(10);

        let example_image = concat!(
            "##..##..##..##..##..##..##..##..##..##..\n",
            "###...###...###...###...###...###...###.\n",
            "####....####....####....####....####....\n",
            "#####.....#####.....#####.....#####.....\n",
            "######......######......######......####\n",
            "#######.......#######.......#######.....\n"
        );

        // EPJBRKAH
        let input_image = concat!(
            "####.###....##.###..###..#..#..##..#..#.\n",
            "#....#..#....#.#..#.#..#.#.#..#..#.#..#.\n",
            "###..#..#....#.###..#..#.##...#..#.####.\n",
            "#....###.....#.#..#.###..#.#..####.#..#.\n",
            "#....#....#..#.#..#.#.#..#.#..#..#.#..#.\n",
            "####.#.....##..###..#..#.#..#.#..#.#..#.\n",
        );
        let tests = vec![
            ("example/day10.txt", example_image),
            ("input/day10.txt", input_image),
        ];

        for test in tests {
            let (filename, want) = test;
            let got = day10::draw_sprite(filename);

            assert_eq!(got, want, "{filename}:\ngot:\n{got}\nwanted\n{want}");
        }
    }
}
