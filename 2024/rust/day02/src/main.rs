use miette::miette;
use std::io::{self, Write};

fn main() -> miette::Result<()> {
    let input = include_str!("../../../input_day02.txt");

    writeln!(io::stdout(), "Part1: {}", part1(&input)?)
        .map_err(|e| miette!("failed to write {}", e))?;

    writeln!(io::stdout(), "Part2: {}", part2(&input)?)
        .map_err(|e| miette!("failed to write {}", e))?;

    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Ascending,
    Decending,
}

fn part1(input: &str) -> miette::Result<String> {
    let mut safe_cnt = 0;
    for line in input.lines() {
        let levels: Vec<Option<i32>> = line
            .split_whitespace()
            .map(|l| l.parse::<i32>().ok())
            .collect();
        safe_cnt += if is_safe(levels) { 1 } else { 0 };
    }
    return Ok(safe_cnt.to_string());
}

fn part2(input: &str) -> miette::Result<String> {
    let mut safe_cnt = 0;
    for line in input.lines() {
        let levels: Vec<Option<i32>> = line
            .split_whitespace()
            .map(|l| l.parse::<i32>().ok())
            .collect();
        if is_safe(levels.clone()) {
            safe_cnt += 1;
        } else {
            for idx in 0..levels.len() {
                let mut levels2 = levels.clone();
                levels2.remove(idx);
                if is_safe(levels2) {
                    safe_cnt += 1;
                    break;
                }
            }
        }
    }
    return Ok(safe_cnt.to_string());
}

fn is_safe(levels: Vec<Option<i32>>) -> bool {
    let mut prev_val: Option<i32> = None;
    let mut prev_dir: Option<Direction> = None;
    for curr_val in levels {
        match (prev_val, prev_dir, curr_val) {
            (None, None, Some(c)) => prev_val = Some(c),
            (Some(p), None, Some(c)) => {
                if p == c {
                    return false;
                };

                prev_dir = if p < c {
                    Some(Direction::Ascending)
                } else {
                    Some(Direction::Decending)
                };

                if (p - c).abs() > 3 {
                    return false;
                }

                prev_val = Some(c);
            }

            (Some(p), Some(d), Some(c)) => {
                if p == c {
                    return false;
                };

                let curr_dir = if p < c {
                    Direction::Ascending
                } else {
                    Direction::Decending
                };

                if curr_dir != d {
                    return false;
                }

                if (p - c).abs() > 3 {
                    return false;
                }

                prev_val = Some(c);
            }
            _ => {}
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!("2", part1(&input)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!("4", part2(&input)?);
        Ok(())
    }
}
