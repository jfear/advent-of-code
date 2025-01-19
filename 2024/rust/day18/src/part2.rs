use glam::IVec2;
use pathfinding::prelude::dijkstra;
use winnow::{
    ascii::{dec_int, line_ending},
    combinator::{separated, seq},
    Parser,
};

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

#[tracing::instrument]
pub fn process(input: &mut &str, width: usize) -> miette::Result<String> {
    let bytes = parse(input);
    let mut byte_idx = 1;
    loop {
        match run(width, &bytes[..byte_idx]) {
            Some(_) => {
                byte_idx += 1;
            }
            None => break,
        }
    }
    let fatal_byte = bytes[byte_idx - 1];

    Ok(format!("{},{}", fatal_byte.x, fatal_byte.y))
}

fn run(width: usize, bytes: &[IVec2]) -> Option<(Vec<IVec2>, i32)> {
    let start = IVec2::new(0, 0);
    let end = IVec2::new(width as i32, width as i32);
    dijkstra(
        &start,
        |&pos| {
            let mut new_positions = vec![];
            for dir in DIRECTIONS {
                let next_pos = pos + dir;
                if !bytes.contains(&next_pos)
                    && (next_pos.x >= 0)
                    && (next_pos.y >= 0)
                    && (next_pos.x <= width as i32)
                    && (next_pos.y <= width as i32)
                {
                    new_positions.push(next_pos);
                }
            }
            new_positions.into_iter().map(|p| (p, 1))
        },
        |&pos| pos == end,
    )
}

fn parse(input: &mut &str) -> Vec<IVec2> {
    separated(
        1..,
        seq!(IVec2 {
            x: dec_int::<_, i32, winnow::error::ErrorKind>,
            _: ",",
            y: dec_int::<_, i32, winnow::error::ErrorKind>,
        }),
        line_ending,
    )
    .parse_next(input)
    .expect("AoC should provide valid input.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let mut input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!("6,1", process(&mut input, 6)?);
        Ok(())
    }
}
