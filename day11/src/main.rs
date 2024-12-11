use std::collections::HashMap;

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let mut stones = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .counts_by(|s| s);
    for i in 1..=75 {
        let mut new = HashMap::new();
        for (stone, number) in stones.iter() {
            if *stone == 0 {
                *new.entry(1).or_default() += number;
            } else if stone.to_string().len() % 2 == 0 {
                let s = stone.to_string();
                let (l, r) = s.split_at(s.len() / 2);
                for s in [l, r] {
                    *new.entry(s.parse().unwrap()).or_default() += number;
                }
            } else {
                *new.entry(stone * 2024).or_default() += number;
            }
        }
        stones = new;
        println!("{i} {}", stones.values().sum::<usize>());
    }
    Ok(())
}
