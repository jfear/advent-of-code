use std::collections::HashSet;

use glam::IVec2;
use miette::Context;

#[tracing::instrument]
pub fn process(input: &mut &str) -> miette::Result<String> {
    let (n_rows, n_cols, guard, objects) = parse(input).context("Could not parse input")?;

    let n_rows = n_rows as i32;
    let n_cols = n_cols as i32;

    let mut guard = guard.clone();
    let mut visited = HashSet::new();
    visited.insert(guard.position.clone());

    loop {
        let next = guard.peek();

        if objects.contains(&next) {
            guard.turn_right();
            continue;
        }

        if next.x == -1 || next.x > n_rows {
            break;
        }

        if next.y == -1 || next.y > n_cols {
            break;
        }

        guard.step();
        visited.insert(guard.position.clone());
    }
    Ok(visited.len().to_string())
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Up
    }
}

#[derive(Default, Debug, Clone)]
struct Guard {
    position: IVec2,
    direction: Direction,
}

impl Guard {
    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn peek(&mut self) -> IVec2 {
        match self.direction {
            Direction::Up => IVec2::new(self.position.x - 1, self.position.y),
            Direction::Right => IVec2::new(self.position.x, self.position.y + 1),
            Direction::Down => IVec2::new(self.position.x + 1, self.position.y),
            Direction::Left => IVec2::new(self.position.x, self.position.y - 1),
        }
    }

    fn step(&mut self) {
        match self.direction {
            Direction::Up => self.position.x -= 1,
            Direction::Right => self.position.y += 1,
            Direction::Down => self.position.x += 1,
            Direction::Left => self.position.y -= 1,
        }
    }
}

fn parse(input: &mut &str) -> miette::Result<(usize, usize, Guard, Vec<IVec2>)> {
    let mut objects: Vec<IVec2> = Vec::new();
    let mut guard = Guard::default();
    let mut n_rows = 0;
    let mut n_cols = 0;

    for (i, line) in input.lines().enumerate() {
        n_rows = i;
        for (j, c) in line.chars().enumerate() {
            n_cols = j;
            match c {
                '#' => objects.push(IVec2::new(i as i32, j as i32)),
                '<' => {
                    guard.position = IVec2::new(i as i32, j as i32);
                    guard.direction = Direction::Left;
                }
                '^' => {
                    guard.position = IVec2::new(i as i32, j as i32);
                    guard.direction = Direction::Up;
                }
                '>' => {
                    guard.position = IVec2::new(i as i32, j as i32);
                    guard.direction = Direction::Right;
                }
                'v' => {
                    guard.position = IVec2::new(i as i32, j as i32);
                    guard.direction = Direction::Down;
                }
                _ => {}
            }
        }
    }

    Ok((n_rows, n_cols, guard, objects))
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
        assert_eq!("6", process(&mut input)?);
        Ok(())
    }
}
