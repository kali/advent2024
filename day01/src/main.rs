use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let (mut left, mut right) = input
        .trim()
        .lines()
        .map(|s| {
            let (a, b) = s.split_whitespace().collect_tuple().unwrap();
            (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
        })
        .unzip::<_, _, Vec<usize>, Vec<usize>>();
    left.sort();
    right.sort();
    let p1 = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<usize>();
    dbg!(p1);
    let p2 = left
        .iter()
        .map(|l| l * right.iter().filter(|x| *x == l).count())
        .sum::<usize>();
    dbg!(p2);
    Ok(())
}
