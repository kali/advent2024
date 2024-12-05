use std::cmp::Ordering;

use itertools::Itertools;
use topological_sort::TopologicalSort;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules: Vec<(&str, &str)> = rules
        .lines()
        .map(|rule| rule.split_once("|").unwrap())
        .collect_vec();
    let mut p1 = 0;
    'update: for update in updates.lines() {
        let values = update.split(",").collect_vec();
        for (left, right) in &rules {
            let left = values.iter().position(|&x| x == *left);
            let right = values.iter().position(|&x| x == *right);
            if let (Some(left), Some(right)) = (left, right) {
                if left > right {
                    continue 'update;
                }
            }
        }
        p1 += values[values.len() / 2].parse::<usize>().unwrap();
    }
    dbg!(p1);

    let mut p2 = 0;
    for update in updates.lines() {
        let values = update.split(",").collect_vec();
        let mut sorted = TopologicalSort::<&str>::new();
        for (l, r) in &rules {
            if values.contains(&l) && values.contains(&r) {
                sorted.add_dependency(*l, *r);
            }
        }
        let sorted = sorted.into_iter().collect_vec();
        if sorted != values {
            p2 += sorted[values.len() / 2].parse::<usize>().unwrap();
        }
    }
    dbg!(p2);
    Ok(())
}
