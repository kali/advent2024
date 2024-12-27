use std::collections::HashMap;
use std::iter::once;

use itertools::Itertools;
use pathfinding::prelude::dijkstra_all;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let map = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect_vec();
    let [start, end] = [b'S', b'E'].map(|needle| {
        map.iter()
            .enumerate()
            .find_map(|(y, l)| {
                l.iter()
                    .position(|c| *c == needle)
                    .map(|x| (x as isize, y as isize))
            })
            .unwrap()
    });
    let [from_start, from_end] = [start, end].map(|loc| {
        dijkstra_all(&loc, |&(x0, y0)| {
            [(x0 + 1, y0), (x0 - 1, y0), (x0, y0 - 1), (x0, y0 + 1)]
                .into_iter()
                .filter(|&(x, y)| {
                    (0..map[0].len() as isize).contains(&x)
                        && (0..map.len() as isize).contains(&y)
                        && map[y as usize][x as usize] != b'#'
                })
                .map(|p| (p, 1))
        })
        .into_iter()
        .chain(once((loc, (loc, 0))))
        .collect::<HashMap<_, _>>()
    });
    let fair = from_start[&end].1;
    for cheat_len in [2, 20] {
        let mut count = 0;
        for x0 in 0..map[0].len() {
            for y0 in 0..map.len() {
                for x1 in 0..map[0].len() {
                    for y1 in 0..map.len() {
                        let len = x1.abs_diff(x0) + y1.abs_diff(y0);
                        if len > cheat_len {
                            continue;
                        }
                        if let (Some(d0), Some(d1)) = (
                            from_start.get(&(x0 as isize, y0 as isize)),
                            from_end.get(&(x1 as isize, y1 as isize)),
                        ) {
                            count += (d0.1 + d1.1 + len <= fair - 100) as usize;
                        }
                    }
                }
            }
        }
        dbg!(count);
    }
    Ok(())
}
