use std::collections::HashSet;

use glam::IVec2;
use pathfinding::prelude::dijkstra;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (start, end, walls) = parse(input);

    let (_path, score) = dijkstra(
        &(start, IVec2::X),
        |(pos, dir)| {
            let next_pos = pos + dir;
            if walls.contains(&next_pos) {
                vec![((*pos, dir.perp()), 1000), ((*pos, -dir.perp()), 1000)]
            } else {
                vec![
                    ((next_pos, *dir), 1),
                    ((*pos, dir.perp()), 1000),
                    ((*pos, -dir.perp()), 1000),
                ]
            }
        },
        |&(pos, _)| pos == end,
    )
    .expect("AoC should have valid result.");

    Ok(score.to_string())
}

// The Reindeer start on the Start Tile (marked S) facing East and need to reach the End Tile
// (marked E). They can move forward one tile at a time (increasing their score by 1 point),
// but never into a wall (#). They can also rotate clockwise or counterclockwise 90 degrees
// at a time (increasing their score by 1000 points).
//
// What is the lowest score a Reindeer could possibly get?

fn parse(input: &str) -> (IVec2, IVec2, HashSet<IVec2>) {
    let mut start: IVec2 = IVec2::default();
    let mut end: IVec2 = IVec2::default();
    let mut walls: HashSet<IVec2> = HashSet::new();

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                'S' => {
                    start = IVec2::new(x as i32, y as i32);
                }
                'E' => {
                    end = IVec2::new(x as i32, y as i32);
                }
                '#' => {
                    walls.insert(IVec2::new(x as i32, y as i32));
                }
                _ => {}
            }
        }
    }

    (start, end, walls)
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
