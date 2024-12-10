use miette::miette;
use winnow::{
    combinator::{preceded, repeat},
    stream::{AsChar, Location},
    token::take_till,
    Located, Parser,
};

pub type Span<'a> = Located<&'a str>;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut span = Span::new(input);
    let _result = parse(&mut span).map_err(|e| miette!("parser failed {}", e))?;
    Ok(0.to_string())
}

fn alphanum_pos(input: &mut Span) -> winnow::PResult<()> {
    let loc = input.location();
    dbg!(&loc);

    Ok(())
}

fn parse(input: &mut Span) -> winnow::PResult<()> {
    repeat(
        0..,
        preceded(take_till(0.., |c: char| c.is_alphanum()), alphanum_pos),
    )
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
