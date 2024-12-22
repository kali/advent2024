use itertools::Itertools;
use num_complex::Complex;

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
                    .map(|x| Complex::new(x as isize, y as isize))
            })
            .unwrap()
    });
    macro_rules! at {
        ($x: expr) => {
            map[$x.im as usize][$x.re as usize]
        };
    }
    let path = pathfinding::directed::astar::astar_bag_collect(
        &(start, Complex::ONE),
        |pos: &(Complex<isize>, Complex<isize>)| {
            let mut moves = vec![
                ((pos.0, pos.1 * Complex::i()), 1000),
                ((pos.0, -pos.1 * Complex::i()), 1000),
            ];
            if at!(pos.0 + pos.1) != b'#' {
                moves.push(((pos.0 + pos.1, pos.1), 1));
            }
            moves
        },
        |x| x.0.re.abs_diff(end.re) + x.0.im.abs_diff(end.im),
        |x| x.0 == end,
    )
    .unwrap();
    dbg!(path.1);
    let p2 = path
        .0
        .iter()
        .flat_map(|p| p.iter().map(|x| x.0))
        .unique()
        .count();
    dbg!(p2);
    Ok(())
}
