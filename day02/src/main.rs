use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?
        .trim()
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    fn safe(vs: &[usize]) -> bool {
        vs.iter()
            .tuple_windows()
            .all(|(a, b)| a.abs_diff(*b) >= 1 && a.abs_diff(*b) <= 3 && a < b)
            || vs
                .iter()
                .tuple_windows()
                .all(|(a, b)| a.abs_diff(*b) >= 1 && a.abs_diff(*b) <= 3 && a > b)
    }
    let p1 = input.iter().filter(|vs| safe(&vs)).count();
    dbg!(p1);
    let p2 = input
        .iter()
        .filter(|vs| {
            vs.iter()
                .combinations(vs.len() - 1)
                .any(|vs| safe(&*vs.iter().cloned().cloned().collect_vec()))
        })
        .count();
    dbg!(p2);
    Ok(())
}
