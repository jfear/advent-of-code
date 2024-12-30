use miette::{miette, Context};
use std::collections::HashSet;

use winnow::{
    ascii::{alpha1, line_ending},
    combinator::{separated, seq, terminated},
    Parser,
};

#[tracing::instrument]
pub fn process(input: &mut &str) -> miette::Result<String> {
    let (towels, display) =
        parse(input).map_err(|e| miette!("AoC should provide valid input {}", e))?;

    let max_window = towels
        .iter()
        .map(|t| t.len())
        .max()
        .context("should be able to get length of at least 1 towel")?;

    let num_valid = display
        .into_iter()
        .filter(|&d| check_display(d, &towels))
        .count();
    Ok(num_valid.to_string())
}

fn check_display(display: &str, towels: &Vec<&str>) -> bool {
    towels
        .iter()
        .map(|towel| {
            if display.starts_with(*towel) {
                let new_display = &display[towel.len()..];
                if new_display.is_empty() {
                    return true;
                }
                check_display(new_display, towels)
            } else {
                false
            }
        })
        .any(|v| v)
}

fn parse<'a>(input: &mut &'a str) -> winnow::PResult<(Vec<&'a str>, Vec<&'a str>)> {
    seq!(
        parse_towels,
        _: line_ending,
        parse_display
    )
    .parse_next(input)
}

fn parse_towels<'a>(input: &mut &'a str) -> winnow::PResult<Vec<&'a str>> {
    terminated(
        separated::<_, _, Vec<&'a str>, _, _, _, _>(1.., alpha1, ", "),
        line_ending,
    )
    .parse_next(input)
}

fn parse_display<'a>(input: &mut &'a str) -> winnow::PResult<Vec<&'a str>> {
    separated(1.., alpha1, line_ending).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        // ubwu & bbrgwb are not possible
        let mut input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!("6", process(&mut input)?);
        Ok(())
    }

    #[test]
    fn test_slice() {
        let bob = "abc";
        assert_eq!(&bob[0..0 + 1], "a");
    }
}
