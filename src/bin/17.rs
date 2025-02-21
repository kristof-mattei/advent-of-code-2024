use advent_of_code_2024::shared::{PartSolution, Parts};

advent_of_code_2024::solution!("3,4,3,1,7,6,5,6,0", 775_457_178);

#[derive(Clone, Copy)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<&OpCode> for u32 {
    fn from(value: &OpCode) -> Self {
        *value as u32
    }
}

impl From<OpCode> for u32 {
    fn from(value: OpCode) -> Self {
        value as u32
    }
}

impl TryFrom<u32> for OpCode {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok([
            OpCode::Adv,
            OpCode::Bxl,
            OpCode::Bst,
            OpCode::Jnz,
            OpCode::Bxc,
            OpCode::Out,
            OpCode::Bdv,
            OpCode::Cdv,
        ][usize::try_from(value).unwrap()])
    }
}

#[derive(Clone, Copy)]
struct Instruction {
    opcode: OpCode,
    operand: u32,
}

impl Instruction {
    fn to_raw(self) -> [u32; 2] {
        [self.opcode as u32, self.operand]
    }
}

fn parse_instructions(program: &[u32]) -> Vec<Instruction> {
    let mut parsed = Vec::with_capacity(program.len() / 2);

    for instruction in program.chunks(2) {
        let opcode: OpCode = instruction[0].try_into().unwrap();

        let operand = instruction[1];

        parsed.push(Instruction { opcode, operand });
    }

    parsed
}

#[derive(Default, Clone)]
struct State {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    index: usize,
    instructions: Vec<Instruction>,
    outputs: Vec<u64>,
}

impl State {
    fn output(&mut self, value: u64) {
        self.outputs.push(value);
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Register A: {}", self.register_a)?;
        writeln!(f, "Register B: {}", self.register_b)?;
        writeln!(f, "Register C: {}", self.register_c)?;
        writeln!(f)?;
        let instructions_original = self
            .instructions
            .iter()
            .map(|Instruction { opcode, operand }| {
                format!("{},{}", Into::<u32>::into(opcode), operand)
            })
            .collect::<Vec<String>>();

        writeln!(f, "Program: {}", instructions_original.join(","))
    }
}

fn parse_input(input: &str) -> State {
    let mut lines = input.trim().lines();

    let register_a = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap();
    let register_b = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap();
    let register_c = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap();

    let _empty = lines.next().unwrap();

    let instructions = parse_instructions(
        &lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(',')
            .map(|c| c.parse::<u32>().unwrap())
            .collect::<Vec<u32>>(),
    );

    State {
        register_a,
        register_b,
        register_c,
        instructions,
        ..State::default()
    }
}

fn parse_operand(state: &State, opcode: OpCode, operand: u32) -> u64 {
    match opcode {
        // 1 & 3
        OpCode::Bxl | OpCode::Jnz => operand.into(),
        // 0, 2, 4 -> 7
        OpCode::Adv | OpCode::Bst | OpCode::Bxc | OpCode::Out | OpCode::Bdv | OpCode::Cdv => {
            match operand {
                0..=3 => operand.into(),
                4 => state.register_a,
                5 => state.register_b,
                6 => state.register_c,
                _ => panic!("Invalid opcode / operand combo"),
            }
        },
    }
}

fn execute(state: &mut State) -> Vec<u64> {
    while let Some(Instruction { opcode, operand }) = state.instructions.get(state.index) {
        let operand = parse_operand(state, *opcode, *operand);

        match opcode {
            OpCode::Adv => {
                state.register_a >>= operand;
            },
            OpCode::Bxl => {
                state.register_b ^= operand;
            },
            OpCode::Bst => state.register_b = operand % 8,
            OpCode::Jnz => {
                if state.register_a != 0 {
                    state.index = usize::try_from(operand).expect("Operand too large") / 2;
                    continue;
                }
            },
            OpCode::Bxc => {
                state.register_b ^= state.register_c;
            },
            OpCode::Out => {
                state.output(operand % 8);
            },
            OpCode::Bdv => {
                state.register_b = state.register_a >> operand;
            },
            OpCode::Cdv => {
                state.register_c = state.register_a >> operand;
            },
        }

        state.index += 1;
    }

    state.outputs.clone()
}

fn execute_program(input: &str) -> PartSolution {
    let mut input = parse_input(input);

    let result = execute(&mut input);

    result
        .iter()
        .map(|&o| o.to_string())
        .collect::<Vec<String>>()
        .join(",")
        .into()
}

fn execute_program_util_match(input: &str) -> PartSolution {
    let input = parse_input(input);

    input
        .instructions
        .iter()
        .flat_map(|instruction: &Instruction| Instruction::to_raw(*instruction))
        .rev()
        .fold(vec![0u64], |candidates, instruction| {
            candidates
                .iter()
                .flat_map(|&candidate| {
                    // for each candidate shift to the left and then try all (candidate + each 0..8)
                    // whether that number in register A produces the expected instruction
                    let next = candidate << 3;

                    (next..next + 8).filter(|&possibility| {
                        let mut clone = input.clone();
                        clone.register_a = possibility;

                        let output = execute(&mut clone);

                        output.first() == Some(&instruction.into())
                    })
                })
                .collect::<Vec<_>>()
        })
        .first()
        .copied()
        .unwrap_or(0)
        .into()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        execute_program(input)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        execute_program_util_match(input)
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2024::shared::solution::read_file;
        use advent_of_code_2024::shared::{PartSolution, Parts};

        use crate::{DAY, Solution, State, execute, parse_instructions};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::String("3,4,3,1,7,6,5,6,0".into()),
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example_1() {
            assert_eq!(
                PartSolution::String("4,6,3,5,6,3,5,2,1,0".into()),
                (Solution {}).part_1(&read_file("examples", &DAY))
            );
        }

        #[test]
        fn example_2() {
            let mut state = State {
                register_c: 9,
                instructions: parse_instructions(&[2, 6]),
                ..Default::default()
            };

            let _result = execute(&mut state);

            assert_eq!(state.register_b, 1);
        }

        #[test]
        fn example_3() {
            let mut state = State {
                register_a: 10,
                instructions: parse_instructions(&[5, 0, 5, 1, 5, 4]),
                ..Default::default()
            };

            let result = execute(&mut state);

            assert_eq!(&[0, 1, 2][..], &result);
        }

        #[test]
        fn example_4() {
            let mut state = State {
                register_a: 2024,
                instructions: parse_instructions(&[0, 1, 5, 4, 3, 0]),
                ..Default::default()
            };

            let result = execute(&mut state);

            assert_eq!(&[4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0][..], &result);

            assert_eq!(state.register_a, 0);
        }

        #[test]
        fn example_5() {
            let mut state = State {
                register_b: 29,
                instructions: parse_instructions(&[1, 7]),
                ..Default::default()
            };

            let _result = execute(&mut state);

            assert_eq!(state.register_b, 26);
        }

        #[test]
        fn example_6() {
            let mut state = State {
                register_b: 2024,
                register_c: 43690,
                instructions: parse_instructions(&[4, 0]),
                ..Default::default()
            };

            let _result = execute(&mut state);

            assert_eq!(state.register_b, 44354);
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::Parts;
        use advent_of_code_2024::shared::solution::read_file;

        use crate::{DAY, Solution, execute, execute_program_util_match, parse_input};

        #[test]
        fn outcome() {
            let expected = 109_019_930_331_546;
            assert_eq!(expected, (Solution {}).part_2(&read_file("inputs", &DAY)));

            let mut program = parse_input(read_file("inputs", &DAY).as_str());
            program.register_a = expected;

            let result = execute(&mut program);

            assert_eq!(
                result,
                program
                    .instructions
                    .iter()
                    .flat_map(|i| i.to_raw())
                    .map(Into::into)
                    .collect::<Vec<u64>>()
            );
        }

        #[test]
        fn example() {
            assert_eq!(29328, (Solution {}).part_2(&read_file("examples", &DAY)));
        }

        #[test]
        fn example_2() {
            let input = r"Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

            assert_eq!(117_440, execute_program_util_match(input));
        }
    }
}
