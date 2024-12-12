use std::collections::HashSet;

use itertools::Itertools;
use pathfinding::prelude::strongly_connected_component;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    dbg!(run(&input));
    Ok(())
}

fn run(input: &str) -> (usize, usize) {
    let grid: Vec<Vec<u8>> = input.lines().map(|s| s.bytes().collect_vec()).collect_vec();
    let mut regions: Vec<HashSet<(isize, isize)>> = vec![];
    let at = |x: isize, y: isize| -> u8 {
        if x >= 0 && y >= 0 && (x as usize) < grid[0].len() && (y as usize) < grid.len() {
            grid[y as usize][x as usize]
        } else {
            b'.'
        }
    };
    for y in 0..grid.len() as isize {
        for x in 0..grid[0].len() as isize {
            if regions.iter().any(|reg| reg.contains(&(x, y))) {
                continue;
            }
            let region: HashSet<(isize, isize)> =
                strongly_connected_component(&(x, y), |&(x, y)| {
                    [(0, 1), (1, 0), (0, -1), (-1, 0)]
                        .into_iter()
                        .map(move |(dx, dy)| (x + dx, y + dy))
                        .filter(move |&(x1, y1)| at(x1, y1) == at(x, y))
                })
                .into_iter()
                .collect();
            regions.push(region);
        }
    }
    let mut p1 = 0;
    let mut p2 = 0;
    for reg in &regions {
        let mut peri = HashSet::new();
        for &(x, y) in reg {
            peri.extend(
                [(0, 1), (1, 0), (0, -1), (-1, 0)]
                    .into_iter()
                    .filter(move |&(dx, dy)| at(x + dx, y + dy) != at(x, y))
                    .map(move |(dx, dy)| (x, y, dx, dy)),
            );
        }
        p1 += peri.len() * reg.len();
        let filtered = peri
            .iter()
            .filter(|&(x, y, dx, dy)| {
                if *dx == 0 {
                    !peri.contains(&(x - 1, *y, *dx, *dy))
                } else {
                    !peri.contains(&(*x, y - 1, *dx, *dy))
                }
            })
            .count();
        p2 += filtered * reg.len();
    }
    (p1, p2)
}

#[test]
fn t() {
    assert_eq!(run("AA").1, 8);
}
