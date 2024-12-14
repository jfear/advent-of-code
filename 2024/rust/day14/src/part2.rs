use std::collections::HashMap;

use glam::IVec2;
use itertools::Itertools;
use miette::miette;
use winnow::{
    ascii::{dec_int, line_ending, space1},
    combinator::{preceded, separated, separated_pair},
    prelude::*,
};

const MAP_SIZE: IVec2 = IVec2::new(101, 103);

#[tracing::instrument]
pub fn process(input: &mut &str) -> miette::Result<String> {
    let robots = parse(input).map_err(|e| miette!("could not parse {}", e))?;
    let mut plots = Vec::new();
    for i in 1..10000 {
        let field = robots.iter().fold(HashMap::new(), |mut acc, r| {
            let loc = (r.position + r.velocity * i).rem_euclid(MAP_SIZE);
            acc.entry(loc).and_modify(|v| *v += 1).or_insert(1);
            acc
        });
        if tree_test(&field) {
            plots.push(format!("Round {}\n{}\n\n", i, print_field(field)));
        }
    }
    Ok(plots.into_iter().collect::<String>())
}

fn tree_test(field: &HashMap<IVec2, i32>) -> bool {
    field.values().all(|v| *v == 1)
}
fn print_field(field: HashMap<IVec2, i32>) -> String {
    (0..MAP_SIZE.x)
        .map(|x| {
            (0..MAP_SIZE.y)
                .map(|y| match field.get(&IVec2::new(x, y)) {
                    None => ".".to_string(),
                    Some(cnt) => format!("{cnt}"),
                })
                .collect::<String>()
        })
        .join("\n")
}

#[derive(Debug)]
struct Robot {
    position: IVec2,
    velocity: IVec2,
}

fn parse(input: &mut &str) -> PResult<Vec<Robot>> {
    separated(
        1..,
        separated_pair(
            preceded("p=", separated_pair(dec_int, ",", dec_int)).map(|(x, y)| IVec2::new(x, y)),
            space1,
            preceded("v=", separated_pair(dec_int, ",", dec_int)).map(|(x, y)| IVec2::new(x, y)),
        )
        .map(|(p, v)| Robot {
            position: p,
            velocity: v,
        }),
        line_ending,
    )
    .parse_next(input)
}
