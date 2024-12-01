use std::{
    collections::HashMap,
    io::{self, Write},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = include_str!("../input.txt");

    writeln!(io::stdout(), "Part1: {}", part1(&input)?)?;
    writeln!(io::stdout(), "Part2: {}", part2(&input)?)?;
    Ok(())
}

fn part1(input: &str) -> Result<String> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<i32>().unwrap());
        right.push(items.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let diffs = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u32>();

    Ok(diffs.to_string())
}

fn part2(input: &str) -> Result<String> {
    let mut left = vec![];
    let mut right: HashMap<&str, i32> = HashMap::new();

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap());
        *right.entry(items.next().unwrap()).or_insert(0) += 1;
    }

    let diffs = left
        .into_iter()
        .map(|l| {
            let cnt = right.get(l).unwrap_or(&0);
            l.parse::<i32>().unwrap() * cnt
        })
        .sum::<i32>();

    Ok(diffs.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!("11", part1(&input)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", part2(&input)?);
        Ok(())
    }
}
