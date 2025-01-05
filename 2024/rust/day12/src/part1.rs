use std::{collections::HashMap, iter::successors};

use glam::IVec2;
use itertools::Itertools;

//                                      North         East      South     West
pub const DIRECTIONS: [IVec2; 4] = [IVec2::NEG_Y, IVec2::X, IVec2::Y, IVec2::NEG_X];

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let garden = parse(input);
    let mut visited_plots = Vec::new();
    let mut total_price = 0;

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
        let edges = DIRECTIONS
            .iter()
            .cartesian_product(&curr_plots)
            .filter(|&(dir1, &curr_plot)| {
                let d1 = dir1 + curr_plot;
                !curr_plots.contains(&&d1)
            })
            .count();

        total_price += area * edges;
    }

    Ok(total_price.to_string())
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

// Test Case:
//   - A region of R plants with price 12 * 18 = 216.
//   - A region of I plants with price 4 * 8 = 32.
//   - A region of C plants with price 14 * 28 = 392.
//   - A region of F plants with price 10 * 18 = 180.
//   - A region of V plants with price 13 * 20 = 260.
//   - A region of J plants with price 11 * 20 = 220.
//   - A region of C plants with price 1 * 4 = 4.
//   - A region of E plants with price 13 * 18 = 234.
//   - A region of I plants with price 14 * 22 = 308.
//   - A region of M plants with price 5 * 12 = 60.
//   - A region of S plants with price 3 * 8 = 24.
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
        assert_eq!("1930", process(input)?);
        Ok(())
    }
}
