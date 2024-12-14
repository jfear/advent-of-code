use std::collections::HashMap;

use glam::IVec2;
use itertools::Itertools;

pub const DIRECTIONS: [IVec2; 4] = [IVec2::NEG_X, IVec2::X, IVec2::NEG_Y, IVec2::Y];

// multiple garden plots growing the same type of plant and touch (horizontally or vertically) form a region.
// area of a region is the number of garden plots the region contains.
// The perimeter is the number of sides of garden plots in the region.
//   - A lone plot has a perimiter of 4
// Price for a region = area * perimeter
// Answer is sum of price over region
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let garden = parse(input);
    let mut visited_plots = Vec::new();
    let mut total_price = 0;
    for (plot, crop) in garden.iter() {
        if visited_plots.contains(&plot) {
            continue;
        }
        visited_plots.push(plot);
        let mut checked_plots = vec![plot];
        let mut curr_plots = vec![plot];
        let mut perimeter = 0;
        let mut area = 1;

        loop {
            curr_plots = DIRECTIONS
                .iter()
                .cartesian_product(&curr_plots)
                .filter_map(|(dir, &loc)| match garden.get_key_value(&(loc + dir)) {
                    Some((next_plot, next_crop)) => {
                        if checked_plots.contains(&next_plot) {
                            None
                        } else if next_crop == crop {
                            Some(next_plot)
                        } else {
                            perimeter += 1;
                            None
                        }
                    }
                    None => {
                        perimeter += 1;
                        None
                    }
                })
                .unique()
                .collect_vec();

            area += curr_plots.len();

            if curr_plots.is_empty() {
                total_price += area * perimeter;
                break;
            }

            checked_plots.extend(&curr_plots);
            visited_plots.extend(&curr_plots);
        }
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
