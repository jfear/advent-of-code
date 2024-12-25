use std::ops::{BitXor, BitXorAssign};

use miette::miette;
use winnow::{
    ascii::{dec_int, line_ending},
    combinator::{preceded, repeat, separated, seq, terminated},
    Parser,
};

#[tracing::instrument]
pub fn process(input: &mut &str) -> miette::Result<String> {
    let mut computer = parse(input);
    let mut idx = 0;
    let mut output: Vec<i32> = vec![];

    loop {
        if idx >= computer.program.len() {
            break;
        }

        let opcode = computer.program.get(idx).expect("already tested bounts");
        let operand = computer
            .program
            .get(idx + 1)
            .expect("already tested bounds");

        match (opcode, operand) {
            (0, 0..=3) => {
                // The adv instruction (opcode 0) performs division.
                // The numerator is the value in the A register.
                // The denominator is found by raising 2 to the power of the instruction's combo operand.
                // (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
                // The result of the division operation is truncated to an integer and then written to the A register.
                computer.register_a = computer.register_a / 2i32.pow(*operand as u32);
                idx += 2;
            }
            (0, 4) => {
                computer.register_a = computer.register_a / 2i32.pow(computer.register_a as u32);
                idx += 2;
            }
            (0, 5) => {
                computer.register_a = computer.register_a / 2i32.pow(computer.register_b as u32);
                idx += 2;
            }
            (0, 6) => {
                computer.register_a = computer.register_a / 2i32.pow(computer.register_c as u32);
                idx += 2;
            }
            (1, 0..=7) => {
                // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's
                // literal operand, then stores the result in register B.
                computer.register_b ^= operand;
                idx += 2;
            }
            (2, 0..=3) => {
                // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
                // (thereby keeping only its lowest 3 bits), then writes that value to the B register.
                computer.register_b = operand % 8;
                idx += 2;
            }
            (2, 4) => {
                computer.register_b = computer.register_a % 8;
                idx += 2;
            }
            (2, 5) => {
                computer.register_b = computer.register_b % 8;
                idx += 2;
            }
            (2, 6) => {
                computer.register_b = computer.register_c % 8;
                idx += 2;
            }
            (3, 0..=6) => {
                // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the
                // A register is not zero, it jumps by setting the instruction pointer to the value of
                // its literal operand; if this instruction jumps, the instruction pointer is not
                // increased by 2 after this instruction.
                if computer.register_a != 0 {
                    idx = *operand as usize;
                } else {
                    idx += 2;
                }
            }
            (4, 0..=6) => {
                // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
                // then stores the result in register B. (For legacy reasons, this instruction reads an
                // operand but ignores it.)
                computer.register_b ^= computer.register_c;
                idx += 2;
            }
            (5, 0..=3) => {
                // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs
                // that value. (If a program outputs multiple values, they are separated by commas.)
                output.push(operand % 8);
                idx += 2;
            }
            (5, 4) => {
                output.push(computer.register_a % 8);
                idx += 2;
            }
            (5, 5) => {
                output.push(computer.register_b % 8);
                idx += 2;
            }
            (5, 6) => {
                output.push(computer.register_c % 8);
                idx += 2;
            }
            (6, 0..=3) => {
                // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result
                // is stored in the B register. (The numerator is still read from the A register.)
                computer.register_b = computer.register_a / 2i32.pow(*operand as u32);
                idx += 2;
            }
            (6, 4) => {
                computer.register_b = computer.register_a / 2i32.pow(computer.register_a as u32);
                idx += 2;
            }
            (6, 5) => {
                computer.register_b = computer.register_a / 2i32.pow(computer.register_b as u32);
                idx += 2;
            }
            (6, 6) => {
                computer.register_b = computer.register_a / 2i32.pow(computer.register_c as u32);
                idx += 2;
            }
            (7, 0..=3) => {
                // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result
                // is stored in the C register. (The numerator is still read from the A register.)
                computer.register_c = computer.register_a / 2i32.pow(*operand as u32);
                idx += 2;
            }
            (7, 4) => {
                computer.register_c = computer.register_a / 2i32.pow(computer.register_a as u32);
                idx += 2;
            }
            (7, 5) => {
                computer.register_c = computer.register_a / 2i32.pow(computer.register_b as u32);
                idx += 2;
            }
            (7, 6) => {
                computer.register_c = computer.register_a / 2i32.pow(computer.register_c as u32);
                idx += 2;
            }
            (_, 7) => panic!("This is reserved and should never happen."),
            (_, _) => panic!("Don't know what is going on {}, {}", opcode, operand),
        }
    }
    Ok(output
        .iter()
        .map(|&v| format!("{v}"))
        .collect::<Vec<String>>()
        .join(","))
}

#[allow(dead_code)]
#[derive(Debug)]
struct Computer {
    register_a: i32,
    register_b: i32,
    register_c: i32,
    program: Vec<i32>,
}

fn parse(input: &mut &str) -> Computer {
    seq!(Computer {
        register_a: parse_register_a,
        register_b: parse_register_b,
        register_c: parse_register_c,
        _: line_ending,
        program: parse_program,
    })
    .parse_next(input)
    .expect("AoC should provide valid input.")
}

fn parse_register_a(input: &mut &str) -> winnow::PResult<i32> {
    terminated(preceded("Register A: ", dec_int::<_, i32, _>), line_ending).parse_next(input)
}

fn parse_register_b(input: &mut &str) -> winnow::PResult<i32> {
    terminated(preceded("Register B: ", dec_int::<_, i32, _>), line_ending).parse_next(input)
}

fn parse_register_c(input: &mut &str) -> winnow::PResult<i32> {
    terminated(preceded("Register C: ", dec_int::<_, i32, _>), line_ending).parse_next(input)
}

fn parse_program(input: &mut &str) -> winnow::PResult<Vec<i32>> {
    preceded("Program: ", separated(1.., dec_int::<_, i32, _>, ",")).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let mut input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,6,3,5,6,3,5,2,1,0", process(&mut input)?);
        Ok(())
    }

    #[test]
    fn test_reg_a() -> miette::Result<()> {
        let mut input = "Register A: 729\n";
        assert_eq!(
            729,
            parse_register_a(&mut input)
                .map_err(|e| miette!("could not parse register A {}", e))?
        );
        Ok(())
    }

    #[test]
    fn test_reg_b() -> miette::Result<()> {
        let mut input = "Register B: 0\n";
        assert_eq!(
            0,
            parse_register_b(&mut input)
                .map_err(|e| miette!("could not parse register B {}", e))?
        );
        Ok(())
    }

    #[test]
    fn test_reg_c() -> miette::Result<()> {
        let mut input = "Register C: 0\n";
        assert_eq!(
            0,
            parse_register_c(&mut input)
                .map_err(|e| miette!("could not parse register C {}", e))?
        );
        Ok(())
    }

    #[test]
    fn test_prog() -> miette::Result<()> {
        let mut input = "Program: 0,1,5,4,3,0";
        assert_eq!(
            vec![0, 1, 5, 4, 3, 0],
            parse_program(&mut input).map_err(|e| miette!("could not parse program {}", e))?
        );
        Ok(())
    }
}
