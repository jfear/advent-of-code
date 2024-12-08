use std::collections::HashSet;

use glam::IVec2;
use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &mut &str) -> miette::Result<String> {
    let (n_rows, n_cols, results) = parse(input);

    let total = results
        .into_iter()
        .into_group_map()
        .into_iter()
        .fold(
            &mut HashSet::new(),
            |acc: &mut HashSet<IVec2>, (_key, val): (_, Vec<IVec2>)| {
                for v in val.into_iter().permutations(2) {
                    let a = v[0];
                    let b = v[1];
                    let delta = a - b;

                    acc.insert(a);
                    acc.insert(b);

                    let mut antinode1 = a;
                    loop {
                        antinode1 += delta;
                        if (antinode1.x >= 0)
                            && (antinode1.x < n_cols)
                            && (antinode1.y >= 0)
                            && (antinode1.y < n_rows)
                        {
                            acc.insert(antinode1);
                        } else {
                            break;
                        }
                    }

                    let mut antinode2 = b;
                    loop {
                        antinode2 -= delta;
                        if (antinode2.x >= 0)
                            && (antinode2.x < n_cols)
                            && (antinode2.y >= 0)
                            && (antinode2.y < n_rows)
                        {
                            acc.insert(antinode2);
                        } else {
                            break;
                        }
                    }
                }
                acc
            },
        )
        .len();

    Ok(total.to_string())
}

fn parse(input: &mut &str) -> (i32, i32, Vec<(char, IVec2)>) {
    let n_rows = input.lines().count() as i32;
    let n_cols = input.lines().next().unwrap().len() as i32;

    let data = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(col, val)| match val {
                    '.' => None,
                    _ => Some((val, IVec2::new(col as i32, row as i32))),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    (n_rows, n_cols, data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let mut input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("34", process(&mut input)?);
        Ok(())
    }
}
