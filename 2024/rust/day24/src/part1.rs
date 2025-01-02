use itertools::Itertools;
use miette::miette;
use std::{
    collections::HashMap,
    ops::{BitAnd, BitOr, BitXor},
};

use winnow::{
    ascii::{alphanumeric1, dec_uint, line_ending, multispace1, space1},
    combinator::{repeat, separated, separated_pair, seq},
    token::take_while,
    Parser,
};

pub fn process(input: &mut &str) -> miette::Result<String> {
    let (mut initial, wires) =
        parse(input).map_err(|e| miette!("could not parse AoC input {}", e))?;

    let zwires = wires
        .keys()
        .cloned()
        .filter(|k| k.starts_with("z"))
        .sorted()
        .collect::<Vec<_>>();

    loop {
        if zwires.iter().all(|z| initial.contains_key(z)) {
            break;
        }

        for (k, v) in wires.iter() {
            if initial.contains_key(k) {
                continue;
            }

            if initial.contains_key(&v.lhs) && initial.contains_key(&v.rhs) {
                let lhs = initial.get(&v.lhs).unwrap();
                let rhs = initial.get(&v.rhs).unwrap();
                let value = match v.op {
                    Operation::AND => lhs.bitand(rhs),
                    Operation::OR => lhs.bitor(rhs),
                    Operation::XOR => lhs.bitxor(rhs),
                };
                initial.insert(k.clone(), value);
            }
        }
    }

    let bytestring = initial
        .iter()
        .filter(|&(k, _)| k.starts_with("z"))
        .sorted()
        .rev()
        .map(|(_, v)| format!("{}", v))
        .join("");

    let res = u64::from_str_radix(&bytestring, 2).unwrap();

    Ok(res.to_string())
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    AND,
    OR,
    XOR,
}

#[derive(Debug, PartialEq, Eq)]
struct Wire {
    lhs: String,
    op: Operation,
    rhs: String,
}

impl Wire {
    fn new(lhs: String, op: Operation, rhs: String) -> Self {
        Wire { lhs, op, rhs }
    }
}

fn parse(input: &mut &str) -> winnow::PResult<(HashMap<String, u8>, HashMap<String, Wire>)> {
    seq!(
        parse_initial_values,
        _: multispace1,
        parse_wires
    )
    .parse_next(input)
}

fn parse_initial_values(input: &mut &str) -> winnow::PResult<HashMap<String, u8>> {
    separated(
        1..,
        separated_pair(alphanumeric1.parse_to(), ": ", dec_uint::<_, u8, _>),
        line_ending,
    )
    .parse_next(input)
}

fn parse_wires(input: &mut &str) -> winnow::PResult<HashMap<String, Wire>> {
    // ntg XOR fgs -> mjb
    separated(
        1..,
        separated_pair(separated(1.., alphanumeric1, space1), " -> ", alphanumeric1).map(
            |(a, b): (Vec<&str>, &str)| {
                let op = if a[1] == "AND" {
                    Operation::AND
                } else if a[1] == "OR" {
                    Operation::OR
                } else {
                    Operation::XOR
                };
                (
                    b.to_string(),
                    Wire::new(a[0].to_string(), op, a[2].to_string()),
                )
            },
        ),
        line_ending,
    )
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let mut input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
        assert_eq!("2024", process(&mut input)?);
        Ok(())
    }

    #[test]
    fn test_init() -> miette::Result<()> {
        let mut input = "x00: 1\nx01: 0";
        let expected = HashMap::from_iter(vec![("x00".to_string(), 1), ("x01".to_string(), 0)]);
        assert_eq!(expected, parse_initial_values(&mut input).unwrap());
        Ok(())
    }

    #[test]
    fn test_parse_wire() -> miette::Result<()> {
        let mut input = "ntg XOR fgs -> mjb\ny02 OR x01 -> tnw";
        let expected = HashMap::from_iter(vec![
            (
                "mjb".to_string(),
                Wire::new("ntg".to_string(), Operation::XOR, "fgs".to_string()),
            ),
            (
                "tnw".to_string(),
                Wire::new("y02".to_string(), Operation::OR, "x01".to_string()),
            ),
        ]);
        assert_eq!(expected, parse_wires(&mut input).unwrap());
        Ok(())
    }
    #[test]
    fn test_parse() -> miette::Result<()> {
        let mut input = "x00: 1
x01: 0

ntg XOR fgs -> mjb
y02 OR x01 -> tnw";
        let expected = (
            HashMap::from_iter(vec![("x00".to_string(), 1), ("x01".to_string(), 0)]),
            HashMap::from_iter(vec![
                (
                    "mjb".to_string(),
                    Wire::new("ntg".to_string(), Operation::XOR, "fgs".to_string()),
                ),
                (
                    "tnw".to_string(),
                    Wire::new("y02".to_string(), Operation::OR, "x01".to_string()),
                ),
            ]),
        );
        assert_eq!(expected, parse(&mut input).unwrap());
        Ok(())
    }
}
