use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str, blinks: u32) -> miette::Result<String> {
    let mut stones: HashMap<u64, u64> = HashMap::new();
    for stone in input
        .split_ascii_whitespace()
        .map(|d| d.parse::<u64>().unwrap())
    {
        stones.entry(stone).and_modify(|v| *v += 1).or_insert(1);
    }

    for _blink in 0..blinks {
        let mut new_stones = HashMap::new();
        for (stone, cnt) in stones {
            match stone {
                0 => new_stones.entry(1).and_modify(|v| *v += cnt).or_insert(cnt),
                n => match n.ilog10() + 1 {
                    // If has an even number of digits
                    s if s % 2 == 0 => {
                        let left = n / 10u64.pow(s / 2);
                        new_stones
                            .entry(left)
                            .and_modify(|v| *v += cnt)
                            .or_insert(cnt);

                        let right = n % 10u64.pow(s / 2);
                        new_stones
                            .entry(right)
                            .and_modify(|v| *v += cnt)
                            .or_insert(cnt)
                    }
                    _ => new_stones
                        .entry(n * 2024)
                        .and_modify(|v| *v += cnt)
                        .or_insert(cnt),
                },
            };
        }

        stones = new_stones.clone();
    }

    Ok(stones.values().sum::<u64>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input, 25)?);
        Ok(())
    }
}
