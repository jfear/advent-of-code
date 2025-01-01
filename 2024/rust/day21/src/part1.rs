use std::collections::HashMap;

use glam::IVec2;
use itertools::Itertools;
use miette::miette;
use pathfinding::prelude::yen;

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

// Robot 1 (029A)
// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
//
// Robot 1 <A^A>^^AvvvA or <A^A^>^AvvvA or <A^A^^>AvvvA
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
//    <    A     ^     A    >      ^   ^  A    v   v   v   A
//  <v<A  >>^A  <A    >A    vA    <^A  A  >A  <vA  A   A  >^A
//
// Robot 2 <v<A>>^A<A>AvA<^AA>A<vAAA>^A
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
//
//   <   v   <     A    >  >   ^    A   <      A      >   A    v     A    <      ^   A   A   >   A   <    v    A   A  A  >   ^    A
// <v<A  >A  <A  >>^A  vA  A  <^A  >A  <v<A   >>^A   vA  ^A   <vA   >^A  <v<A   >^A  >A  A  vA  ^A  <v<A  >A  >^A  A  A  vA  <^A  >A
//
// Robot 3 <v<A>A<A>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
//
// Me
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
//

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let res: i32 = input
        .lines()
        .map(|code| (code, find_path(code)))
        .map(|(code, key_presses)| {
            calc_complexity(code, key_presses).expect("AoC codes should be parsable")
        })
        .sum();

    Ok(res.to_string())
}

fn find_path(code: &str) -> String {
    let key_pad_paths = find_key_pad_paths(code);
    let key_pad_min = key_pad_paths.iter().map(|p| p.len()).min().unwrap();

    let dir1_pad_paths = key_pad_paths
        .iter()
        .filter(|&p| p.len() == key_pad_min)
        .flat_map(|p| find_dir_pad_paths(p))
        .collect::<Vec<_>>();
    let dir1_pad_min = dir1_pad_paths.iter().map(|p| p.len()).min().unwrap();

    let dir2_pad_paths = dir1_pad_paths
        .iter()
        .filter(|&p| p.len() == dir1_pad_min)
        .flat_map(|p| find_dir_pad_paths(p))
        .collect::<Vec<_>>();
    let dir2_pad_min = dir2_pad_paths.iter().map(|p| p.len()).min().unwrap();

    dir2_pad_paths
        .iter()
        .filter(|&p| p.len() == dir2_pad_min)
        .last()
        .unwrap()
        .to_owned()
}

/// Find the location of a given character on a keypad.
fn find_position(map: &Map, key: &char) -> Option<IVec2> {
    map.iter()
        .filter_map(|(k, v)| if v == key { Some(k.clone()) } else { None })
        .last()
}

/// Find the shortest path between key presses.
fn find_shortest_paths(map: &Map, start: &IVec2, end: &IVec2) -> Vec<String> {
    let keys = map.keys().collect::<Vec<&IVec2>>();
    let k_paths = yen(
        start,
        |p| {
            DIRECTIONS
                .iter()
                .filter_map(|dir| {
                    let next_pos = p + dir;
                    if keys.contains(&&next_pos) {
                        Some((next_pos, 1))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(IVec2, i32)>>()
        },
        |p| p == end,
        5,
    );
    let shortest_path_weight = k_paths
        .iter()
        .map(|(_, len)| len)
        .min()
        .expect("At least one path");

    k_paths
        .iter()
        .filter(|(_, l)| l == shortest_path_weight)
        .map(|(p, _)| {
            p.iter()
                .tuple_windows()
                .filter_map(|(a, b)| match b - a {
                    s if s == DIRECTIONS[0] => Some('>'),
                    s if s == DIRECTIONS[1] => Some('<'),
                    s if s == DIRECTIONS[2] => Some('v'),
                    s if s == DIRECTIONS[3] => Some('^'),
                    _ => None,
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}

/// Find the path for a code on the keypad.
fn find_key_pad_paths(code: &str) -> Vec<String> {
    let map = parse_key_pad();
    vec!['A']
        .into_iter()
        .chain(code.chars())
        .tuple_windows()
        .map(|(a, b)| {
            let start = find_position(&map, &a).expect("key should be on keypad.");
            let end = find_position(&map, &b).expect("key should be on keypad");
            find_shortest_paths(&map, &start, &end)
        })
        .multi_cartesian_product()
        .map(|p| {
            let mut path = p.join("A");
            path.push_str("A");
            path
        })
        .collect()
}

/// Find the path for a code on the direction pad.
fn find_dir_pad_paths(path: &str) -> Vec<String> {
    let map = parse_dir_pad();
    vec!['A']
        .into_iter()
        .chain(path.chars())
        .tuple_windows()
        .map(|(a, b)| {
            let start = find_position(&map, &a).expect("key should be on keypad.");
            let end = find_position(&map, &b).expect("key should be on keypad");
            find_shortest_paths(&map, &start, &end)
        })
        .multi_cartesian_product()
        .map(|p| {
            let mut path = p.join("A");
            path.push_str("A");
            path
        })
        .collect()
}

/// Calculate the code complexity.
fn calc_complexity(code: &str, key_presses: String) -> miette::Result<i32> {
    let num = code
        .strip_suffix("A")
        .expect("AoC code should end in 'A'")
        .parse::<i32>()
        .map_err(|e| miette!("Could not parse number {}", e))?;

    Ok(num * key_presses.len() as i32)
}

type Map = HashMap<IVec2, char>;

fn parse_key_pad() -> Map {
    let input = "789
456
123
.0A";

    let mut map = HashMap::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '.' {
                continue;
            }
            let coord = IVec2::new(x as i32, y as i32);
            map.insert(coord, c);
        }
    }
    map
}

fn parse_dir_pad() -> Map {
    let input = ".^A
<v>";

    let mut map = HashMap::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '.' {
                continue;
            }
            let coord = IVec2::new(x as i32, y as i32);
            map.insert(coord, c);
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "029A
980A
179A
456A
379A";
        assert_eq!("126384", process(input)?);
        Ok(())
    }

    #[rstest]
    #[case("029A", vec!["<A^A>^^AvvvA", "<A^A^>^AvvvA", "<A^A^^>AvvvA"]
    )]
    fn test_key_pad_path(#[case] code: &str, #[case] key_presses: Vec<&str>) -> miette::Result<()> {
        assert_eq!(
            key_presses
                .into_iter()
                .map(|k| k.to_string())
                .collect::<Vec<String>>(),
            find_key_pad_paths(code)
        );
        Ok(())
    }

    #[rstest]
    #[case(
        "029A",
        "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
        68 * 29
    )]
    #[case("980A", "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A", 60 * 980)]
    #[case(
        "179A",
        "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A", 68 * 179
    )]
    #[case(
        "456A",
        "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A",
        64 * 456
    )]
    #[case(
        "379A",
        "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
        64 * 379
    )]
    fn test_calc(
        #[case] code: &str,
        #[case] key_presses: &str,
        #[case] complexity: i32,
    ) -> miette::Result<()> {
        assert_eq!(complexity, calc_complexity(code, key_presses.to_string())?);
        Ok(())
    }
}
