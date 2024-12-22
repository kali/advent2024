use itertools::Itertools;
use num_complex::Complex;
use pathfinding::prelude::astar;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let bytes: Vec<Complex<isize>> = input
        .lines()
        .map(|line| {
            let (x, y) = line
                .split(",")
                .map(|x| x.parse::<isize>().unwrap())
                .collect_tuple()
                .unwrap();
            Complex::new(x, y)
        })
        .collect_vec();
    let p1 = find_path(&bytes[0..1024]);
    dbg!(p1);
    let se = (1024..bytes.len()).collect_vec();
    let first_block = se.partition_point(|n| find_path(&bytes[..*n]).is_some());
    let p2 = bytes[se[first_block - 1]];
    println!("p2: {},{}", p2.re, p2.im);
    Ok(())
}

fn find_path(bytes: &[Complex<isize>]) -> Option<usize> {
    astar(
        &Complex::new(0, 0),
        |&pos| {
            [Complex::ONE, -Complex::ONE, Complex::i(), -Complex::i()]
                .into_iter()
                .map(move |d| pos + d)
                .filter(|&d| {
                    d.re >= 0 && d.im >= 0 && d.re <= 70 && d.im <= 70 && !bytes.contains(&d)
                })
                .map(|pos| (pos, 1))
        },
        |x| 140 - x.re + x.im,
        |x| x == &Complex::new(70, 70),
    )
    .map(|path| path.1 as usize)
}
