use std::collections::{HashMap, HashSet};

use glam::IVec2;
use itertools::Itertools;

const DIRECTION: [IVec2; 4] = [IVec2::NEG_X, IVec2::X, IVec2::NEG_Y, IVec2::Y];

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let topo = parse(input);
    let total: usize = topo
        .iter()
        .filter(|(_, v)| v == &&0)
        .map(|(k, _)| walk(k, &topo))
        .sum();

    Ok(total.to_string())
}

fn walk(loc: &IVec2, topo: &HashMap<IVec2, u32>) -> usize {
    let mut visited_locations: HashSet<IVec2> = HashSet::new();
    let mut new_locations = HashSet::from([*loc]);
    loop {
        if new_locations.is_empty() {
            break;
        }

        let newer_locations = new_locations
            .iter()
            .flat_map(|loc| {
                DIRECTION
                    .iter()
                    .zip(std::iter::repeat(loc))
                    .filter_map(|(dir, loc)| {
                        let curr_elev = topo.get(&loc).unwrap();
                        let new_elev = topo.get(&(loc + dir)).unwrap_or(&0);
                        if new_elev == &(curr_elev + 1) {
                            Some(loc + dir)
                        } else {
                            None
                        }
                    })
            })
            .collect::<HashSet<IVec2>>();

        visited_locations = visited_locations
            .union(&newer_locations)
            .cloned()
            .collect::<HashSet<IVec2>>();

        new_locations = newer_locations;
    }

    visited_locations
        .iter()
        .filter(|l| topo.get(&l).unwrap() == &9u32)
        .count()
}

fn parse(input: &str) -> HashMap<IVec2, u32> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, v)| (IVec2::new(x as i32, y as i32), v.to_digit(10).unwrap()))
                .collect::<HashMap<IVec2, u32>>()
        })
        .collect::<HashMap<IVec2, u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("36", process(input)?);
        Ok(())
    }
}
