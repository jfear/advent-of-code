use std::{cmp::Ordering, collections::HashSet};

use itertools::all;
use miette::Context;
use winnow::{
    ascii::{digit1, line_ending},
    combinator::{repeat, separated, separated_pair, terminated},
    PResult, Parser,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_report_order, updated_reports) = parse(input).context("should parse input")?;
    let mut total = 0;
    for report in updated_reports {
        let mut in_order = Vec::new();
        for (i, page) in report.iter().enumerate() {
            let j = i + 1;
            for later_page in report[j..].iter() {
                if _report_order.contains(&(*page, *later_page)) {
                    in_order.push(true);
                } else {
                    in_order.push(false);
                }
            }
        }
        if !all(in_order, |x| x) & (report.len() % 2 != 0) {
            let mut sorted_report = report.clone();
            sorted_report.sort_by(|a, b| {
                if _report_order.contains(&(*a, *b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            total += sorted_report[sorted_report.len() / 2];
        }
    }

    Ok(total.to_string())
}

fn parse_page_order(input: &mut &str) -> PResult<Vec<(u32, u32)>> {
    repeat(
        1..,
        terminated(
            separated_pair(digit1.parse_to(), '|', digit1.parse_to()),
            line_ending,
        ),
    )
    .parse_next(input)
}

fn parse_report_update(input: &mut &str) -> PResult<Vec<u32>> {
    separated(1.., digit1.parse_to::<u32>(), ',').parse_next(input)
}

type PageOrder = HashSet<(u32, u32)>;
type UpdatedReport = Vec<Vec<u32>>;

fn parse(input: &str) -> miette::Result<(PageOrder, UpdatedReport)> {
    let mut input = input;
    let page_order: PageOrder =
        HashSet::from_iter(parse_page_order.parse_next(&mut input).unwrap());

    let _: PResult<Vec<_>> = repeat(0.., line_ending).parse_next(&mut input);

    let reports: Vec<Vec<u32>> = repeat(0.., terminated(parse_report_update, line_ending))
        .parse_next(&mut input)
        .unwrap();

    Ok((page_order, reports))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("123", process(input)?);
        Ok(())
    }
}
