use std::{collections::HashMap, iter::successors};

use glam::IVec2;
use itertools::Itertools;

//                                      North         East      South     West
pub const DIRECTIONS: [IVec2; 4] = [IVec2::NEG_Y, IVec2::X, IVec2::Y, IVec2::NEG_X];

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let garden = parse(input);
    let mut visited_plots = vec![];
    let mut total = 0;

    for (plot, crop) in garden.iter() {
        if visited_plots.contains(&plot) {
            continue;
        } else {
            visited_plots.push(plot);
        }

        let curr_plots = successors(Some(vec![plot]), |p| {
            let next_vec = p
                .iter()
                .cartesian_product(DIRECTIONS)
                .filter_map(|(&p, dir)| match garden.get_key_value(&(p + dir)) {
                    Some((next_plot, next_crop))
                        if !visited_plots.contains(&next_plot) && next_crop == crop =>
                    {
                        visited_plots.push(next_plot);
                        Some(next_plot)
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();

            if next_vec.is_empty() {
                None
            } else {
                Some(next_vec)
            }
        })
        .flatten()
        .collect::<Vec<&IVec2>>();

        let area = curr_plots.len();
        let corners = DIRECTIONS
            .iter()
            .circular_tuple_windows()
            .take(4)
            .cartesian_product(&curr_plots)
            .filter_map(|((dir1, dir2), &curr_plot)| {
                let d1 = dir1 + curr_plot;
                let diag = dir1 + dir2 + curr_plot;
                let d2 = dir2 + curr_plot;
                match (
                    curr_plots.contains(&&d1),
                    curr_plots.contains(&&diag),
                    curr_plots.contains(&&d2),
                ) {
                    (false, false, false) => Some(curr_plot),
                    (true, false, true) => Some(curr_plot),
                    (false, true, false) => Some(curr_plot),
                    _ => None,
                }
            })
            .count();

        total += area * corners;
    }

    Ok(total.to_string())
}

fn parse(input: &str) -> HashMap<IVec2, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| (IVec2::new(x as i32, y as i32), c))
                .collect_vec()
        })
        // .filter(|(_, crop)| *crop == 'E')
        .collect::<HashMap<IVec2, char>>()
}

// The larger example from before now has the following updated prices:
// A region of R plants with price 12 * 10 = 120.
// A region of I plants with price 4 * 4 = 16.
// A region of C plants with price 14 * 22 = 308.
// A region of F plants with price 10 * 12 = 120.
// A region of V plants with price 13 * 10 = 130.
// A region of J plants with price 11 * 12 = 132.
// A region of C plants with price 1 * 4 = 4.
// A region of E plants with price 13 * 8 = 104.
// A region of I plants with price 14 * 16 = 224.
// A region of M plants with price 5 * 6 = 30.
// A region of S plants with price 3 * 6 = 18.
//
// Test Case:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1206", process(input)?);
        Ok(())
    }

    #[test]
    fn test_tiny1() -> miette::Result<()> {
        let input = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!("80", process(input)?);
        Ok(())
    }

    #[test]
    fn test_tiny2() -> miette::Result<()> {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!("236", process(input)?);
        Ok(())
    }

    #[test]
    fn test_tiny3() -> miette::Result<()> {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!("368", process(input)?);
        Ok(())
    }
}
