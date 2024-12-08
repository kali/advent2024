use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut antennas = HashMap::<char, Vec<(isize, isize)>>::default();
    for (y, s) in input.lines().enumerate() {
        for (x, c) in s.chars().enumerate() {
            if c != '.' {
                antennas
                    .entry(c)
                    .or_default()
                    .push((x as isize, y as isize));
            }
        }
    }
    dbg!(run(height, width, &antennas, false));
    dbg!(run(height, width, &antennas, true));
    Ok(())
}

fn run(
    height: usize,
    width: usize,
    antennas: &HashMap<char, Vec<(isize, isize)>>,
    p2: bool,
) -> usize {
    let range = if p2 { 0..isize::MAX } else { 1..2 };
    let mut antinodes = HashSet::new();
    for (_, antennas) in antennas {
        for perms in antennas.iter().permutations(2) {
            let &[(x1, y1), (x2, y2)] = &*perms else {
                panic!()
            };
            for n in range.clone() {
                let xa = x1 - n as isize * (x2 - x1);
                let ya = y1 - n as isize * (y2 - y1);
                if xa >= 0 && (xa as usize) < width && ya >= 0 && (ya as usize) < height {
                    antinodes.insert((xa, ya));
                } else {
                    break;
                }
            }
        }
    }
    antinodes.len()
}
