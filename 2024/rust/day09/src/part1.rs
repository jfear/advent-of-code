use itertools::{repeat_n, Itertools};
use std::collections::VecDeque;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut queue = input
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .unwrap_or_else(|| panic!("should always parse digit: {}", c))
        })
        .enumerate()
        .filter_map(|(i, d)| match i % 2 {
            0 => Some(d),
            _ => None,
        })
        .enumerate()
        .flat_map(|(i, d)| repeat_n(i as u32, d as usize).collect_vec())
        .collect::<VecDeque<u32>>();

    let spaces = input
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .unwrap_or_else(|| panic!("should always parse digit: {}", c))
        })
        .enumerate()
        .filter_map(|(i, d)| match i % 2 {
            0 => None,
            _ => Some(d),
        })
        .collect::<Vec<u32>>();

    let mut defrag = Vec::new();
    for n_space in spaces {
        if queue.is_empty() {
            break;
        }

        defrag.extend(pop_next_set(&mut queue));
        for _ in 0..n_space {
            if queue.is_empty() {
                break;
            }
            defrag.push(queue.pop_back());
        }
    }

    let result: u64 = defrag
        .into_iter()
        .enumerate()
        .map(|(i, d)| i as u64 * d.unwrap() as u64)
        .sum();
    Ok(result.to_string())
}

fn pop_next_set(queue: &mut VecDeque<u32>) -> Vec<Option<u32>> {
    let mut next_set = Vec::from([queue.pop_front()]);
    loop {
        if next_set.contains(&queue.get(0).cloned()) {
            next_set.push(queue.pop_front());
        } else {
            break;
        }
    }
    next_set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
