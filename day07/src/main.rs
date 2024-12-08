use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    dbg!(run(&input, false).unwrap());
    dbg!(run(&input, true).unwrap());
    Ok(())
}

fn run(input: &str, part2: bool) -> anyhow::Result<usize> {
    let mut it = 0;
    for line in input.lines() {
        let (result, numbers) = line.split_once(": ").unwrap();
        let result = result.parse::<usize>().unwrap();
        let numbers = numbers
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect_vec();
        let ops = 2 + part2 as usize;
        for mut combi in 0..ops.pow(numbers.len() as u32 - 1) {
            let mut n = numbers[0];
            for i in 1..numbers.len() {
                let op = combi % ops;
                combi = combi / ops;
                let n2 = numbers[i];
                match op {
                    0 => n += n2,
                    1 => n *= n2,
                    _ => n = format!("{n}{n2}").parse().unwrap(),
                }
            }
            if n == result {
                it += result;
                break;
            }
        }
    }
    Ok(it)
}

#[test]
fn t() {
    assert_eq!(
        run(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
",
            false
        )
        .unwrap(),
        3749
    );
}
