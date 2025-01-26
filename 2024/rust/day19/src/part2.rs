use cached::proc_macro::cached;
use miette::miette;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use winnow::{
    ascii::{alpha1, line_ending},
    combinator::{separated, seq, terminated},
    Parser,
};

#[tracing::instrument]
pub fn process(input: &mut &str) -> miette::Result<String> {
    let (towels, displays) =
        parse(input).map_err(|e| miette!("AoC should provide valid input {}", e))?;

    let num_valid: usize = displays
        .par_iter()
        .map(|&d| {
            println!("{d}");
            find_ways(d, &towels)
        })
        .sum();
    Ok(num_valid.to_string())
}

#[cached(key = "String", convert = r##"{ format!("{display}") }"##)]
fn find_ways(display: &str, towels: &[&str]) -> usize {
    towels
        .iter()
        .filter_map(|&towel| {
            if display.starts_with(towel) {
                let new_display = &display[towel.len()..];
                if new_display.is_empty() {
                    return Some(1);
                }
                Some(find_ways(new_display, towels))
            } else {
                None
            }
        })
        .sum()
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
    use rstest::rstest;

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
        assert_eq!("16", process(&mut input)?);
        Ok(())
    }

    #[rstest]
    #[case("brwrr", 2usize)]
    #[case("bggr", 1usize)]
    #[case("gbbr", 4usize)]
    #[case("rrbgbr", 6usize)]
    #[case("bwurrg", 1usize)]
    #[case("brgr", 2usize)]
    fn test_count_ways(#[case] display: &str, #[case] ways: usize) {
        let towels = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
        let n = find_ways(display, &towels);
        assert_eq!(ways, n)
    }
}
