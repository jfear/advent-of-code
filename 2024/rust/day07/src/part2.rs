use itertools::{repeat_n, Itertools};
use winnow::{
    ascii::{dec_int, line_ending, space1},
    combinator::{separated, separated_pair},
    Parser,
};

#[tracing::instrument]
pub fn process(input: &mut &str) -> miette::Result<String> {
    let result = parse(input)
        .unwrap()
        .into_iter()
        .filter(|(r, vs)| {
            let totals = repeat_n(
                [Instruction::Add, Instruction::Mul, Instruction::Cat],
                vs.len() - 1,
            )
            .multi_cartesian_product()
            .map(|ops| {
                ops.iter()
                    .enumerate()
                    .fold(vs[0] as i64, |acc, (i, op)| match op {
                        Instruction::Add => acc + vs[i + 1] as i64,
                        Instruction::Mul => acc * vs[i + 1] as i64,
                        Instruction::Cat => {
                            let cat_num = format!("{}{}", acc, vs[i + 1]);
                            cat_num.parse::<i64>().unwrap()
                        }
                    })
            })
            .collect::<Vec<i64>>();
            totals.contains(&(*r as i64))
        })
        .map(|(r, _)| r)
        .sum::<i64>();
    Ok(result.to_string())
}

#[derive(Debug, Clone)]
enum Instruction {
    Add,
    Mul,
    Cat,
}

fn parse_callibrations(input: &mut &str) -> winnow::PResult<(i64, Vec<i64>)> {
    separated_pair(dec_int, ": ", separated(1.., dec_int::<_, i64, _>, space1)).parse_next(input)
}

fn parse(input: &mut &str) -> winnow::PResult<Vec<(i64, Vec<i64>)>> {
    separated(1.., parse_callibrations, line_ending).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let mut input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("11387", process(&mut input)?);
        Ok(())
    }
}
