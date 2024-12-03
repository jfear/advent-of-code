use regex::Regex;
use std::io::{self, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = include_str!("../../../input_day03.txt");

    writeln!(io::stdout(), "Part 1: {}", part1(input)?)?;
    writeln!(io::stdout(), "Part 2: {}", part2(input)?)?;
    Ok(())
}

fn part1(input: &str) -> Result<String> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;
    for line in input.lines() {
        for (_, [digit1, digit2]) in re.captures_iter(line).map(|c| c.extract()) {
            let result = digit1.parse::<i32>()? * digit2.parse::<i32>()?;
            total += result;
        }
    }
    Ok(total.to_string())
}

fn part2(input: &str) -> Result<String> {
    let re1 = Regex::new(r"(don\'t|do|mul\(\d+,\d+\))").unwrap();
    let re2 = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;
    let mut enabled = true;
    for line in input.lines() {
        for (_, [command]) in re1.captures_iter(line).map(|c| c.extract()) {
            match command {
                "don't" => enabled = false,
                "do" => enabled = true,
                _ => {
                    if enabled {
                        let m2 = re2.captures(command).unwrap();                        
                        let digit1 = &m2[1].parse::<i32>()?;
                        let digit2 = &m2[2].parse::<i32>()?;
                        total += digit1 * digit2;
                    }
                }
            }
        }
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", part1(input)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", part2(input)?);
        Ok(())
    }
}
