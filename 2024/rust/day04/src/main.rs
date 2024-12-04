use grid::Grid;
use miette::IntoDiagnostic;
use std::io::{self, Write};

fn main() -> miette::Result<()> {
    let input = include_str!("../../../input_day04.txt");
    writeln!(io::stdout(), "Part 1: {}", part1(input)?).into_diagnostic()?;
    writeln!(io::stdout(), "Part 2: {}", part2(input)?).into_diagnostic()?;
    Ok(())
}

// Scan string for 'X'
// Look in all directions
//                     X         M               A               S
//   - forward      (i, j), (i + 1, j),     (i + 2, j),      (i + 3, j)
//   - reverse      (i, j), (i - 1, j),     (i - 2, j),      (i - 3, j)
//   - down         (i, j), (i, j + 1),     (i, j + 2),      (i, j + 3)
//   - up           (i, j), (i, j - 1),     (i, j - 2),      (i, j - 3)
//   - forward up   (i, j), (i + 1, j - 1), (i + 2, j - 2),  (i + 3, j - 3)
//   - forward down (i, j), (i + 1, j + 1), (i + 2, j + 2),  (i + 3, j + 3)
//   - reverse up   (i, j), (i - 1, j - 1), (i - 2, j - 2),  (i - 3, j - 3)
//   - reverse down (i, j), (i - 1, j + 1), (i - 2, j + 2),  (i - 3, j + 3)

fn _get_xmas_count(
    row1: usize,
    row2: usize,
    row3: usize,
    col1: usize,
    col2: usize,
    col3: usize,
    g: &Grid<char>,
) -> usize {
    if g.get(row1, col1) == Some(&'M')
        && g.get(row2, col2) == Some(&'A')
        && g.get(row3, col3) == Some(&'S')
    {
        1
    } else {
        0
    }
}

fn forward(row: usize, col: usize, g: &Grid<char>) -> usize {
    _get_xmas_count(row, row, row, col + 1, col + 2, col + 3, g)
}

fn reverse(row: usize, col: usize, g: &Grid<char>) -> usize {
    _get_xmas_count(row, row, row, col - 1, col - 2, col - 3, g)
}

fn down(row: usize, col: usize, g: &Grid<char>) -> usize {
    _get_xmas_count(row + 1, row + 2, row + 3, col, col, col, g)
}

fn up(row: usize, col: usize, g: &Grid<char>) -> usize {
    _get_xmas_count(row - 1, row - 2, row - 3, col, col, col, g)
}

fn forward_up(row: usize, col: usize, g: &Grid<char>) -> usize {
    _get_xmas_count(row - 1, row - 2, row - 3, col + 1, col + 2, col + 3, g)
}

fn forward_down(row: usize, col: usize, g: &Grid<char>) -> usize {
    _get_xmas_count(row + 1, row + 2, row + 3, col + 1, col + 2, col + 3, g)
}

fn reverse_up(row: usize, col: usize, g: &Grid<char>) -> usize {
    _get_xmas_count(row - 1, row - 2, row - 3, col - 1, col - 2, col - 3, g)
}

fn reverse_down(row: usize, col: usize, g: &Grid<char>) -> usize {
    _get_xmas_count(row + 1, row + 2, row + 3, col - 1, col - 2, col - 3, g)
}

fn part1(input: &str) -> miette::Result<String> {
    let n_rows = input.lines().count();
    let n_cols = input.lines().next().unwrap().len();
    let data = input.lines().flat_map(|l| l.chars()).collect::<Vec<char>>();
    let g = Grid::from_vec(data, n_cols);

    let mut xmas_cnt = 0;

    for row in 0..n_rows {
        for col in 0..n_cols {
            match g.get(row, col) {
                Some('X') => {
                    if col <= n_cols - 3 {
                        xmas_cnt += forward(row, col, &g);
                    }

                    if col >= 3 {
                        xmas_cnt += reverse(row, col, &g);
                    }

                    if row <= n_rows - 3 {
                        xmas_cnt += down(row, col, &g);
                    }

                    if row >= 3 {
                        xmas_cnt += up(row, col, &g);
                    }

                    if col <= n_cols - 3 && row <= n_rows - 3 {
                        xmas_cnt += forward_down(row, col, &g);
                    }

                    if col <= n_cols - 3 && row >= 3 {
                        xmas_cnt += forward_up(row, col, &g);
                    }

                    if col >= 3 && row <= n_rows - 3 {
                        xmas_cnt += reverse_down(row, col, &g);
                    }

                    if col >= 3 && row >= 3 {
                        xmas_cnt += reverse_up(row, col, &g);
                    }
                }
                _ => {}
            }
        }
    }

    Ok(xmas_cnt.to_string())
}

fn forward_diag1(row: usize, col: usize, g: &Grid<char>) -> usize {
    _get_xmas_count(row - 1, row, row + 1, col - 1, col, col + 1, g)
}

fn forward_diag2(row: usize, col: usize, g: &Grid<char>) -> usize {
    _get_xmas_count(row + 1, row, row - 1, col + 1, col, col - 1, g)
}

fn reverse_diag1(row: usize, col: usize, g: &Grid<char>) -> usize {
    _get_xmas_count(row - 1, row, row + 1, col + 1, col, col - 1, g)
}

fn reverse_diag2(row: usize, col: usize, g: &Grid<char>) -> usize {
    _get_xmas_count(row + 1, row, row - 1, col - 1, col, col + 1, g)
}

fn part2(input: &str) -> miette::Result<String> {
    let n_rows = input.lines().count();
    let n_cols = input.lines().next().unwrap().len();
    let data = input.lines().flat_map(|l| l.chars()).collect::<Vec<char>>();
    let g = Grid::from_vec(data, n_cols);

    let mut xmas_cnt = 0;

    for row in 0..n_rows {
        for col in 0..n_cols {
            match g.get(row, col) {
                Some('A') => {
                    if row >= 1 && row <= n_rows - 2 && col >= 1 && col <= n_cols - 2 {
                        let mut mas_cnt = 0;
                        mas_cnt += forward_diag1(row, col, &g);
                        mas_cnt += forward_diag2(row, col, &g);
                        mas_cnt += reverse_diag1(row, col, &g);
                        mas_cnt += reverse_diag2(row, col, &g);
                        if mas_cnt == 2 {
                            xmas_cnt += 1;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Ok(xmas_cnt.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", part1(input)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("9", part2(input)?);
        Ok(())
    }
}
