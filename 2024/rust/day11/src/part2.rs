#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut stones = input
        .split_ascii_whitespace()
        .map(|d| d.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    for _blink in 0..75 {
        stones = stones
            .into_iter()
            .flat_map(|stone| {
                let mut res = Vec::new();
                match stone {
                    0 => res.push(1),
                    _ => match stone.ilog10() + 1 {
                        s if s % 2 == 0 => res.extend([
                            stone / 10i32.pow(s / 2) as u64,
                            stone % 10i32.pow(s / 2) as u64,
                        ]),
                        _ => res.push(stone * 2024),
                    },
                }
                res
            })
            .collect::<Vec<u64>>();
    }

    Ok(stones.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let mut input = "125 17";
        assert_eq!("55312", process(&mut input)?);
        Ok(())
    }
}
