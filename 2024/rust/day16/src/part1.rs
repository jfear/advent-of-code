use std::{cmp, collections::HashMap};

use glam::IVec2;
use itertools::Itertools;

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::Y, IVec2::NEG_Y, IVec2::NEG_X];

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let map = parse(input);
    let start = map
        .iter()
        .find_map(|(l, c)| match c {
            'S' => Some(l),
            _ => None,
        })
        .expect("There should always be a start location.");

    let mut best_score = u32::MAX;
    let mut curr_paths = vec![Path::default(vec![start])];

    loop {
        if curr_paths.is_empty() {
            break;
        }

        curr_paths = DIRECTIONS
            .iter()
            .cartesian_product(curr_paths)
            .filter_map(|(dir, mut path)| {
                if *dir == path.dir * IVec2::NEG_ONE {
                    return None;
                }

                let score = if dir == path.dir { 1 } else { 1001 };
                let next_loc = path.locs.iter().last().copied().unwrap() + dir;

                if path.score + score >= best_score {
                    return None;
                }

                if path.locs.contains(&&next_loc) {
                    return None;
                }

                match map.get_key_value(&next_loc) {
                    Some((k, '.')) => {
                        path.locs.push(k);
                        path.score += score;
                        path.dir = dir;
                        Some(path)
                    }
                    Some((_, 'E')) => {
                        best_score = cmp::min(best_score, path.score + score);
                        None
                    }
                    Some((_, '#')) => None,
                    Some((_, 'S')) => None,
                    Some((_, _)) => {
                        panic!("this should not happen")
                    }
                    None => None,
                }
            })
            .collect();
    }

    Ok(best_score.to_string())
}

#[derive(Debug, Clone)]
struct Path<'a> {
    locs: Vec<&'a IVec2>,
    dir: &'a IVec2,
    score: u32,
}

impl<'a> Path<'a> {
    fn new(locs: Vec<&'a IVec2>, dir: &'a IVec2, score: u32) -> Self {
        Self { locs, dir, score }
    }

    fn default(locs: Vec<&'a IVec2>) -> Self {
        Self::new(locs, &IVec2::X, 0)
    }
}

// The Reindeer start on the Start Tile (marked S) facing East and need to reach the End Tile
// (marked E). They can move forward one tile at a time (increasing their score by 1 point),
// but never into a wall (#). They can also rotate clockwise or counterclockwise 90 degrees
// at a time (increasing their score by 1000 points).
//
// What is the lowest score a Reindeer could possibly get?

fn parse(input: &str) -> HashMap<IVec2, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| (IVec2::new(x as i32, y as i32), c))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_maze() -> miette::Result<()> {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!("7036", process(input)?);
        Ok(())
    }

    #[test]
    fn test_large_maze() -> miette::Result<()> {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!("11048", process(input)?);
        Ok(())
    }
}
