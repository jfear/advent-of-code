use miette::miette;
use num_traits::ops::euclid::Euclid;
use std::fmt::Debug;
use winnow::{
    ascii::{dec_int, line_ending},
    combinator::{preceded, separated, separated_pair, terminated},
    prelude::*,
};

use glam::IVec2;

#[tracing::instrument]
pub fn process(input: &mut &str) -> miette::Result<String> {
    let games = parse(input).map_err(|_| miette!("should be able to parse AoC input"))?;
    let tokens = games
        .iter()
        .filter_map(|g| solve(g))
        .map(|(a, b)| a * 3 + b * 1)
        .sum::<u32>();
    Ok(tokens.to_string())
}

fn solve(game: &Game) -> Option<(u32, u32)> {
    let (a_div, a_rem) = Euclid::div_rem_euclid(
        &(game.b.x * game.prize.y - game.b.y * game.prize.x),
        &(game.b.x * game.a.y - game.b.y * game.a.x),
    );

    let (b_div, b_rem) = Euclid::div_rem_euclid(
        &(game.a.x * game.prize.y - game.a.y * game.prize.x),
        &(game.a.x * game.b.y - game.a.y * game.b.x),
    );

    match (a_rem, b_rem) {
        (0, 0) => Some((a_div as u32, b_div as u32)),
        (_, _) => None,
    }
}

fn parse_a_button(input: &mut &str) -> PResult<IVec2> {
    preceded(
        "Button A: X+",
        separated_pair(dec_int::<_, i32, _>, ", Y+", dec_int::<_, i32, _>),
    )
    .map(|(x, y)| IVec2::new(x, y))
    .parse_next(input)
}

fn parse_b_button(input: &mut &str) -> PResult<IVec2> {
    preceded(
        "Button B: X+",
        separated_pair(dec_int::<_, i32, _>, ", Y+", dec_int::<_, i32, _>),
    )
    .map(|(x, y)| IVec2::new(x, y))
    .parse_next(input)
}

fn parse_prize(input: &mut &str) -> PResult<IVec2> {
    preceded(
        "Prize: X=",
        separated_pair(dec_int::<_, i32, _>, ", Y=", dec_int::<_, i32, _>),
    )
    .map(|(x, y)| IVec2::new(x, y))
    .parse_next(input)
}

fn parse_game(input: &mut &str) -> PResult<Game> {
    (
        terminated(parse_a_button, line_ending),
        terminated(parse_b_button, line_ending),
        parse_prize,
    )
        .map(|(a, b, prize)| Game { a, b, prize })
        .parse_next(input)
}

fn parse(input: &mut &str) -> PResult<Vec<Game>> {
    separated(1.., parse_game, (line_ending, line_ending)).parse_next(input)
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    a: IVec2,
    b: IVec2,
    prize: IVec2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let mut input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("480", process(&mut input)?);
        Ok(())
    }

    #[test]
    fn test_parse_game() -> miette::Result<()> {
        let mut input = "Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450";
        let expected = Game {
            a: IVec2::new(17, 86),
            b: IVec2::new(84, 37),
            prize: IVec2::new(7870, 6450),
        };
        let observed = parse_game
            .parse_next(&mut input)
            .map_err(|e| miette!("should parse {}", e))?;
        assert_eq!(expected, observed);
        Ok(())
    }
}
