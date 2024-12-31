use miette::miette;
use pathfinding::prelude::dijkstra;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;

use glam::IVec2;

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

#[tracing::instrument]
pub fn process(input: &str, savings: i32) -> miette::Result<String> {
    let (start, end, walls) = parse(input).map_err(|e| miette!("should of parsed {}", e))?;

    let max_x = walls
        .iter()
        .map(|w| w.x)
        .max()
        .expect("walls suround perimeter");

    let max_y = walls
        .iter()
        .map(|w| w.y)
        .max()
        .expect("walls surround perimeter");

    let base_time = find_shortest_path(&start, &end, &walls, max_x, max_y, None);

    let cheats = walls
        .iter()
        .filter_map(|w| {
            let north = w + IVec2::NEG_Y;
            let east = w + IVec2::X;
            let south = w + IVec2::Y;
            let west = w + IVec2::NEG_X;

            // find walls were both sides are open.
            if !(walls.contains(&north) || walls.contains(&south))
                || !(walls.contains(&east) || walls.contains(&west))
            {
                Some(w)
            } else {
                None
            }
        })
        .collect::<Vec<&IVec2>>();

    let num = cheats
        .par_iter()
        .map(|w| find_shortest_path(&start, &end, &walls, max_x, max_y, Some(w)))
        .filter_map(|p| {
            let diff = base_time - p;
            if diff >= savings {
                Some(diff)
            } else {
                None
            }
        })
        .count();

    Ok(num.to_string())
}

fn find_shortest_path(
    start: &IVec2,
    end: &IVec2,
    walls: &Vec<IVec2>,
    max_x: i32,
    max_y: i32,
    cheat: Option<&IVec2>,
) -> i32 {
    dijkstra(
        start,
        |p| {
            DIRECTIONS
                .iter()
                .filter_map(|&d| {
                    let next_pos = p + d;

                    if cheat.clone().is_some_and(|c| c == &next_pos) {
                        Some((next_pos, 1))
                    } else if next_pos.x < 0
                        || next_pos.x > max_x
                        || next_pos.y < 0
                        || next_pos.y > max_y
                        || walls.contains(&next_pos)
                    {
                        None
                    } else {
                        Some((next_pos, 1))
                    }
                })
                .collect::<Vec<(IVec2, i32)>>()
        },
        |p| p == end,
    )
    .expect("should be at least one path")
    .1
}

fn parse(input: &str) -> miette::Result<(IVec2, IVec2, Vec<IVec2>)> {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| (IVec2::new(x as i32, y as i32), c))
                .collect::<HashMap<IVec2, char>>()
        })
        .collect::<HashMap<IVec2, char>>();

    let start = map
        .iter()
        .filter_map(|(&k, &v)| match v {
            'S' => Some(k),
            _ => None,
        })
        .collect::<Vec<IVec2>>()[0];

    let end = map
        .iter()
        .filter_map(|(&k, &v)| match v {
            'E' => Some(k),
            _ => None,
        })
        .collect::<Vec<IVec2>>()[0];

    let walls = map
        .iter()
        .filter_map(|(&k, &v)| match v {
            '#' => Some(k),
            _ => None,
        })
        .collect::<Vec<IVec2>>();

    Ok((start, end, walls))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!("5", process(input, 20)?);
        Ok(())
    }
}
