use std::cmp::Ordering;
use std::collections::HashSet;

use itertools::Itertools;
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let edges = parse(input);
    let nodes: HashSet<&str> = edges
        .clone()
        .into_iter()
        .flat_map(|(lhs, rhs)| [lhs, rhs])
        .collect();

    let games = nodes
        .into_iter()
        .sorted()
        .combinations(3)
        .filter(|network| network.iter().any(|c| c.starts_with('t')))
        .filter(|network| {
            network
                .iter()
                .combinations(2)
                .all(|n| edges.contains(&(*n[0], *n[1])))
        })
        .count();

    Ok(games.to_string())
}

fn parse<'a>(input: &'a str) -> HashSet<(&'a str, &'a str)> {
    input
        .lines()
        .filter_map(|l| {
            let mut split = l.split("-");
            match (split.next(), split.next()) {
                (Some(lhs), Some(rhs)) => Some((lhs, rhs)),
                _ => None,
            }
        })
        .filter_map(|(lhs, rhs)| match lhs.cmp(rhs) {
            Ordering::Less => Some((lhs, rhs)),
            Ordering::Greater => Some((rhs, lhs)),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        assert_eq!("7", process(input)?);
        Ok(())
    }
}
