use std::collections::HashMap;

use glam::IVec2;

#[tracing::instrument]
pub fn process(input: &mut &str) -> miette::Result<String> {
    let (mut map, directions) = parse(input);
    for (i, dir) in directions.into_iter().enumerate() {
        let robot = map
            .clone()
            .into_iter()
            .find_map(|(k, v)| match v {
                '@' => Some(k),
                _ => None,
            })
            .expect("should always have a robot");

        let mut new_loc = robot + dir;
        let mut to_move = vec![robot];
        loop {
            match map.get(&new_loc) {
                Some('.') => {
                    to_move.push(new_loc.clone());
                    break;
                }
                Some('O') => {
                    to_move.push(new_loc.clone());
                    new_loc += dir;
                }
                Some('#') => {
                    to_move.clear();
                    break;
                }
                v => {
                    panic!("should never happen!")
                }
            }
        }

        for loc in to_move.into_iter().rev() {
            match map.remove(&loc) {
                Some('@') => {
                    map.insert(loc, '.');
                    map.insert(loc + dir, '@');
                }
                Some('O') => {
                    map.insert(loc, '.');
                    map.insert(loc + dir, 'O');
                }
                Some('.') => (),
                Some(_) => {
                    panic!("should never happen!");
                }
                None => {
                    panic!("should never happen!");
                }
            }
        }
    }

    let gps = map
        .iter()
        .filter_map(|(k, v)| match v {
            'O' => Some(k.y * 100 + k.x),
            _ => None,
        })
        .sum::<i32>();

    Ok(gps.to_string())
}

fn display_map(map: &HashMap<IVec2, char>) -> miette::Result<()> {
    let n_rows = map.keys().map(|p| p.y).max().unwrap();
    let n_cols = map.keys().map(|p| p.x).max().unwrap();

    let view = (0..=n_rows)
        .map(|y| {
            (0..=n_cols)
                .map(|x| match map.get(&IVec2::new(x, y)) {
                    Some(&c) => c,
                    None => '-',
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", view);

    Ok(())
}

fn parse_map(input: &str) -> HashMap<IVec2, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| (IVec2::new(x as i32, y as i32), c))
        })
        .collect()
}

fn parse_directions(input: &str) -> Vec<IVec2> {
    input
        .lines()
        .flat_map(|l| {
            l.chars().map(|c| match c {
                '^' => IVec2::NEG_Y,
                '>' => IVec2::X,
                'v' => IVec2::Y,
                '<' => IVec2::NEG_X,
                _ => panic!("invalid command!"),
            })
        })
        .collect()
}

fn parse(input: &mut &str) -> (HashMap<IVec2, char>, Vec<IVec2>) {
    let mut blocks = input.split("\n\n");
    let m = blocks.next().expect("should have the map block.");
    let d = blocks.next().expect("should have the directions block.");
    (parse_map(m), parse_directions(d))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process_small() -> miette::Result<()> {
        let mut input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!("2028", process(&mut input)?);
        Ok(())
    }

    #[test]
    fn test_process_large() -> miette::Result<()> {
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
        assert_eq!("10092", process(&mut input)?);
        Ok(())
    }

    #[test]
    fn test_parse_map() -> miette::Result<()> {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";
        let map = parse_map(&input);
        assert_eq!('#', *map.get(&IVec2::new(0, 0)).unwrap());
        assert_eq!('.', *map.get(&IVec2::new(1, 1)).unwrap());
        assert_eq!('@', *map.get(&IVec2::new(2, 2)).unwrap());
        Ok(())
    }

    #[test]
    fn test_parse_directions() -> miette::Result<()> {
        let input = "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let directions = parse_directions(&input);
        assert_eq!(IVec2::NEG_X, *directions.get(0).unwrap());
        assert_eq!(IVec2::Y, *directions.get(1).unwrap());
        assert_eq!(IVec2::X, *directions.get(3).unwrap());
        assert_eq!(IVec2::NEG_Y, *directions.get(4).unwrap());
        Ok(())
    }
}
