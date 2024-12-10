use std::collections::HashSet;

use itertools::Itertools;
use pathfinding::prelude::{bfs_reach, count_paths};

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let ref grid = input
        .lines()
        .map(|l| l.bytes().map(|c| c - b'0').collect_vec())
        .collect_vec();
    let mut pairs = HashSet::<((isize, isize), (isize, isize))>::new();
    let mut paths = 0;
    let successors = |&(x, y): &(isize, isize)| {
        [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .iter()
            .map(move |(dx, dy)| (x + dx, y + dy))
            .filter(move |&(x1, y1)| {
                x1 >= 0
                    && y1 >= 0
                    && x1 < grid[0].len() as isize
                    && y1 < grid.len() as isize
                    && grid[y1 as usize][x1 as usize] == grid[y as usize][x as usize] + 1
            })
    };
    for y0 in 0..grid.len() {
        for x0 in 0..grid[0].len() {
            if grid[y0][x0] == 0 {
                for reach in bfs_reach((x0 as isize, y0 as isize), successors) {
                    if grid[reach.1 as usize][reach.0 as usize] == 9 {
                        pairs.insert(((x0 as isize, y0 as isize), reach));
                    }
                }
                paths += count_paths((x0 as isize, y0 as isize), successors, |&(x1, y1)| {
                    grid[y1 as usize][x1 as usize] == 9
                });
            }
        }
    }
    dbg!(pairs.len());
    dbg!(paths);
    Ok(())
}
