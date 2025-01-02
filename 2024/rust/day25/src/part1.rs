use itertools::Itertools;
use miette::miette;
use winnow::{
    ascii::{line_ending, multispace1},
    combinator::{alt, repeat, separated},
    Parser,
};

#[tracing::instrument]
pub fn process(input: &mut &str) -> miette::Result<String> {
    let parsed = parse(input).map_err(|e| miette!("should be able to parse AoC {}", e))?;
    let locks = parsed
        .iter()
        .filter(|&v| v[0] == vec![1, 1, 1, 1, 1])
        .map(|lock| {
            lock.iter().fold(vec![0, 0, 0, 0, 0], |acc: Vec<u8>, row| {
                acc.iter().zip(row).map(|(a, b)| a + b).collect()
            })
        })
        .collect::<Vec<Vec<u8>>>();

    let keys = parsed
        .iter()
        .filter(|&v| v[0] == vec![0, 0, 0, 0, 0])
        .map(|lock| {
            lock.iter().fold(vec![0, 0, 0, 0, 0], |acc: Vec<u8>, row| {
                acc.iter().zip(row).map(|(a, b)| a + b).collect()
            })
        })
        .collect::<Vec<Vec<u8>>>();

    let res = locks
        .iter()
        .cartesian_product(keys.iter())
        .filter_map(|(l, k)| {
            let cnt = l
                .iter()
                .zip(k.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<u8>>();
            match cnt.iter().all(|&v| v <= 7) {
                true => Some((l, k)),
                false => None,
            }
        })
        .count();

    Ok(res.to_string())
}

fn parse(input: &mut &str) -> winnow::PResult<Vec<Vec<Vec<u8>>>> {
    separated(1.., parse_rows, multispace1).parse_next(input)
}

fn parse_row(input: &mut &str) -> winnow::PResult<Vec<u8>> {
    repeat::<_, _, Vec<&str>, _, _>(1.., alt((".", "#")))
        .map(|v| {
            v.iter()
                .filter_map(|&c| match c {
                    "#" => Some(1),
                    "." => Some(0),
                    _ => None,
                })
                .collect()
        })
        .parse_next(input)
}

fn parse_rows(input: &mut &str) -> winnow::PResult<Vec<Vec<u8>>> {
    separated(1.., parse_row, line_ending).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let mut input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        assert_eq!("3", process(&mut input)?);
        Ok(())
    }

    #[test]
    fn test_lock() -> miette::Result<()> {
        let mut input = "#####
.####
.####
.####
.#.#.
.#...
.....";
        let expected: Vec<Vec<u8>> = vec![
            vec![1, 1, 1, 1, 1],
            vec![0, 1, 1, 1, 1],
            vec![0, 1, 1, 1, 1],
            vec![0, 1, 1, 1, 1],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];

        assert_eq!(expected, parse_rows(&mut input).unwrap());
        Ok(())
    }

    #[test]
    fn test_parse_row() -> miette::Result<()> {
        let mut input = ".####";
        let expected = vec![0, 1, 1, 1, 1];
        assert_eq!(expected, parse_row(&mut input).unwrap());
        Ok(())
    }
}
