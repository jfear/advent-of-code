use miette::miette;
use std::{collections::HashMap, iter::successors};

use glam::IVec2;

#[tracing::instrument]
pub fn process(input: &mut &str) -> miette::Result<String> {
    let (mut map, directions) = parse(input);

    for dir in directions {
        let step = match dir {
            Direction::NORTH => IVec2::NEG_Y,
            Direction::EAST => IVec2::X,
            Direction::SOUTH => IVec2::Y,
            Direction::WEST => IVec2::NEG_X,
        };

        let to_move = match dir {
            Direction::NORTH | Direction::SOUTH => checked_move_y(&map, step),
            Direction::EAST | Direction::WEST => checked_move_x(&map, step),
        };
        // dbg!(&dir, &to_move);

        match to_move {
            Ok(to_move) => {
                for (key, value) in to_move.iter().rev() {
                    map.entry(*key).and_modify(|v| *v = '.');
                    map.entry(key + step).and_modify(|v| *v = *value);
                }
            }
            Err(_) => {}
        }
    }

    let gps = map
        .iter()
        .filter_map(|(k, v)| match v {
            &'[' => Some(100 * k.y + k.x),
            _ => None,
        })
        .sum::<i32>();

    Ok(gps.to_string())
}

fn checked_move_x(map: &HashMap<IVec2, char>, step: IVec2) -> miette::Result<Vec<(IVec2, char)>> {
    let robot = map
        .iter()
        .find_map(|(k, v)| match v {
            &'@' => Some((k.clone(), v.clone())),
            _ => None,
        })
        .expect("should always be a robot.");

    let to_move = successors(Some(robot), |(curr_pos, curr_value)| {
        if curr_value == &'#' {
            return None;
        }

        let (next_pos, next_value) = map
            .get_key_value(&(*curr_pos + step))
            .expect("should always be in the map");

        match next_value {
            '.' => None,
            _ => Some((next_pos.clone(), next_value.clone())),
        }
    })
    .collect::<Vec<(IVec2, char)>>();

    if to_move.iter().any(|(_, v)| v == &'#') {
        Err(miette!("Oops, you hit a wall"))
    } else {
        Ok(to_move)
    }
}

fn checked_move_y(map: &HashMap<IVec2, char>, step: IVec2) -> miette::Result<Vec<(IVec2, char)>> {
    let robot = map
        .iter()
        .find_map(|(k, v)| match v {
            &'@' => Some((k.clone(), v.clone())),
            _ => None,
        })
        .expect("should always be a robot.");

    let to_move: Vec<(IVec2, char)> = successors(Some(vec![robot]), |curr_values| {
        if curr_values.is_empty() || curr_values.iter().any(|(_, v)| v == &'#') {
            return None;
        }

        Some(
            curr_values
                .iter()
                .filter_map(|&(curr_pos, _curr_value)| {
                    let (next_pos, next_value) = map
                        .get_key_value(&(curr_pos + step))
                        .expect("should be in map");

                    match next_value {
                        '.' => None,
                        '[' => Some(vec![(next_pos.clone(), '['), (next_pos + IVec2::X, ']')]),
                        ']' => Some(vec![
                            (next_pos + IVec2::NEG_X, '['),
                            (next_pos.clone(), ']'),
                        ]),
                        _ => Some(vec![(next_pos.clone(), next_value.clone())]),
                    }
                })
                .flatten()
                .collect::<Vec<_>>(),
        )
    })
    .flatten()
    .collect();

    if to_move.iter().any(|(_, v)| v == &'#') {
        Err(miette!("Oops, you hit a wall"))
    } else {
        Ok(to_move)
    }
}

fn parse_map(input: &str) -> HashMap<IVec2, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            let y = y as i32;

            l.chars().enumerate().flat_map(move |(x, c)| {
                let x = x as i32 * 2;

                match c {
                    '#' => {
                        vec![(IVec2::new(x, y), '#'), (IVec2::new(x + 1, y), '#')]
                    }
                    '.' => {
                        vec![(IVec2::new(x, y), '.'), (IVec2::new(x + 1, y), '.')]
                    }
                    'O' => {
                        vec![(IVec2::new(x, y), '['), (IVec2::new(x + 1, y), ']')]
                    }
                    '@' => {
                        vec![(IVec2::new(x, y), '@'), (IVec2::new(x + 1, y), '.')]
                    }
                    _ => vec![],
                }
            })
        })
        .collect::<HashMap<IVec2, char>>()
}

#[derive(Debug)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input
        .lines()
        .flat_map(|l| {
            l.chars().filter_map(|c| match c {
                '^' => Some(Direction::NORTH),
                '>' => Some(Direction::EAST),
                'v' => Some(Direction::SOUTH),
                '<' => Some(Direction::WEST),
                _ => None,
            })
        })
        .collect()
}

fn parse(input: &mut &str) -> (HashMap<IVec2, char>, Vec<Direction>) {
    let mut blocks = input.split("\n\n");
    let m = blocks.next().expect("should have the map block.");
    let d = blocks.next().expect("should have the directions block.");
    (parse_map(m), parse_directions(d))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let mut input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!("9021", process(&mut input)?);
        Ok(())
    }
}
