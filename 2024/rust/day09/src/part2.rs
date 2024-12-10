#[derive(Debug, Clone, Copy)]
struct Sector {
    name: u32,
    size: u32,
    stype: SectorType,
}

#[derive(Debug, Clone, Copy)]
enum SectorType {
    File,
    Free,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let disk = input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let c = c.to_digit(10).unwrap();
            match i % 2 {
                0 => Sector {
                    name: i as u32 / 2,
                    size: c,
                    stype: SectorType::File,
                },
                _ => Sector {
                    name: i as u32 / 2,
                    size: c,
                    stype: SectorType::Free,
                },
            }
        })
        .collect::<Vec<Sector>>();

    let mut defrag = disk.clone();
    for sector in disk.iter().rev() {
        match sector.stype {
            SectorType::Free => {}
            SectorType::File => {
                defrag = defrag
                    .iter()
                    .scan(false, |moved, dsector| {
                        let mut res = Vec::new();
                        match dsector.stype {
                            SectorType::Free if (sector.name <= dsector.name) => res.push(*dsector),
                            SectorType::Free if !(*moved) && (sector.size == dsector.size) => {
                                *moved = true;
                                res.push(*sector);
                            }
                            SectorType::Free if !(*moved) && (sector.size < dsector.size) => {
                                *moved = true;
                                res.push(*sector);
                                res.push(Sector {
                                    name: dsector.name,
                                    size: dsector.size - sector.size,
                                    stype: SectorType::Free,
                                });
                            }
                            SectorType::File if *moved && sector.name == dsector.name => {
                                res.push(Sector {
                                    name: dsector.name,
                                    size: dsector.size,
                                    stype: SectorType::Free,
                                })
                            }
                            _ => res.push(*dsector),
                        }
                        Some(res)
                    })
                    .flatten()
                    .collect::<Vec<_>>();
            }
        }
    }

    let mut checksum = (0, 0);
    defrag.iter().fold(&mut checksum, |acc, s| {
        match s.stype {
            SectorType::Free => acc.0 += s.size,
            SectorType::File => {
                for _ in 0..s.size {
                    acc.1 += acc.0 * s.name;
                    acc.0 += 1;
                }
            }
        }
        acc
    });
    Ok(checksum.1.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input)?);
        Ok(())
    }
}
