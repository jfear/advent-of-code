use std::collections::BTreeMap;

use glam::IVec2;
use miette::miette;
use winnow::{
    ascii::{dec_int, line_ending, space1},
    combinator::{preceded, separated, separated_pair},
    prelude::*,
};

const SIM_SEC: i32 = 100;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Quadrant {
    One,
    Two,
    Three,
    Four,
}

#[tracing::instrument]
pub fn process(input: &mut &str, x_max: i32, y_max: i32) -> miette::Result<String> {
    let robots = parse(input).map_err(|e| miette!("could not parse {}", e))?;
    let score = robots
        .iter()
        .map(|r| simulate(&r, x_max, y_max, SIM_SEC))
        .filter_map(|l| assign_quadrant(l, x_max, y_max))
        .fold(BTreeMap::new(), |mut acc, q| {
            acc.entry(q).and_modify(|v| *v += 1).or_insert(1);
            acc
        })
        .into_iter()
        .map(|m| m.1)
        .product::<u64>();

    Ok(score.to_string())
}

fn assign_quadrant(loc: IVec2, x_max: i32, y_max: i32) -> Option<Quadrant> {
    let x_mid = x_max / 2;
    let y_mid = y_max / 2;
    if (loc.x < x_mid) && (loc.y < y_mid) {
        Some(Quadrant::One)
    } else if (loc.x > x_mid) && (loc.y < y_mid) {
        Some(Quadrant::Two)
    } else if (loc.x < x_mid) && (loc.y > y_mid) {
        Some(Quadrant::Three)
    } else if (loc.x > x_mid) && (loc.y > y_mid) {
        Some(Quadrant::Four)
    } else {
        None
    }
}

fn _correct(v: i32, v_max: i32) -> i32 {
    if v >= v_max {
        v % v_max
    } else if v < 0 {
        let v2 = v_max + (v % v_max);
        _correct(v2, v_max)
    } else {
        v
    }
}

fn simulate(robot: &Robot, x_max: i32, y_max: i32, n: i32) -> IVec2 {
    let new_location = robot.position + robot.velocity * n;
    IVec2::new(
        _correct(new_location.x, x_max),
        _correct(new_location.y, y_max),
    )
}

#[derive(Debug)]
struct Robot {
    position: IVec2,
    velocity: IVec2,
}

fn parse(input: &mut &str) -> PResult<Vec<Robot>> {
    separated(
        1..,
        separated_pair(
            preceded("p=", separated_pair(dec_int, ",", dec_int)).map(|(x, y)| IVec2::new(x, y)),
            space1,
            preceded("v=", separated_pair(dec_int, ",", dec_int)).map(|(x, y)| IVec2::new(x, y)),
        )
        .map(|(p, v)| Robot {
            position: p,
            velocity: v,
        }),
        line_ending,
    )
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_process() -> miette::Result<()> {
        let mut input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!("12", process(&mut input, 11, 7)?);
        Ok(())
    }

    #[rstest]
    #[case(IVec2::new(7 ,6), IVec2::new(-1 ,-3), IVec2::new(6, 0))]
    #[case(IVec2::new(7 ,3), IVec2::new(-1 ,2), IVec2::new(6, 0))]
    #[case(IVec2::new(10 ,3), IVec2::new(-1 ,2), IVec2::new(9, 0))]
    #[case(IVec2::new(9, 3), IVec2::new(2, 3), IVec2::new(0, 2))]
    #[case(IVec2::new(3 ,0), IVec2::new(-2 ,-2), IVec2::new(1, 3))]
    #[case(IVec2::new(3 ,0), IVec2::new(-1 ,-2), IVec2::new(2, 3))]
    #[case(IVec2::new(6 ,3), IVec2::new(-1 ,-3), IVec2::new(5, 4))]
    #[case(IVec2::new(0 ,4), IVec2::new(3 ,-3), IVec2::new(3, 5))]
    #[case(IVec2::new(2 ,0), IVec2::new(2 ,-1), IVec2::new(4, 5))]
    #[case(IVec2::new(2 ,4), IVec2::new(2 ,-3), IVec2::new(4, 5))]
    #[case(IVec2::new(0, 0), IVec2::new(1, 3), IVec2::new(1, 6))]
    #[case(IVec2::new(9 ,5), IVec2::new(-3 ,-3), IVec2::new(6, 6))]

    fn test_simulate(
        #[case] p: IVec2,
        #[case] v: IVec2,
        #[case] expected: IVec2,
    ) -> miette::Result<()> {
        let start = Robot {
            position: p,
            velocity: v,
        };
        let observed = simulate(&start, 11, 7, 100);
        assert_eq!(expected, observed);
        Ok(())
    }

    #[rstest]
    #[case(IVec2::new(6, 0), Some(Quadrant::Two))]
    #[case(IVec2::new(6, 0), Some(Quadrant::Two))]
    #[case(IVec2::new(9, 0), Some(Quadrant::Two))]
    #[case(IVec2::new(0, 2), Some(Quadrant::One))]
    #[case(IVec2::new(1, 3), None)]
    #[case(IVec2::new(2, 3), None)]
    #[case(IVec2::new(5, 4), None)]
    #[case(IVec2::new(3, 5), Some(Quadrant::Three))]
    #[case(IVec2::new(4, 5), Some(Quadrant::Three))]
    #[case(IVec2::new(4, 5), Some(Quadrant::Three))]
    #[case(IVec2::new(1, 6), Some(Quadrant::Three))]
    #[case(IVec2::new(6, 6), Some(Quadrant::Four))]

    fn test_assing_quadrant(
        #[case] location: IVec2,
        #[case] expected: Option<Quadrant>,
    ) -> miette::Result<()> {
        let observed = assign_quadrant(location, 11, 7);
        assert_eq!(expected, observed);
        Ok(())
    }
}
