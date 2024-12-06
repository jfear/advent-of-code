use std::collections::HashSet;

use glam::IVec2;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let parsed = parse(input);
    let n_cols = parsed.iter().map(|p| p.0.x).max().unwrap();
    let n_rows = parsed.iter().map(|p| p.0.y).max().unwrap();

    let walls = parsed
        .iter()
        .filter(|p| p.1 == '#')
        .map(|p| p.0)
        .collect::<HashSet<IVec2>>();

    let mut direction = Direction::Up;
    let mut guard = parsed
        .iter()
        .filter(|p| p.1 == '^')
        .map(|p| p.0)
        .last()
        .unwrap();

    let mut visited = HashSet::from([guard.clone()]);
    loop {
        let peek = guard + direction.step();

        if walls.contains(&peek) {
            direction = direction.turn_right();
            continue;
        }

        guard += direction.step();

        if !(0..=n_cols).contains(&guard.x) || !(0..=n_rows).contains(&guard.y) {
            break;
        }

        visited.insert(guard.clone());
    }

    Ok(visited.len().to_string())
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn step(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::NEG_Y,
            Direction::Right => IVec2::X,
            Direction::Down => IVec2::Y,
            Direction::Left => IVec2::NEG_X,
        }
    }
}

fn parse(input: &str) -> Vec<(IVec2, char)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(r, l)| {
            l.chars()
                .enumerate()
                .map(move |(c, v)| (IVec2::new(c.clone() as i32, r.clone() as i32), v.to_owned()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let mut input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("41", process(&mut input)?);
        Ok(())
    }
}