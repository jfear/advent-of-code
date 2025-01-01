use miette::miette;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let parsed = parse(input)?;
    let res = parsed.iter().map(|&seed| simulate_2k(seed)).sum::<i64>();

    Ok(res.to_string())
}

fn parse(input: &str) -> miette::Result<Vec<i64>> {
    input
        .lines()
        .map(|l| {
            l.parse::<i64>()
                .map_err(|e| miette!("could not parse number {}", e))
        })
        .collect()
}

fn simulate_2k(seed: i64) -> i64 {
    let mut curr = seed;
    for _ in 0..2000 {
        curr = next_number(curr);
    }
    curr
}

fn next_number(curr: i64) -> i64 {
    let mut curr = curr;

    curr ^= curr * 64;
    curr %= 16777216;

    curr ^= curr / 32;
    curr %= 16777216;

    curr ^= curr * 2048;
    curr %= 16777216;

    curr
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1
10
100
2024";
        assert_eq!("37327623", process(input)?);
        Ok(())
    }

    #[rstest]
    #[case(1, 8685429)]
    #[case(10, 4700978)]
    #[case(100, 15273692)]
    #[case(2024, 8667524)]
    fn test_final_number(#[case] seed: i64, #[case] end: i64) -> miette::Result<()> {
        assert_eq!(end, simulate_2k(seed));
        Ok(())
    }

    #[rstest]
    #[case(123, 15887950)]
    #[case(15887950, 16495136)]
    #[case(16495136, 527345)]
    #[case(527345, 704524)]
    #[case(704524, 1553684)]
    #[case(1553684, 12683156)]
    #[case(12683156, 11100544)]
    #[case(11100544, 12249484)]
    #[case(12249484, 7753432)]
    #[case(7753432, 5908254)]
    fn test_next_number(#[case] seed: i64, #[case] res: i64) -> miette::Result<()> {
        assert_eq!(res, next_number(seed));
        Ok(())
    }
}
