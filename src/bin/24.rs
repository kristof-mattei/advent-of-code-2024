use std::collections::VecDeque;

use advent_of_code_2024::shared::{PartSolution, Parts};
use hashbrown::{HashMap, HashSet};

advent_of_code_2024::solution!(
    55_544_677_167_336_u64,
    PartSolution::String("gsd,kth,qnf,tbt,vpm,z12,z26,z32".into())
);

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Instruction {
    wire1: String,
    wire2: String,
    output: String,
    operator: Operator,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
#[repr(usize)]
enum Operator {
    And = 0,
    Or = 1,
    Xor = 2,
}

impl Instruction {
    fn execute(&self, state: &mut HashMap<String, bool>) -> bool {
        let Some(&input1_value) = state.get(self.wire1.as_str()) else {
            return false;
        };

        let Some(&input2_value) = state.get(self.wire2.as_str()) else {
            return false;
        };

        let value = match self.operator {
            Operator::And => input1_value & input2_value,
            Operator::Or => input1_value | input2_value,
            Operator::Xor => input1_value ^ input2_value,
        };

        state.insert(self.output.clone(), value);

        true
    }
}

fn check_wire_is_input(wire: &str) -> bool {
    let first = wire.chars().next();

    matches!(first, Some('x' | 'y'))
}

fn check_wire_is_output(wire: &str) -> bool {
    let first = wire.chars().next();

    matches!(first, Some('z'))
}

fn parse_input(input: &str) -> (HashMap<String, bool>, Vec<Instruction>) {
    let mut state = HashMap::new();

    let mut instructions = vec![];

    let mut before_linebreak = true;

    for line in input.trim().lines() {
        if line.is_empty() {
            before_linebreak = false;
            continue;
        }

        if before_linebreak {
            // parse state
            let (key, value) = parse_state_line(line);
            state.insert(key, value);
        } else {
            // parse instruction
            instructions.push(parse_instruction_line(line));
        }
    }

    (state, instructions)
}

fn parse_instruction_line(line: &str) -> Instruction {
    let pieces: [String; 4] = line
        .split([' ', '-', '>'])
        .filter(|piece| !piece.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let [input1, operator, input2, output] = pieces;

    match operator.as_str() {
        "AND" => Instruction {
            wire1: input1,
            wire2: input2,
            output,
            operator: Operator::And,
        },
        "XOR" => Instruction {
            wire1: input1,
            wire2: input2,
            output,
            operator: Operator::Xor,
        },
        "OR" => Instruction {
            wire1: input1,
            wire2: input2,
            output,
            operator: Operator::Or,
        },
        _ => panic!(),
    }
}

fn parse_state_line(line: &str) -> (String, bool) {
    let (register, value) = line.split_once(": ").unwrap();

    (register.into(), value.parse::<u8>().unwrap() > 0)
}

fn execute_program(input: &str) -> PartSolution {
    let (mut state, instructions) = parse_input(input);

    let mut instructions = VecDeque::from(instructions);

    while let Some(instruction) = instructions.pop_front() {
        if !instruction.execute(&mut state) {
            instructions.push_back(instruction);
        }
    }

    for instruction in instructions {
        instruction.execute(&mut state);
    }

    let mut output = 0_u64;

    for i in 0..64_u32 {
        let z_key = format!("z{:0>2}", i);

        let Some(mut bit) = state.get(&z_key).map(|&b| u64::from(b)) else {
            break;
        };

        bit <<= i;
        output |= bit;
    }

    PartSolution::U64(output)
}

fn build_reverse_map(instructions: &[Instruction]) -> (usize, HashMap<String, Vec<Instruction>>) {
    let mut map = HashMap::<String, Vec<Instruction>>::new();

    let mut max_z = usize::MIN;

    for instruction in instructions {
        map.entry(instruction.wire1.clone())
            .and_modify(|set| {
                set.push(instruction.clone());
            })
            .or_insert_with(|| vec![instruction.clone()]);

        map.entry(instruction.wire2.clone())
            .and_modify(|set| {
                set.push(instruction.clone());
            })
            .or_insert_with(|| vec![instruction.clone()]);

        if let Some(without_prefix) = instruction
            .output
            .strip_prefix('z')
            .map(|number| number.parse::<usize>().unwrap())
        {
            max_z = max_z.max(without_prefix);
        }
    }

    (max_z, map)
}

fn flip_wires(input: &str) -> PartSolution {
    let (mut _state, instructions) = parse_input(input);

    let (max_z, mut map) = build_reverse_map(&instructions);

    for (_, instructions) in &mut map {
        #[expect(clippy::as_conversions, reason = "Operator is repr(usize)")]
        instructions.sort_by_key(|i| (i.operator) as usize);
    }

    let mut incorrect_outputs = HashSet::new();

    for instruction in &instructions {
        let wire_in_1 = &instruction.wire1;
        let wire_in_2 = &instruction.wire2;
        let wire_out = &instruction.output;

        let operator = instruction.operator;

        if let Some(without_prefix) = wire_out.strip_prefix('z') {
            let index = without_prefix.parse::<usize>().unwrap();

            if index == 0 || index == max_z {
                continue;
            }
        }

        let wire1_is_input = check_wire_is_input(wire_in_1);
        let wire2_is_input = check_wire_is_input(wire_in_2);

        let wire_out_is_output = check_wire_is_output(wire_out);

        // below is based on the fact that our adder is a ripple carry adder
        // https://en.wikipedia.org/wiki/Adder_(electronics)#Ripple-carry_adder

        // sanity
        assert!(!(wire1_is_input ^ wire2_is_input), "Bad input");

        // the first bit is a half adder
        if wire_in_1 == "x00" && wire_in_2 == "y00" {
            continue;
        }

        // checking the wire1 is enough, as we have no combinations where the inputs are wrong. Only the outputs are wrong
        match operator {
            Operator::Xor => {
                if wire1_is_input && wire_out_is_output {
                    incorrect_outputs.insert(wire_out.clone());
                }

                // check for the (in1, in2) -> (Xor, And) block
                if wire1_is_input
                    && map.get(wire_out).is_none_or(|next| {
                        // ensure we are followed by Xor and And
                        !instructions_match(next, &[Operator::And, Operator::Xor])
                    })
                {
                    incorrect_outputs.insert(wire_out.clone());
                }

                // check for the (from_previous_xor, in2) -> out block
                if !wire1_is_input && !wire_out_is_output {
                    incorrect_outputs.insert(wire_out.clone());
                }
            },
            Operator::And => {
                if map.get(wire_out).is_none_or(|next| {
                    // ensure we are followed by Or
                    !instructions_match(next, &[Operator::Or])
                }) {
                    incorrect_outputs.insert(wire_out.clone());
                }
            },
            Operator::Or => {
                if wire1_is_input || wire2_is_input {
                    // no Or block has inputs
                    incorrect_outputs.insert(wire_out.clone());
                }

                if map.get(wire_out).is_none_or(|next| {
                    // ensure we are followed by Xor and And
                    !instructions_match(next, &[Operator::And, Operator::Xor])
                }) {
                    incorrect_outputs.insert(wire_out.clone());
                }
            },
        }
    }

    let mut incorrect_outputs: Vec<String> = incorrect_outputs.into_iter().collect();
    incorrect_outputs.sort_unstable();

    PartSolution::String(incorrect_outputs.join(","))
}

fn instructions_match(instructions: &[Instruction], expected: &[Operator]) -> bool {
    if instructions.len() != expected.len() {
        return false;
    }

    for (index, operator) in instructions
        .iter()
        .map(|instruction| instruction.operator)
        .enumerate()
    {
        if operator != expected[index] {
            return false;
        }
    }

    true
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        execute_program(input)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        flip_wires(input)
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2024::shared::Parts as _;
        use advent_of_code_2024::shared::solution::{read_file, read_file_part};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                55_544_677_167_336_u64,
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example_1() {
            assert_eq!(
                4,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 1))
            );
        }

        #[test]
        fn example_2() {
            assert_eq!(
                2024,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 2))
            );
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::solution::{read_file, read_file_part};
        use advent_of_code_2024::shared::{PartSolution, Parts as _};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::String("gsd,kth,qnf,tbt,vpm,z12,z26,z32".into()),
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example_3() {
            assert_eq!(
                PartSolution::String("z01,z02,z03,z04".into()),
                (Solution {}).part_2(&read_file_part("examples", &DAY, 3))
            );
        }
    }
}
