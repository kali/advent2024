use std::collections::HashMap;

use itertools::Itertools;

fn assemble(towels: &[&str], design: &str) -> bool {
    design.len() == 0
        || towels.iter().any(|towel| {
            design.starts_with(towel) && assemble(towels, design.trim_start_matches(towel))
        })
}

fn ways<'c>(cache: &mut HashMap<&'c str, usize>, towels: &[&str], design: &'c str) -> usize {
    if design.len() == 0 {
        return 1;
    }
    if let Some(c) = cache.get(design) {
        return *c;
    }
    let computed = towels
        .iter()
        .map(|towel| {
            if design.starts_with(towel) {
                ways(cache, towels, &design[towel.len()..])
            } else {
                0
            }
        })
        .sum();
    cache.insert(design, computed);
    computed
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let towels = input.lines().nth(0).unwrap().split(", ").collect_vec();
    let designs = input.lines().skip(2).collect_vec();
    let p1 = designs.iter().filter(|d| assemble(&towels, d)).count();
    dbg!(p1);
    let mut cache = Default::default();
    let p2 = designs
        .iter()
        .map(|d| ways(&mut cache, &towels, d))
        .sum::<usize>();
    dbg!(p2);
    Ok(())
}

#[test]
fn t2() {
    let towels = "r wr b g bwu rb gb br".split_whitespace().collect_vec();
    assert_eq!(ways(&mut Default::default(), &towels, "brwrr"), 2);
    assert_eq!(ways(&mut Default::default(), &towels, "gbbr"), 4);
    assert_eq!(ways(&mut Default::default(), &towels, "rrbgbr"), 6);
}
