use miette::miette;
use winnow::{
    ascii::{digit1, space1},
    combinator::separated,
    Parser,
};

#[tracing::instrument]
pub fn process(input: &mut &str) -> miette::Result<String> {
    let mut stones = parse(input)
        .map_err(|e| miette!("failed to parse {}", e))?
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    for _blink in 0..25 {
        stones = stones
            .into_iter()
            .flat_map(|stone| match stone.as_str() {
                "0" => Vec::from(["1".to_string()]),
                s if s.len() % 2 == 0 => {
                    let half = s.len() / 2;
                    let s1 = s[..half].to_string();
                    let s2 = match s[half..].trim_start_matches("0") {
                        "" => "0",
                        s => s,
                    }
                    .to_string();
                    Vec::from([s1, s2])
                }
                _ => {
                    let d = stone.parse::<u64>().unwrap();
                    let new_d = d * 2024;
                    Vec::from([String::from(format!("{new_d}"))])
                }
            })
            .collect::<Vec<String>>();
    }
    Ok(stones.len().to_string())
}

fn parse<'a>(input: &mut &'a str) -> winnow::PResult<Vec<&'a str>> {
    separated(1.., digit1, space1).parse_next(input)
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
